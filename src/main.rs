#![feature(const_mut_refs)]
#![feature(const_eval_limit)]
#![const_eval_limit = "10000000"]
#![feature(test)]

use board::{Board, Color, Move};
use figment::{
  providers::{Format, Serialized, Toml},
  Figment,
};
use hashbrown::{HashMap, HashSet};
// use std::collections::HashMap;
use mimalloc::MiMalloc;
use pgn_reader::{BufferedReader, RawHeader, San, SanPlus, Skip, Visitor};
use serde::{Deserialize, Serialize};
use shakmaty::{uci, Position};
use std::{
  fs::File,
  io::{BufRead, BufReader, BufWriter, Read, Write},
  path::Path,
  println,
  time::Instant,
};
use tokio::{
  io::{AsyncReadExt, AsyncWriteExt},
  net::TcpStream,
};

mod benches;
mod bitboard;
mod board;
mod tests;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

struct Mover {
  writer: BufWriter<File>,
  board: Board,
  will_skip: bool,
  pos: shakmaty::Chess,
  selected: i32,
  total: i32,
  had: HashSet<u64>,
}

impl Mover {
  fn new() -> Self {
    let file = File::create("db.txt").unwrap();

    Self {
      writer: BufWriter::new(file),
      board: Board::default(),
      will_skip: false,
      pos: shakmaty::Chess::default(),
      selected: 0,
      total: 0,
      had: HashSet::new(),
    }
  }
}

const MIN_ELO: i32 = 2600;

impl Visitor for Mover {
  type Result = ();

  fn begin_game(&mut self) {
    self.will_skip = false;
    self.board = Board::default();
    self.pos = shakmaty::Chess::default();
    self.total += 1;
  }

  fn end_game(&mut self) -> Self::Result {}

  fn header(&mut self, key: &[u8], value: RawHeader<'_>) {
    if key == b"FEN"
      || ((key == b"WhiteElo" || key == b"BlackElo") && value.decode_utf8().unwrap().parse::<i32>().unwrap() < MIN_ELO)
    {
      self.will_skip = true;
    }
  }

  fn end_headers(&mut self) -> Skip {
    if !self.will_skip {
      self.selected += 1;

      if self.selected % 1000 == 0 {
        println!("{}/{} | {}", self.selected, self.total, self.had.len());
      }
    }
    Skip(self.will_skip)
  }

  fn san(&mut self, san_plus: SanPlus) {
    let fen = shakmaty::fen::Fen::from_setup(self.pos.clone().into_setup(shakmaty::EnPassantMode::Always)).to_string();
    let hash = Board::from_fen(&fen).meta.hash;
    let m = san_plus.san.to_move(&self.pos).unwrap();

    if !self.had.contains(&hash) {
      let uci_move = m.to_uci(shakmaty::CastlingMode::Standard).to_string();
      let line = hash.to_string() + "|" + &uci_move + "\n";
      self.writer.write_all(line.as_bytes()).unwrap();

      self.had.insert(hash);
    }

    self.pos.play_unchecked(&m);
  }
}

fn gen_db_file() {
  let file = File::open("db/games.pgn").unwrap();
  let mut reader = BufferedReader::new(file);

  let mut mover = Mover::new();
  reader.read_all(&mut mover).unwrap();
}

// fn main() {
//   gen_db_file();
// }

fn try_find_opening(fen: &str) -> Option<String> {
  println!("- Trying to find a saved position...");
  let hash = Board::from_fen(fen).meta.hash.to_string();

  let file = File::open("db.txt").expect("db.txt file missing");
  let reader = BufReader::new(file);

  for line in reader.lines() {
    let line = line.unwrap();
    let (h, m) = line.split_once('|').unwrap();

    if hash == h {
      println!("- Found a saved position!");
      return Some(m.to_string());
    }
  }

  println!("- Could not find a saved position");
  None
}

// #[derive(Serialize, Deserialize, Debug)]
// struct Config {
//   white_side: bool,
// }

