mod board;

use std::time::Instant;

use board::Board;
use std::env;
use tokio::{
  io::{AsyncReadExt, AsyncWriteExt},
  net::TcpStream,
};

use crate::board::Color;

const BLACK_SIDE: bool = true;

fn perft_div(fen: &str, depth: usize) {
  let current_time = Instant::now();

  let mut board = Board::from_fen(fen);

  dbg!(board.evaluate());

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

fn best_move(fen: &str, depth: usize) {
  let current_time = Instant::now();

  let mut board = Board::from_fen(fen);

  fn eval(board: &mut Board, moves_left: usize) -> i32 {
    if moves_left == 0 {
      return board.evaluate();
    }

    let pseudo_legal_moves = board.pseudo_legal_moves();

    if pseudo_legal_moves.is_empty() {
      if board.in_check(&board.side_to_move) {
        // In check and can't move => Checkmate
        return if board.side_to_move == Color::White { -10000 } else { 10000 };
      } else {
        // Not in check and can't move => Stalemate
        return 0;
      }
    }

    let side_to_move = board.side_to_move.clone();

    let evals = pseudo_legal_moves.into_iter().filter_map(|chess_move| {
      board.make_move(&chess_move);
      if !board.in_check(&side_to_move) {
        let child_eval = eval(board, moves_left - 1);

        board.undo_move(&chess_move);

        Some(child_eval)
      } else {
        board.undo_move(&chess_move);

        None
      }
    });

    if side_to_move == Color::White {
      evals.max().unwrap_or_else(|| {
        if board.in_check(&side_to_move) {
          // In check and can't move => Checkmate
          -10000
        } else {
          // Not in check and can't move => Stalemate
          0
        }
      })
    } else {
      evals.min().unwrap_or_else(|| {
        if board.in_check(&side_to_move) {
          // In check and can't move => Checkmate
          10000
        } else {
          // Not in check and can't move => Stalemate
          0
        }
      })
    }
  }

  let side_to_move = board.side_to_move.clone();

  let evals = board.legal_moves().into_iter().map(|chess_move| {
    board.make_move(&chess_move);
    let chess_move_eval = eval(&mut board, depth - 1);
    board.undo_move(&chess_move);

    (chess_move_eval, chess_move.to_fen())
  });

  if side_to_move == Color::White {
    if let Some(best) = evals.max_by_key(|(e, _)| *e) {
      println!("Best move: {} | Eval: {}", best.1, best.0);
    } else {
      println!("No legal moves");
    }
  } else if side_to_move == Color::Black {
    if let Some(best) = evals.min_by_key(|(e, _)| *e) {
      println!("Best move: {} | Eval: {}", best.1, best.0);
    } else {
      println!("No legal moves");
    }
  }

  let time_elapsed = current_time.elapsed();
  println!("Time taken: {:?}", time_elapsed);
}

struct Packet {
  fen: String,
  // time is in ms
  white_time_left: isize,
  black_time_left: isize,
}

fn decode_packet(buf: &[u8]) -> Packet {
  let packet = String::from_utf8(buf.to_vec()).expect("Couldn't parse packet");

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

#[tokio::main]
async fn main() {
  let port = if BLACK_SIDE { 6970 } else { 6969 };

  if let Ok(mut stream) = TcpStream::connect(format!("127.0.0.1:{port}")).await {
    println!("Connected to the interface");

    let mut buf = [0_u8; 1024];

    println!("Waiting for fen...");
    while let Ok(bytes_read) = stream.read(&mut buf).await {
      if bytes_read <= 1 {
        break;
      }

      let packet = decode_packet(&buf[..bytes_read]);

      let fen = packet.fen;
      println!("Received fen: '{fen}'");

      let mut board = Board::from_fen(&fen);

      let legal_moves = board.legal_moves();
      let chess_move = &legal_moves[rand::random::<usize>() % legal_moves.len()];
      let chess_move_fen = chess_move.to_fen();

      println!("Sending move: '{chess_move_fen}'...");
      stream.write_all(chess_move_fen.as_bytes()).await.expect("Couldn't send move");
      println!("Sent move successfully");

      buf.fill(0);

      println!("Receiving fen...");
    }

    println!("Disconnecting");
    stream.shutdown().await.expect("Couldn't shutdown stream");
  } else {
    println!("Couldn't connect to the interface");
  }
}
