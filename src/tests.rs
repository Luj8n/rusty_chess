use crate::{bitboard, board};

#[test]
fn is_same() {
  let mut board = board::Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
  let board_bb = bitboard::Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");

  assert_eq!(board.evaluate_relative(), board_bb.evaluate_relative());

  let mut eval = 0;
  let mut eval_bb = 0;

  let moves = board.pseudo_legal_moves();
  let moves_bb = board_bb.pseudo_legal_moves();

  let c = board.side_to_move.clone();
  let c_bb = if board_bb.white_to_move { 0 } else { 1 };

  for m in &moves {
    board.make_move(m);
    if !board.in_check(&c) {
      eval += board.evaluate_relative();
    }
    board.undo_move(m);
  }

  for m in &moves_bb {
    let mut new_board_bb = board_bb.clone();
    new_board_bb.make_move(m);
    if !new_board_bb.in_check(c_bb) {
      eval_bb += new_board_bb.evaluate_relative();
    }
  }

  assert_eq!(eval, eval_bb);

  let mut count = 0;
  let mut count_bb = 0;

  for m in &moves {
    if m.is_capture() {
      count += 1;
    }
  }

  for m in &moves_bb {
    if m.is_capture(&board_bb) {
      count_bb += 1;
    }
  }

  assert_eq!(count, count_bb);
}