// impl Default for Config {
//   fn default() -> Config {
//     Config { white_side: true }
//   }
// }

const WHITE_SIDE: bool = true;

#[tokio::main]
async fn main() {
  // let config: Config = Figment::from(Serialized::defaults(Config::default()))
  //   .merge(Toml::file("config.toml"))
  //   .extract()
  //   .unwrap();

  let port = if WHITE_SIDE { 6969 } else { 6970 };
  // let port = if config.white_side { 6969 } else { 6970 };

  if let Ok(mut stream) = TcpStream::connect(format!("127.0.0.1:{port}")).await {
    println!("- Connected to the interface");

    let mut buf = [0_u8; 1024];
    let mut previous_hashes: Vec<u64> = vec![];
    let mut tt: TranspositionTable = HashMap::new();

    println!("- Waiting for fen...");
    while let Ok(bytes_read) = stream.read(&mut buf).await {
      let start_time = Instant::now();

      if bytes_read <= 1 {
        break;
      }

      let packet = decode_packet(&buf[..bytes_read]);

      let mut board = Board::from_fen_saved(&packet.fen, previous_hashes.clone());
      let our_time = if WHITE_SIDE {
        // let our_time = if config.white_side {
        packet.white_time_left
      } else {
        packet.black_time_left
      };
      let time_given = if our_time < 10 * 1000 {
        1000
      } else if our_time < 30 * 1000 {
        2 * 1000
      } else if our_time < 60 * 1000 {
        4 * 1000
      } else if our_time < 2 * 60 * 1000 {
        6 * 1000
      } else if our_time < 3 * 60 * 1000 {
        8 * 1000
      } else if our_time < 4 * 60 * 1000 {
        10 * 1000
      } else {
        12 * 1000
      };

      previous_hashes.push(board.meta.hash);

      let chess_move_fen = if let Some(saved_move) = try_find_opening(&packet.fen) {
        saved_move
      } else {
        let chess_move = find_best_move(&mut board, time_given, &mut tt);

        board.make_move(&chess_move);
        previous_hashes.push(board.meta.hash);

        chess_move.to_fen()
      };

      println!("- Sending move: '{chess_move_fen}'...");
      stream.write_all(chess_move_fen.as_bytes()).await.expect("Couldn't send move");
      println!("- Sent move successfully");

      let time_taken = start_time.elapsed();
      println!("- Time taken: {:?}", time_taken);

      tt.clear();
      buf.fill(0);

      println!("- Receiving fen...");
    }

    println!("- Disconnecting");
    stream.shutdown().await.expect("Couldn't shutdown stream");
  } else {
    println!("- Couldn't connect to the interface");
  }
}

fn perft_div(fen: &str, depth: usize) {
  let current_time = Instant::now();

  let mut board = Board::from_fen(fen);

  fn perft(cur_depth: usize, board: &mut Board, depth: usize) -> u64 {
    let mut nodes = 0;

    if cur_depth == 0 {
      return 1;
    }

    let side_to_move = board.side_to_move.clone();

    for chess_move in board.pseudo_legal_moves() {
      board.make_move(&chess_move);
      if !board.in_check(&side_to_move) {
        let to_add = perft(cur_depth - 1, board, depth);

        if cur_depth == depth {
          println!("{}: {}", chess_move.to_fen(), to_add);
        }

        nodes += to_add;
      }
      board.undo_move(&chess_move);
    }

    nodes
  }

  let nodes = perft(depth, &mut board, depth);
  println!("Depth = {depth}, total nodes = {nodes}");

  let time_elapsed = current_time.elapsed();
  println!("Time taken: {:?}", time_elapsed);

  let seconds = time_elapsed.as_secs_f64();

  println!("Speed: {} Mn/s", ((nodes as f64 / 1_000_000.) / seconds).round())
}

// fn mtdf(board: &mut Board, first: i32, depth: i32, tt: &mut TranspositionTable) -> (i32, Move) {
//   let mut g = first;
//   let mut beta: i32;
//   let mut upperbound = INF;
//   let mut lowerbound = -INF;
//   let mut best_move: Move;

