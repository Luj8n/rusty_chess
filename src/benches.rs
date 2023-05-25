use crate::bitboard;
use crate::board;

extern crate test;
use test::Bencher;

#[bench]
fn pseudo_legal_moves(b: &mut Bencher) {
  let board = board::Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
  b.iter(|| {
    let moves = board.pseudo_legal_moves();
    assert!(!moves.is_empty())
  });
}

#[bench]
fn pseudo_legal_moves_bb(b: &mut Bencher) {
  let board = bitboard::Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
  b.iter(|| {
    let moves = board.pseudo_legal_moves();
    assert!(!moves.is_empty())
  });
}

#[bench]
fn make_unmake_moves(b: &mut Bencher) {
  let mut board = board::Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
  let moves = board.pseudo_legal_moves();
  let mut c = 0;
  b.iter(|| {
    for chess_move in &moves {
      board.make_move(chess_move);
      if board.side_to_move == board::Color::Black {
        c += 1;
      }
      board.undo_move(chess_move);
    }
  });
  dbg!(c);
}

#[bench]
fn make_unmake_moves_bb(b: &mut Bencher) {
  let board = bitboard::Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
  let moves = board.pseudo_legal_moves();
  let mut c = 0;
  b.iter(|| {
    for chess_move in &moves {
      let mut new_board = board.clone();
      new_board.make_move(chess_move);
      if !new_board.white_to_move {
        c += 1
      }
    }
  });
  dbg!(c);
}

#[bench]
fn legal_moves(b: &mut Bencher) {
  let mut board = board::Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
  let side = board.side_to_move.clone();
  let moves = board.pseudo_legal_moves();
  let mut c = 0;
  b.iter(|| {
    for chess_move in &moves {
      board.make_move(chess_move);
      let in_check = board.in_check(&side);
      if in_check {
        c += 1;
      }
      board.undo_move(chess_move);
    }
  });
  dbg!(c);
}

#[bench]
fn legal_moves_bb(b: &mut Bencher) {
  let board = bitboard::Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
  let color = if board.white_to_move { 0 } else { 1 };
  let moves = board.pseudo_legal_moves();
  let mut c = 0;

  b.iter(|| {
    for chess_move in &moves {
      let mut new_board = board.clone();
      new_board.make_move(chess_move);
      let in_check = new_board.in_check(color);
      if in_check {
        c += 1;
      }
    }
  });
  dbg!(c);
}

#[bench]
fn evaluate(b: &mut Bencher) {
  let board = board::Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
  let mut c = 0;
  b.iter(|| {
    let eval = board.evaluate_relative();
    if eval > 0 {
      c += 1;
    }
  });
  dbg!(c);
}

#[bench]
fn evaluate_bb(b: &mut Bencher) {
  let board = bitboard::Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
  let mut c = 0;
  b.iter(|| {
    let eval = board.evaluate_relative();
    if eval > 0 {
      c += 1;
    }
  });
  dbg!(c);
}

#[bench]
fn capture_moves(b: &mut Bencher) {
  let board = board::Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
  let moves = board.pseudo_legal_moves();
  let mut c = 0;
  b.iter(|| {
    for chess_move in &moves {
      if chess_move.is_capture() {
        c += 1;
      }
    }
  });
  dbg!(c);
}

#[bench]
fn capture_moves_bb(b: &mut Bencher) {
  let board = bitboard::Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
  let moves = board.pseudo_legal_moves();
  let mut c = 0;
  b.iter(|| {
    for chess_move in &moves {
      if chess_move.is_capture(&board) {
        c += 1;
      }
    }
  });
  dbg!(c);
}

#[bench]
fn perft(b: &mut Bencher) {
  let mut board = board::Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");

  fn p(depth: usize, board: &mut board::Board) -> u64 {
    let mut nodes = 0;

    if depth == 0 {
      return 1;
    }

    let side_to_move = board.side_to_move.clone();

    for chess_move in board.pseudo_legal_moves() {
      board.make_move(&chess_move);
      if !board.in_check(&side_to_move) {
        nodes += p(depth - 1, board);
      }
      board.undo_move(&chess_move);
    }

    nodes
  }

  b.iter(|| {
    p(3, &mut board);
  });
}

#[bench]
fn perft_bb(b: &mut Bencher) {
  let board = bitboard::Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");

  fn p(depth: usize, board: bitboard::Board) -> u64 {
    let mut nodes = 0;

    if depth == 0 {
      return 1;
    }

    let color = if board.white_to_move { 0 } else { 1 };

    for chess_move in board.pseudo_legal_moves() {
      let mut new_board = board.clone();
      new_board.make_move(&chess_move);
      if !new_board.in_check(color) {
        nodes += p(depth - 1, new_board);
      }
    }

    nodes
  }

  b.iter(|| {
    p(3, board.clone());
  });
}
