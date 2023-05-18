#![feature(const_mut_refs)]
#![feature(const_eval_limit)]
#![const_eval_limit = "10000000"]

mod bitboard;

fn perft_div(fen: &str, depth: usize) {
  let current_time = Instant::now();

  let mut board = bitboard::Board::from_fen(fen);

  // dbg!(board.evaluate());

  fn perft(cur_depth: usize, board: &mut bitboard::Board, depth: usize) -> u64 {
    let mut nodes = 0;

    if cur_depth == 0 {
      return 1;
    }

    let color = if board.white_to_move { 0 } else { 1 };

    for chess_move in board.pseudo_legal_moves() {
      let mut new_board = board.clone();

      new_board.make_move(&chess_move);
      if !new_board.in_check(color) {
        let to_add = perft(cur_depth - 1, &mut new_board, depth);

        if cur_depth == depth {
          println!("{}: {}", chess_move.to_fen(), to_add);
        }

        nodes += to_add;
      }
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
