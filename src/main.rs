// #[global_allocator]
// static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
mod board;

use std::time::Instant;

use board::Board;

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

fn main() {
  perft_div("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8", 6);
}
