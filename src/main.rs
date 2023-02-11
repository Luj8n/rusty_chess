// #[global_allocator]
// static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
mod board;

use std::time::Instant;

use board::Board;

fn main() {
  let current_time = Instant::now();

  let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

  const DEPTH: usize = 6;

  fn perft(depth: usize, board: &mut Board) -> u64 {
    let mut nodes = 0;

    if depth == 0 {
      return 1;
    }

    let side_to_move = board.side_to_move.clone();

    for chess_move in board.pseudo_legal_moves() {
      board.make_move(&chess_move);
      if !board.in_check(&side_to_move) {
        let to_add = perft(depth - 1, board);

        if depth == DEPTH {
          println!("{}: {}", chess_move.to_fen(), to_add);
        }

        nodes += to_add;
      }
      board.undo_move(&chess_move);
    }

    nodes
  }

  let nodes = perft(DEPTH, &mut board);
  println!("Depth = {DEPTH}, total nodes = {nodes}");

  let time_elapsed = current_time.elapsed();
  println!("Time taken: {:?}", time_elapsed);

  let seconds = time_elapsed.as_secs_f64();

  println!("Speed: {} Mn/s", ((nodes as f64 / 1_000_000.) / seconds).round())
}