//   loop {
//     if g == lowerbound {
//       beta = g + 1;
//     } else {
//       beta = g;
//     }

//     g = alpha_beta_tt(board, tt, beta - 1, beta, depth);
//     let tte = tt.get(&board.meta.hash).expect("Root node not in TT");
//     best_move = tte.best_move.clone().expect("Root node does not have best move");

//     if g < beta {
//       upperbound = g;
//     } else {
//       lowerbound = g;
//     }

//     if lowerbound >= upperbound {
//       break;
//     }
//   }

//   (g, best_move)
// }

// fn sss(board: &mut Board, depth: i32, tt: &mut TranspositionTable) -> (i32, Move) {
//   let mut g: i32 = INF;
//   let mut w: i32;
//   let mut best_move: Move;

//   loop {
//     w = g;
//     g = alpha_beta_tt(board, tt, w - 1, 1, depth);
//     let tte = tt.get(&board.meta.hash).expect("Root node not in TT");
//     best_move = tte.best_move.clone().expect("Root node does not have best move");

//     if g == w {
//       break;
//     }
//   }

//   (g, best_move)
// }

fn find_best_move(board: &mut Board, time_given: u128, tt: &mut TranspositionTable) -> Move {
  let (mut best_eval, best_move) = iterative_deepening(board, time_given, tt);

  if board.side_to_move == Color::Black {
    best_eval *= -1;
  }

  println!("- Eval: {best_eval}");
  println!("- TT size: {}", tt.len());

  best_move
}

#[allow(clippy::too_many_arguments)]
fn aspiration(
  board: &mut Board,
  time_given: u128,
  tt: &mut TranspositionTable,
  stopped: &mut bool,
  start: &Instant,
  depth: i32,
  prev_value: i32,
  window: i32,
) -> i32 {
  let alpha = prev_value - window;
  let beta = prev_value + window;

  let mut value = alpha_beta_tt_i(board, tt, alpha, beta, depth, stopped, start, time_given);

  if value >= beta {
    value = alpha_beta_tt_i(board, tt, value, INF, depth, stopped, start, time_given);
  } else if value <= alpha {
    value = alpha_beta_tt_i(board, tt, -INF, value, depth, stopped, start, time_given);
  }

  value
}

fn iterative_deepening_asp(board: &mut Board, time_given: u128, tt: &mut TranspositionTable) -> (i32, Move) {
  let mut best = 0;
  let mut best_move: Option<Move> = None;
  let started_time = Instant::now();

  for depth in 1..=100 {
    let mut stopped = false;
    let eval = aspiration(board, time_given, tt, &mut stopped, &started_time, depth, best, 100);

    if stopped {
      println!("- Time limit reached");
      println!("- Fully searched to depth {}", depth - 1);
      break;
    }

    let tte = tt.get(&board.meta.hash).expect("Root node not in TT");
    best = eval;
    best_move = tte.best_move.clone();

    let dt = started_time.elapsed();

    if best >= CHECKMATE {
      println!("- Found checkmate");
      println!("- Searched to depth {}", depth);
      break;
    }

    if dt.as_millis() >= time_given {
      println!("- Time limit exceeded");
      println!("- Searched to depth {}", depth);
      break;
    }
  }

  (best, best_move.unwrap())
}

fn iterative_deepening(board: &mut Board, time_given: u128, tt: &mut TranspositionTable) -> (i32, Move) {
  let mut best = 0;
  let mut best_move: Option<Move> = None;
  let started_time = Instant::now();

  for depth in 1..=100 {
    let mut stopped = false;
    let eval = alpha_beta_tt_i(board, tt, -INF, INF, depth, &mut stopped, &started_time, time_given);
    if stopped {
      println!("- Time limit reached");
      println!("- Fully searched to depth {}", depth - 1);
      break;
    }

    let tte = tt.get(&board.meta.hash).expect("Root node not in TT");
    best = eval;
    best_move = tte.best_move.clone();

    let dt = started_time.elapsed();

    if best >= CHECKMATE {
      println!("- Found checkmate");
      println!("- Searched to depth {}", depth);
      break;
    }

    if dt.as_millis() >= time_given {
      println!("- Time limit exceeded");
      println!("- Searched to depth {}", depth);
      break;
    }
  }

  (best, best_move.unwrap())
}

// fn iterative_deepening_mtd(board: &mut Board, time_given: u128) -> (i32, Move) {
//   let mut best = 0;
//   let mut best_move: Option<Move> = None;
//   let mut tt: TranspositionTable = HashMap::new();
//   let started_time = Instant::now();

//   for depth in 1..=100 {
//     let (some_guess, some_move) = mtdf(board, best, depth, &mut tt);
//     best = some_guess;
//     best_move = Some(some_move);

//     let dt = started_time.elapsed();

//     if (CHECKMATE..CHECKMATE + 100).contains(&best) {
//       println!("- Found checkmate");
//       println!("- Searched to depth {}", depth);
//       break;
//     }

//     if dt.as_millis() >= time_given {
//       println!("- Time limit exceeded");
//       println!("- Searched to depth {}", depth);
//       break;
//     }
//   }

//   println!("- TT size: {}", tt.len());

//   (best, best_move.unwrap())
// }

fn save_tte(board: &Board, tt: &mut TranspositionTable, value: i32, depth: i32, alpha: i32, beta: i32, best_move: Option<Move>) {
  let typ = if value <= alpha {
    UPPERBOUND
  } else if value >= beta {
    LOWERBOUND
  } else {
    EXACT_VALUE
  };

  let tte = TTEntry {
    typ,
    value,
    depth,
    best_move,
  };

  tt.insert(board.meta.hash, tte);
}

const CHECKMATE: i32 = 100000;
const INF: i32 = 10000000;

const EXACT_VALUE: u8 = 0;
const LOWERBOUND: u8 = 1;
const UPPERBOUND: u8 = 2;

#[derive(Debug)]
struct TTEntry {
  typ: u8,
  value: i32,
  depth: i32,
  best_move: Option<Move>,
}

type TranspositionTable = HashMap<u64, TTEntry>;

#[allow(clippy::too_many_arguments)]
fn alpha_beta_tt_i(
  board: &mut Board,
  tt: &mut TranspositionTable,
  mut alpha: i32,
  mut beta: i32,
  depth: i32,
  stopped: &mut bool,
  start: &Instant,
  limit: u128,
) -> i32 {
  if start.elapsed().as_millis() > limit {
    *stopped = true;
    return 0;
  }

  let mut value: i32;
  let option_tte = tt.get(&board.meta.hash);
  if let Some(tte) = option_tte {
    if tte.depth >= depth {
      if tte.typ == EXACT_VALUE {
        return tte.value;
      }

      if tte.typ == LOWERBOUND && tte.value > alpha {
        alpha = tte.value;
      } else if tte.typ == UPPERBOUND && tte.value < beta {
        beta = tte.value;
      }

      if alpha >= beta {
        return tte.value;
      }
    }
  }

  if depth == 0 {
    value = quiesce_i(board, alpha, beta, stopped, start, limit);

    save_tte(board, tt, value, depth, alpha, beta, None);

    return value;
  }

  let side_to_move = board.side_to_move.clone();

  let pseudo_legal_moves = board.pseudo_legal_moves();

  if pseudo_legal_moves.is_empty() {
    if board.in_check(&side_to_move) {
      // In check and can't move => Checkmate
      value = -CHECKMATE - depth;
    } else {
      // Not in check and can't move => Stalemate
      value = 0;
    }

    save_tte(board, tt, value, depth, alpha, beta, None);

    return value;
  }

  let mut cant_move = true;
  let mut best: i32 = -INF;
  let mut best_move: Option<Move> = None;

  if let Some(tte) = option_tte {
    if let Some(good_move) = &tte.best_move {
      let some_move = good_move.clone();

      board.make_move(&some_move);
      value = -alpha_beta_tt_i(board, tt, -beta, -alpha, depth - 1, stopped, start, limit);
      board.undo_move(&some_move);
      if *stopped {
        return 0;
      }

      if value > best {
        best = value;
        best_move = Some(some_move);
      }
      if best > alpha {
        alpha = best;
      }
      if best >= beta {
        save_tte(board, tt, best, depth, alpha, beta, best_move);
        return best;
      }
    }
  }

  for pseudo_legal_move in pseudo_legal_moves {
    board.make_move(&pseudo_legal_move);

    if !board.in_check(&side_to_move) {
      // It is a legal move
      cant_move = false;

      value = -alpha_beta_tt_i(board, tt, -beta, -alpha, depth - 1, stopped, start, limit);
      board.undo_move(&pseudo_legal_move);
      if *stopped {
        return 0;
      }

      if value > best {
        best = value;
        best_move = Some(pseudo_legal_move.clone());
      }
      if best > alpha {
        alpha = best;
      }
      if best >= beta {
        break;
      }
    } else {
      board.undo_move(&pseudo_legal_move);
    }
  }

  if cant_move {
    if board.in_check(&side_to_move) {
      // In check and can't move => Checkmate
      value = -CHECKMATE - depth;
    } else {
      // Not in check and can't move => Stalemate
      value = 0;
    }

    save_tte(board, tt, value, depth, alpha, beta, None);

    return value;
  }

  save_tte(board, tt, best, depth, alpha, beta, best_move);

  best
}

fn quiesce_i(board: &mut Board, mut alpha: i32, beta: i32, stopped: &mut bool, start: &Instant, limit: u128) -> i32 {
  if start.elapsed().as_millis() > limit {
    *stopped = true;
    return 0;
  }

  let standing_eval = board.evaluate_relative();

  if standing_eval >= beta {
    return beta;
  }

  if alpha < standing_eval {
    alpha = standing_eval;
  }

  let side_to_move = board.side_to_move.clone();

  let pseudo_legal_moves = board.pseudo_legal_moves();

  if pseudo_legal_moves.is_empty() {
    if board.in_check(&side_to_move) {
      // In check and can't move => Checkmate
      return -CHECKMATE;
    } else {
      // Not in check and can't move => Stalemate
      return 0;
    }
  }

  let mut cant_move = true;

  for pseudo_legal_move in pseudo_legal_moves {
    board.make_move(&pseudo_legal_move);

    if !board.in_check(&side_to_move) {
      // It is a legal move
      cant_move = false;

      let should_branch = pseudo_legal_move.is_capture() || board.in_check(&board.side_to_move);

      if should_branch {
        let score = -quiesce_i(board, -beta, -alpha, stopped, start, limit);

        if *stopped {
          board.undo_move(&pseudo_legal_move);
          return 0;
        }

        if score >= beta {
          board.undo_move(&pseudo_legal_move);
          return beta;
        }
        if score > alpha {
          alpha = score;
        }
      }
    }

    board.undo_move(&pseudo_legal_move);
  }

  if cant_move {
    if board.in_check(&side_to_move) {
      // In check and can't move => Checkmate
      return -CHECKMATE;
    } else {
      // Not in check and can't move => Stalemate
      return 0;
    }
  }

  alpha
}

struct Packet {
  fen: String,
  // time is in ms
  white_time_left: isize,
  black_time_left: isize,
}

fn decode_packet(buf: &[u8]) -> Packet {
  let packet = String::from_utf8(buf.to_vec()).expect("Couldn't parse packet");

  println!("- Received packet: '{packet}'");

  let strings: Vec<&str> = packet.split(' ').collect();
  let fen = strings[..6].join(" ");
  let white_time_left = strings[6].parse::<isize>().expect("Couldn't parse time remaining");
  let black_time_left = strings[7].parse::<isize>().expect("Couldn't parse time remaining");

  Packet {
    fen,
    white_time_left,
    black_time_left,
  }
}
