use bitintr::Pext;

use lazy_static::lazy_static;
use rand::prelude::*;

const WHITE: usize = 0;
const BLACK: usize = 1;

const PAWN: usize = 2;
const KNIGHT: usize = 4;
const BISHOP: usize = 6;
const ROOK: usize = 8;
const QUEEN: usize = 10;
const KING: usize = 12;

const EMPTY_SQUARE: usize = 14;

// Bitboard representation:
// 8 | 63 62 61 60 59 58 57 56
// 7 | 55 54 53 52 51 50 49 48
// 6 | 47 46 45 44 43 42 41 40
// 5 | 39 38 37 36 35 34 33 32
// 4 | 31 30 29 28 27 26 25 24
// 3 | 23 22 21 20 19 18 17 16
// 2 | 15 14 13 12 11 10 09 08
// 1 | 07 06 05 04 03 02 01 00
//   +------------------------
//     A  B  C  D  E  F  G  H

pub const FILE_A: u64 = 0x8080808080808080;
pub const FILE_B: u64 = 0x4040404040404040;
pub const FILE_C: u64 = 0x2020202020202020;
pub const FILE_D: u64 = 0x1010101010101010;
pub const FILE_E: u64 = 0x0808080808080808;
pub const FILE_F: u64 = 0x0404040404040404;
pub const FILE_G: u64 = 0x0202020202020202;
pub const FILE_H: u64 = 0x0101010101010101;

pub const RANK_1: u64 = 0x00000000000000ff;
pub const RANK_2: u64 = 0x000000000000ff00;
pub const RANK_3: u64 = 0x0000000000ff0000;
pub const RANK_4: u64 = 0x00000000ff000000;
pub const RANK_5: u64 = 0x000000ff00000000;
pub const RANK_6: u64 = 0x0000ff0000000000;
pub const RANK_7: u64 = 0x00ff000000000000;
pub const RANK_8: u64 = 0xff00000000000000;

pub const A1: u64 = FILE_A & RANK_1;
pub const A2: u64 = FILE_A & RANK_2;
pub const A3: u64 = FILE_A & RANK_3;
pub const A4: u64 = FILE_A & RANK_4;
pub const A5: u64 = FILE_A & RANK_5;
pub const A6: u64 = FILE_A & RANK_6;
pub const A7: u64 = FILE_A & RANK_7;
pub const A8: u64 = FILE_A & RANK_8;
pub const B1: u64 = FILE_B & RANK_1;
pub const B2: u64 = FILE_B & RANK_2;
pub const B3: u64 = FILE_B & RANK_3;
pub const B4: u64 = FILE_B & RANK_4;
pub const B5: u64 = FILE_B & RANK_5;
pub const B6: u64 = FILE_B & RANK_6;
pub const B7: u64 = FILE_B & RANK_7;
pub const B8: u64 = FILE_B & RANK_8;
pub const C1: u64 = FILE_C & RANK_1;
pub const C2: u64 = FILE_C & RANK_2;
pub const C3: u64 = FILE_C & RANK_3;
pub const C4: u64 = FILE_C & RANK_4;
pub const C5: u64 = FILE_C & RANK_5;
pub const C6: u64 = FILE_C & RANK_6;
pub const C7: u64 = FILE_C & RANK_7;
pub const C8: u64 = FILE_C & RANK_8;
pub const D1: u64 = FILE_D & RANK_1;
pub const D2: u64 = FILE_D & RANK_2;
pub const D3: u64 = FILE_D & RANK_3;
pub const D4: u64 = FILE_D & RANK_4;
pub const D5: u64 = FILE_D & RANK_5;
pub const D6: u64 = FILE_D & RANK_6;
pub const D7: u64 = FILE_D & RANK_7;
pub const D8: u64 = FILE_D & RANK_8;
pub const E1: u64 = FILE_E & RANK_1;
pub const E2: u64 = FILE_E & RANK_2;
pub const E3: u64 = FILE_E & RANK_3;
pub const E4: u64 = FILE_E & RANK_4;
pub const E5: u64 = FILE_E & RANK_5;
pub const E6: u64 = FILE_E & RANK_6;
pub const E7: u64 = FILE_E & RANK_7;
pub const E8: u64 = FILE_E & RANK_8;
pub const F1: u64 = FILE_F & RANK_1;
pub const F2: u64 = FILE_F & RANK_2;
pub const F3: u64 = FILE_F & RANK_3;
pub const F4: u64 = FILE_F & RANK_4;
pub const F5: u64 = FILE_F & RANK_5;
pub const F6: u64 = FILE_F & RANK_6;
pub const F7: u64 = FILE_F & RANK_7;
pub const F8: u64 = FILE_F & RANK_8;
pub const G1: u64 = FILE_G & RANK_1;
pub const G2: u64 = FILE_G & RANK_2;
pub const G3: u64 = FILE_G & RANK_3;
pub const G4: u64 = FILE_G & RANK_4;
pub const G5: u64 = FILE_G & RANK_5;
pub const G6: u64 = FILE_G & RANK_6;
pub const G7: u64 = FILE_G & RANK_7;
pub const G8: u64 = FILE_G & RANK_8;
pub const H1: u64 = FILE_H & RANK_1;
pub const H2: u64 = FILE_H & RANK_2;
pub const H3: u64 = FILE_H & RANK_3;
pub const H4: u64 = FILE_H & RANK_4;
pub const H5: u64 = FILE_H & RANK_5;
pub const H6: u64 = FILE_H & RANK_6;
pub const H7: u64 = FILE_H & RANK_7;
pub const H8: u64 = FILE_H & RANK_8;

fn square_to_bitboard(square: &str) -> u64 {
  match &square.to_lowercase()[..] {
    "a1" => A1,
    "a2" => A2,
    "a3" => A3,
    "a4" => A4,
    "a5" => A5,
    "a6" => A6,
    "a7" => A7,
    "a8" => A8,
    "b1" => B1,
    "b2" => B2,
    "b3" => B3,
    "b4" => B4,
    "b5" => B5,
    "b6" => B6,
    "b7" => B7,
    "b8" => B8,
    "c1" => C1,
    "c2" => C2,
    "c3" => C3,
    "c4" => C4,
    "c5" => C5,
    "c6" => C6,
    "c7" => C7,
    "c8" => C8,
    "d1" => D1,
    "d2" => D2,
    "d3" => D3,
    "d4" => D4,
    "d5" => D5,
    "d6" => D6,
    "d7" => D7,
    "d8" => D8,
    "e1" => E1,
    "e2" => E2,
    "e3" => E3,
    "e4" => E4,
    "e5" => E5,
    "e6" => E6,
    "e7" => E7,
    "e8" => E8,
    "f1" => F1,
    "f2" => F2,
    "f3" => F3,
    "f4" => F4,
    "f5" => F5,
    "f6" => F6,
    "f7" => F7,
    "f8" => F8,
    "g1" => G1,
    "g2" => G2,
    "g3" => G3,
    "g4" => G4,
    "g5" => G5,
    "g6" => G6,
    "g7" => G7,
    "g8" => G8,
    "h1" => H1,
    "h2" => H2,
    "h3" => H3,
    "h4" => H4,
    "h5" => H5,
    "h6" => H6,
    "h7" => H7,
    "h8" => H8,
    _ => panic!(),
  }
}

// Converts a bitboard, which has only one 1 bit, to a square, which is a string.
fn bitboard_to_square(bitboard: u64) -> &'static str {
  match bitboard {
    A1 => "a1",
    A2 => "a2",
    A3 => "a3",
    A4 => "a4",
    A5 => "a5",
    A6 => "a6",
    A7 => "a7",
    A8 => "a8",
    B1 => "b1",
    B2 => "b2",
    B3 => "b3",
    B4 => "b4",
    B5 => "b5",
    B6 => "b6",
    B7 => "b7",
    B8 => "b8",
    C1 => "c1",
    C2 => "c2",
    C3 => "c3",
    C4 => "c4",
    C5 => "c5",
    C6 => "c6",
    C7 => "c7",
    C8 => "c8",
    D1 => "d1",
    D2 => "d2",
    D3 => "d3",
    D4 => "d4",
    D5 => "d5",
    D6 => "d6",
    D7 => "d7",
    D8 => "d8",
    E1 => "e1",
    E2 => "e2",
    E3 => "e3",
    E4 => "e4",
    E5 => "e5",
    E6 => "e6",
    E7 => "e7",
    E8 => "e8",
    F1 => "f1",
    F2 => "f2",
    F3 => "f3",
    F4 => "f4",
    F5 => "f5",
    F6 => "f6",
    F7 => "f7",
    F8 => "f8",
    G1 => "g1",
    G2 => "g2",
    G3 => "g3",
    G4 => "g4",
    G5 => "g5",
    G6 => "g6",
    G7 => "g7",
    G8 => "g8",
    H1 => "h1",
    H2 => "h2",
    H3 => "h3",
    H4 => "h4",
    H5 => "h5",
    H6 => "h6",
    H7 => "h7",
    H8 => "h8",
    _ => panic!(),
  }
}

const WHITE_PAWN_EVAL: [i32; 64] = [
  0, 0, 0, 0, 0, 0, 0, 0, //
  3, 3, 3, 3, 3, 3, 3, 3, //
  1, 1, 1, 1, 1, 1, 1, 1, //
  1, 1, 1, 1, 1, 1, 1, 1, //
  2, 2, 2, 2, 2, 2, 2, 2, //
  2, 2, 2, 2, 2, 2, 2, 2, //
  1, 1, 1, 1, 1, 1, 1, 1, //
  0, 0, 0, 0, 0, 0, 0, 0, //
];

const BLACK_PAWN_EVAL: [i32; 64] = [
  0, 0, 0, 0, 0, 0, 0, 0, //
  1, 1, 1, 1, 1, 1, 1, 1, //
  2, 2, 2, 2, 2, 2, 2, 2, //
  2, 2, 2, 2, 2, 2, 2, 2, //
  1, 1, 1, 1, 1, 1, 1, 1, //
  1, 1, 1, 1, 1, 1, 1, 1, //
  3, 3, 3, 3, 3, 3, 3, 3, //
  0, 0, 0, 0, 0, 0, 0, 0, //
];

const WHITE_KNIGHT_EVAL: [i32; 64] = [
  0, 0, 0, 0, 0, 0, 0, 0, //
  1, 2, 2, 2, 2, 2, 2, 1, //
  1, 3, 3, 4, 4, 3, 3, 1, //
  1, 3, 4, 5, 5, 4, 3, 1, //
  1, 3, 4, 5, 5, 4, 3, 1, //
  1, 3, 3, 4, 4, 3, 3, 1, //
  1, 2, 2, 2, 2, 2, 2, 1, //
  0, 0, 0, 0, 0, 0, 0, 0, //
];

const BLACK_KNIGHT_EVAL: [i32; 64] = [
  0, 0, 0, 0, 0, 0, 0, 0, //
  1, 2, 2, 2, 2, 2, 2, 1, //
  1, 3, 3, 4, 4, 3, 3, 1, //
  1, 3, 4, 5, 5, 4, 3, 1, //
  1, 3, 4, 5, 5, 4, 3, 1, //
  1, 3, 3, 4, 4, 3, 3, 1, //
  1, 2, 2, 2, 2, 2, 2, 1, //
  0, 0, 0, 0, 0, 0, 0, 0, //
];

const WHITE_BISHOP_EVAL: [i32; 64] = [
  0, 0, 1, 1, 1, 1, 0, 0, //
  0, 2, 2, 2, 2, 2, 2, 0, //
  1, 2, 2, 2, 2, 2, 2, 1, //
  1, 2, 2, 2, 2, 2, 2, 1, //
  1, 2, 2, 2, 2, 2, 2, 1, //
  1, 2, 2, 2, 2, 2, 2, 1, //
  0, 2, 2, 2, 2, 2, 2, 0, //
  0, 0, 1, 1, 1, 1, 0, 0, //
];

const BLACK_BISHOP_EVAL: [i32; 64] = [
  0, 0, 1, 1, 1, 1, 0, 0, //
  0, 2, 2, 2, 2, 2, 2, 0, //
  1, 2, 2, 2, 2, 2, 2, 1, //
  1, 2, 2, 2, 2, 2, 2, 1, //
  1, 2, 2, 2, 2, 2, 2, 1, //
  1, 2, 2, 2, 2, 2, 2, 1, //
  0, 2, 2, 2, 2, 2, 2, 0, //
  0, 0, 1, 1, 1, 1, 0, 0, //
];

const WHITE_ROOK_EVAL: [i32; 64] = [
  5, 3, 2, 2, 2, 2, 3, 5, //
  5, 5, 5, 5, 5, 5, 5, 5, //
  3, 2, 2, 2, 2, 2, 2, 3, //
  2, 1, 1, 1, 1, 1, 1, 2, //
  2, 1, 1, 1, 1, 1, 1, 2, //
  3, 2, 2, 2, 2, 2, 2, 3, //
  5, 5, 5, 5, 5, 5, 5, 5, //
  5, 3, 2, 2, 2, 2, 3, 5, //
];

const BLACK_ROOK_EVAL: [i32; 64] = [
  5, 3, 2, 2, 2, 2, 3, 5, //
  5, 5, 5, 5, 5, 5, 5, 5, //
  3, 2, 2, 2, 2, 2, 2, 3, //
  2, 1, 1, 1, 1, 1, 1, 2, //
  2, 1, 1, 1, 1, 1, 1, 2, //
  3, 2, 2, 2, 2, 2, 2, 3, //
  5, 5, 5, 5, 5, 5, 5, 5, //
  5, 3, 2, 2, 2, 2, 3, 5, //
];

const WHITE_QUEEN_EVAL: [i32; 64] = [
  1, 1, 1, 1, 1, 1, 1, 1, //
  1, 2, 2, 2, 2, 2, 2, 1, //
  1, 2, 2, 2, 2, 2, 2, 1, //
  1, 2, 2, 2, 2, 2, 2, 1, //
  1, 2, 2, 2, 2, 2, 2, 1, //
  1, 2, 2, 2, 2, 2, 2, 1, //
  1, 2, 2, 2, 2, 2, 2, 1, //
  1, 1, 1, 1, 1, 1, 1, 1, //
];

const BLACK_QUEEN_EVAL: [i32; 64] = [
  1, 1, 1, 1, 1, 1, 1, 1, //
  1, 2, 2, 2, 2, 2, 2, 1, //
  1, 2, 2, 2, 2, 2, 2, 1, //
  1, 2, 2, 2, 2, 2, 2, 1, //
  1, 2, 2, 2, 2, 2, 2, 1, //
  1, 2, 2, 2, 2, 2, 2, 1, //
  1, 2, 2, 2, 2, 2, 2, 1, //
  1, 1, 1, 1, 1, 1, 1, 1, //
];

const WHITE_KING_EVAL: [i32; 64] = [
  5, 5, 3, 3, 3, 3, 5, 5, //
  3, 5, 5, 5, 5, 5, 5, 3, //
  3, 3, 2, 0, 0, 2, 3, 3, //
  3, 2, 0, 0, 0, 0, 2, 3, //
  3, 2, 0, 0, 0, 0, 2, 3, //
  5, 3, 2, 1, 1, 2, 3, 5, //
  10, 5, 5, 3, 3, 5, 5, 10, //
  5, 10, 15, 5, 3, 5, 15, 10, //
];

const BLACK_KING_EVAL: [i32; 64] = [
  5, 10, 15, 5, 3, 5, 15, 10, //
  10, 5, 5, 3, 3, 5, 5, 10, //
  5, 3, 2, 1, 1, 2, 3, 5, //
  3, 2, 0, 0, 0, 0, 2, 3, //
  3, 2, 0, 0, 0, 0, 2, 3, //
  3, 3, 2, 0, 0, 2, 3, 3, //
  3, 5, 5, 5, 5, 5, 5, 3, //
  5, 5, 3, 3, 3, 3, 5, 5, //
];

const POSSIBLE_CASTLING_VALUE: i32 = 5;

const PAWN_VALUE: i32 = 100;
const KNIGHT_VALUE: i32 = 350;
const BISHOP_VALUE: i32 = 350;
const ROOK_VALUE: i32 = 525;
const QUEEN_VALUE: i32 = 1000;

pub fn print_bitboard(b: u64) {
  for y in 0..8 {
    let mut line = "".to_string();
    for x in 0..8 {
      let i = 63 - (y * 8 + x);
      let c = if get_bit(b, i) != 0 { "@ " } else { ". " };
      line += c;
    }
    println!("{}", line);
  }
}

const fn set_bit(b: &mut u64, i: usize) {
  *b |= 1 << i;
}

// maybe its better have this be an array
const fn get_bit(b: u64, i: usize) -> u64 {
  b & (1 << i)
}

const fn clear_bit(b: &mut u64, i: usize) {
  *b &= !(1 << i);
}

const fn get_lsb(b: u64) -> usize {
  b.trailing_zeros() as usize
}

const fn pop_lsb(b: &mut u64) -> usize {
  let i = get_lsb(*b);
  *b &= *b - 1;
  i
}

const NOT_FILE_A: u64 = !FILE_A;
const NOT_FILE_H: u64 = !FILE_H;

const fn north(b: u64) -> u64 {
  b << 8
}
const fn north_east(b: u64) -> u64 {
  (b & NOT_FILE_H) << 7
}
const fn east(b: u64) -> u64 {
  (b & NOT_FILE_H) >> 1
}
const fn south_east(b: u64) -> u64 {
  (b & NOT_FILE_H) >> 9
}
const fn south(b: u64) -> u64 {
  b >> 8
}
const fn south_west(b: u64) -> u64 {
  (b & NOT_FILE_A) >> 7
}
const fn west(b: u64) -> u64 {
  (b & NOT_FILE_A) << 1
}
const fn north_west(b: u64) -> u64 {
  (b & NOT_FILE_A) << 9
}

pub const KNIGHT_MOVES: [u64; 64] = {
  const fn gen_knight_moves(square: usize) -> u64 {
    let mut b = 0_u64;

    set_bit(&mut b, square);

    ((b << 15 | b >> 17) & !FILE_A)
      | ((b << 6 | b >> 10) & !(FILE_A | FILE_B))
      | ((b << 17 | b >> 15) & !FILE_H)
      | ((b << 10 | b >> 6) & !(FILE_G | FILE_H))
  }

  let mut moves = [0_u64; 64];

  let mut i = 0;
  while i < 64 {
    moves[i] = gen_knight_moves(i);
    i += 1;
  }

  moves
};

pub const KING_MOVES: [u64; 64] = {
  const fn gen_king_moves(square: usize) -> u64 {
    let mut b = 0_u64;

    set_bit(&mut b, square);

    north(b) | north_east(b) | east(b) | south_east(b) | south(b) | south_west(b) | west(b) | north_west(b)
  }

  let mut moves = [0_u64; 64];

  let mut i = 0;
  while i < 64 {
    moves[i] = gen_king_moves(i);
    i += 1;
  }

  moves
};

pub const ROOK_BLOCKER_MASKS: [u64; 64] = {
  const fn gen_rook_blocker_masks(square: usize) -> u64 {
    let mut board: u64 = 0;
    let square = square as isize;

    let mut i = square - 8;
    while i >= 8 {
      board |= 1 << i;
      i -= 8;
    }

    let mut i = square + 1;
    while i % 8 < 7 && i % 8 != 0 {
      board |= 1 << i;
      i += 1;
    }

    let mut i = square + 8;
    while i < 56 {
      board |= 1 << i;
      i += 8;
    }

    let mut i = square - 1;
    while i % 8 > 0 && i % 8 != 7 {
      board |= 1 << i;
      i -= 1;
    }

    board
  }

  let mut moves = [0_u64; 64];

  let mut i = 0;
  while i < 64 {
    moves[i] = gen_rook_blocker_masks(i);
    i += 1;
  }

  moves
};

pub const BISHOP_BLOCKER_MASKS: [u64; 64] = {
  const fn gen_bishop_blocker_masks(square: usize) -> u64 {
    let mut board: u64 = 0;
    let square = square as isize;

    let mut x = square % 8 + 1;
    let mut y: isize = square / 8 + 1;
    while x < 7 && y < 7 {
      board |= 1 << (x + y * 8);
      x += 1;
      y += 1;
    }

    let mut x = square % 8 - 1;
    let mut y = square / 8 + 1;
    while x > 0 && y < 7 {
      board |= 1 << (x + y * 8);
      x -= 1;
      y += 1;
    }

    let mut x = square % 8 - 1;
    let mut y = square / 8 - 1;
    while x > 0 && y > 0 {
      board |= 1 << (x + y * 8);
      x -= 1;
      y -= 1;
    }

    let mut x = square % 8 + 1;
    let mut y = square / 8 - 1;
    while x < 7 && y > 0 {
      board |= 1 << (x + y * 8);
      x += 1;
      y -= 1;
    }

    board
  }

  let mut moves = [0_u64; 64];

  let mut i = 0;
  while i < 64 {
    moves[i] = gen_bishop_blocker_masks(i);
    i += 1;
  }

  moves
};

const fn slow_const_pext(value: u64, mut mask: u64) -> u64 {
  let mut res = 0;
  let mut bb = 1;
  while mask != 0 {
    if value & mask & (mask.wrapping_neg()) != 0 {
      res |= bb;
    }
    mask &= mask - 1;
    bb += bb;
  }
  res
}

#[allow(clippy::large_const_arrays)]
pub const ROOK_MAGICS: [[u64; 4096]; 64] = {
  let mut magics = [[0_u64; 4096]; 64];

  let mut square = 0;
  while square < 64 {
    let set = ROOK_BLOCKER_MASKS[square];

    let mut subset: u64 = 0;
    loop {
      let pext_index = slow_const_pext(subset, set);

      let mut moves: u64 = 0;

      let mut x = (square % 8) as isize + 1;
      let mut y = (square / 8) as isize;
      while x < 8 {
        let pos = (x + y * 8) as usize;
        set_bit(&mut moves, pos);
        if get_bit(subset, pos) != 0 {
          break;
        }
        x += 1;
      }

      let mut x = (square % 8) as isize;
      let mut y = (square / 8) as isize + 1;
      while y < 8 {
        let pos = (x + y * 8) as usize;
        set_bit(&mut moves, pos);
        if get_bit(subset, pos) != 0 {
          break;
        }
        y += 1;
      }

      let mut x = (square % 8) as isize - 1;
      let mut y = (square / 8) as isize;
      while x >= 0 {
        let pos = (x + y * 8) as usize;
        set_bit(&mut moves, pos);
        if get_bit(subset, pos) != 0 {
          break;
        }
        x -= 1;
      }

      let mut x = (square % 8) as isize;
      let mut y = (square / 8) as isize - 1;
      while y >= 0 {
        let pos = (x + y * 8) as usize;
        set_bit(&mut moves, pos);
        if get_bit(subset, pos) != 0 {
          break;
        }
        y -= 1;
      }

      magics[square][pext_index as usize] = moves;

      subset = subset.wrapping_sub(set) & set;
      if subset == 0 {
        break;
      }
    }

    square += 1;
  }

  magics
};

#[allow(clippy::large_const_arrays)]
pub const BISHOP_MAGICS: [[u64; 4096]; 64] = {
  let mut magics = [[0_u64; 4096]; 64];

  let mut square = 0;
  while square < 64 {
    let set = BISHOP_BLOCKER_MASKS[square];

    let mut subset: u64 = 0;
    loop {
      let pext_index = slow_const_pext(subset, set);

      let mut moves: u64 = 0;

      let mut x = (square % 8) as isize + 1;
      let mut y = (square / 8) as isize + 1;
      while x < 8 && y < 8 {
        let pos = (x + y * 8) as usize;
        set_bit(&mut moves, pos);
        if get_bit(subset, pos) != 0 {
          break;
        }
        x += 1;
        y += 1;
      }

      let mut x = (square % 8) as isize - 1;
      let mut y = (square / 8) as isize + 1;
      while x >= 0 && y < 8 {
        let pos = (x + y * 8) as usize;
        set_bit(&mut moves, pos);
        if get_bit(subset, pos) != 0 {
          break;
        }
        x -= 1;
        y += 1;
      }

      let mut x = (square % 8) as isize - 1;
      let mut y = (square / 8) as isize - 1;
      while x >= 0 && y >= 0 {
        let pos = (x + y * 8) as usize;
        set_bit(&mut moves, pos);
        if get_bit(subset, pos) != 0 {
          break;
        }
        x -= 1;
        y -= 1;
      }

      let mut x = (square % 8) as isize + 1;
      let mut y = (square / 8) as isize - 1;
      while x < 8 && y >= 0 {
        let pos = (x + y * 8) as usize;
        set_bit(&mut moves, pos);
        if get_bit(subset, pos) != 0 {
          break;
        }
        x += 1;
        y -= 1;
      }

      magics[square][pext_index as usize] = moves;

      subset = subset.wrapping_sub(set) & set;
      if subset == 0 {
        break;
      }
    }

    square += 1;
  }

  magics
};

// Includes the from square
// PATH[from][to]
// pub const PATH: [[u64; 64]; 64] = {
//   const fn gen_path(from: usize, to: usize) -> u64 {
//     let from_x = from % 8;
//     let from_y = from / 8;
//     let to_x = to % 8;
//     let to_y = to / 8;

//     let mut b = 0_u64;

//     set_bit(&mut b, from);

//     if from_x == to_x {
//       let mut y = from_y;
//       if from_y > to_y {
//         while y != to_y {
//           set_bit(&mut b, y * 8 + from_x);
//           y -= 1;
//         }
//       } else {
//         while y != to_y {
//           set_bit(&mut b, y * 8 + from_x);
//           y += 1;
//         }
//       }
//     } else if from_y == to_y {
//       let mut x = from_x;
//       if from_x > to_x {
//         while x != to_x {
//           set_bit(&mut b, from_y * 8 + x);
//           x -= 1;
//         }
//       } else {
//         while x != to_x {
//           set_bit(&mut b, from_y * 8 + x);
//           x += 1;
//         }
//       }
//     } else if (from_y as isize - to_y as isize) == (from_x as isize - to_x as isize) {
//       // main diag
//       let mut x = from_x;
//       let mut y = from_y;

//       if to_y > from_y {
//         // north west
//         while y < to_y {
//           set_bit(&mut b, y * 8 + x);

//           x += 1;
//           y += 1;
//         }
//       } else {
//         // south east
//         while y > to_y {
//           set_bit(&mut b, y * 8 + x);

//           x -= 1;
//           y -= 1;
//         }
//       }
//     } else if (from_y as isize - to_y as isize) == -(from_x as isize - to_x as isize) {
//       // other diag
//       let mut x = from_x;
//       let mut y = from_y;

//       if to_y > from_y {
//         // north east
//         while y < to_y {
//           set_bit(&mut b, y * 8 + x);

//           x -= 1;
//           y += 1;
//         }
//       } else {
//         // south west
//         while y > to_y {
//           set_bit(&mut b, y * 8 + x);

//           x += 1;
//           y -= 1;
//         }
//       }
//     }

//     b
//   }

//   let mut moves = [[0_u64; 64]; 64];

//   let mut from = 0;
//   while from < 64 {
//     let mut to = 0;
//     while to < 64 {
//       moves[from][to] = gen_path(from, to);
//       to += 1;
//     }
//     from += 1;
//   }

//   moves
// };

// PINS_H[h_attackers.pext(h_mask)][hv_defenders.pext(h_mask)][king_x];
// pub const PINS_H: [[[u64; 256]; 8]] = {
//
// };

pub struct HashTable {
  // One number for each piece at each square
  // White pawn, black pawn, white knight, black knight...
  pieces: [[u64; 12]; 64],

  // One number to indicate the side to move is black
  black_to_move: u64,

  // Four numbers to indicate the castling rights
  white_king_castle: u64,
  white_queen_castle: u64,
  black_king_castle: u64,
  black_queen_castle: u64,

  // Eight numbers to indicate the file of a valid en passant square
  en_passant: [u64; 8],
}

impl HashTable {
  fn new() -> HashTable {
    let mut pieces = [[0_u64; 12]; 64];

    let mut rng = StdRng::seed_from_u64(572114346);

    let mut position = 0;
    while position < 64 {
      let mut piece = 0;
      while piece < 12 {
        pieces[position][piece] = rng.next_u64();
        piece += 1;
      }
      position += 1;
    }

    let black_to_move = rng.next_u64();
    let white_king_castle = rng.next_u64();
    let white_queen_castle = rng.next_u64();
    let black_king_castle = rng.next_u64();
    let black_queen_castle = rng.next_u64();

    let mut en_passant = [0_u64; 8];

    let mut file = 0;
    while file < 8 {
      en_passant[file] = rng.next_u64();
      file += 1;
    }

    HashTable {
      pieces,
      black_to_move,
      white_king_castle,
      white_queen_castle,
      black_king_castle,
      black_queen_castle,
      en_passant,
    }
  }
}

lazy_static! {
  pub static ref HASH_TABLE: HashTable = HashTable::new();
}

#[derive(Clone)]
pub struct BoardMeta {
  white_king_castle: bool,
  white_queen_castle: bool,
  black_king_castle: bool,
  black_queen_castle: bool,

  en_passant_bitboard: u64,

  halfmove_clock: u8,
  pub hash: u64,
}

/// Chess board
#[derive(Clone)]
pub struct Board {
  /// To get a specific bitboard: `Color + Piece` or just `Color`
  /// White pieces, black pieces, white pawns, black pawns, white knights...
  bitboards: [u64; 14],
  pub white_to_move: bool,
  fullmove_counter: u32,
  pub meta: BoardMeta,
  previous_hashes: Vec<u64>,
  // pub seen_squares: u64,
  // pub checked_squares: u64,
  // pub pinned_hv: u64,
  // pub pinned_diag: u64,
}

// pub const DOUBLE_CHECK: u64 = 0b10101010101010101010101101011001100;

#[derive(Clone, Copy, Debug)]
pub struct ChessMove(u16);

pub const NO_PROMOTION: u16 = 0b00 << 12;
pub const KNIGHT_PROMOTION: u16 = 0b00 << 12;
pub const BISHOP_PROMOTION: u16 = 0b01 << 12;
pub const ROOK_PROMOTION: u16 = 0b10 << 12;
pub const QUEEN_PROMOTION: u16 = 0b11 << 12;

pub const NORMAL_MOVE: u16 = 0b00 << 14;
pub const PROMOTION_MOVE: u16 = 0b01 << 14;
pub const EN_PASSANT_MOVE: u16 = 0b10 << 14;
pub const CASTLING_MOVE: u16 = 0b11 << 14;

impl ChessMove {
  pub fn new(from: usize, to: usize, promotion: u16, move_type: u16) -> Self {
    Self((from as u16) ^ ((to as u16) << 6) ^ promotion ^ move_type)
  }
  pub fn from(&self) -> usize {
    (self.0 & 0b111111_u16) as usize
  }
  pub fn to(&self) -> usize {
    ((self.0 >> 6) & 0b111111_u16) as usize
  }
  fn promotion(&self) -> u16 {
    self.0 & (0b11_u16 << 12)
  }
  fn move_type(&self) -> u16 {
    self.0 & (0b11_u16 << 14)
  }
  pub fn to_fen(&self) -> String {
    let from_bb = 1 << self.from();
    let to_bb = 1 << self.to();

    let mut s = format!("{}{}", bitboard_to_square(from_bb), bitboard_to_square(to_bb));

    if self.move_type() == PROMOTION_MOVE {
      s += match self.promotion() {
        KNIGHT_PROMOTION => "n",
        BISHOP_PROMOTION => "b",
        ROOK_PROMOTION => "r",
        QUEEN_PROMOTION => "q",
        _ => panic!(),
      }
    }

    s
  }
  // Needs a board before the move was made
  pub fn is_capture(&self, board: &Board) -> bool {
    if self.move_type() == EN_PASSANT_MOVE {
      return true;
    }

    let other_color = if board.white_to_move { 1 } else { 0 };
    let other_piece = board.piece_on_with_color(self.to(), other_color);
    // let other_piece = board.piece_on(self.to());

    other_piece != EMPTY_SQUARE
  }

  fn evaluate_relative(&self, board: &Board) -> i32 {
    const fn pv(piece: usize) -> i32 {
      let c = piece % 2;
      let piece = piece - c;
      match piece {
        PAWN => PAWN_VALUE,
        KNIGHT => KNIGHT_VALUE,
        BISHOP => BISHOP_VALUE,
        ROOK => ROOK_VALUE,
        QUEEN => QUEEN_VALUE,
        _ => 0,
      }
    }

    let color = if board.white_to_move { 0 } else { 1 };

    match self.move_type() {
      NORMAL_MOVE => {
        let from_piece = board.piece_on_with_color(self.from(), color);
        let to_piece = board.piece_on_with_color(self.to(), color ^ 1);

        if to_piece == EMPTY_SQUARE {
          0
        } else {
          pv(to_piece) - pv(from_piece)
        }
      }
      PROMOTION_MOVE => {
        let to_piece = board.piece_on_with_color(self.to(), color ^ 1);
        let selected_piece = match self.promotion() {
          KNIGHT_PROMOTION => KNIGHT,
          BISHOP_PROMOTION => BISHOP,
          ROOK_PROMOTION => ROOK,
          QUEEN_PROMOTION => QUEEN,
          _ => panic!(),
        } + color;

        pv(to_piece) + pv(selected_piece)
      }
      EN_PASSANT_MOVE => 10,
      CASTLING_MOVE => 20,
      _ => panic!(),
    }
  }
}

impl Board {
  pub fn print(&self) {
    let mut lines: Vec<String> = vec![];
    for y in 0..8 {
      let mut line: Vec<String> = vec![];
      for x in 0..8 {
        let i = 63 - (y * 8 + x);
        let mut piece = self.piece_on(i);

        let color = piece % 2;
        piece -= color;

        let mut c = match piece {
          PAWN => "P",
          KNIGHT => "N",
          BISHOP => "B",
          ROOK => "R",
          QUEEN => "Q",
          KING => "K",
          _ => " ",
        }
        .to_string();

        if color == BLACK {
          c = c.to_lowercase();
        }

        line.push(c);
      }
      lines.push(line.join(" | "));
    }
    println!("{}", lines.join("\n-----------------------------\n"));
  }

  pub fn from_fen(fen: &str) -> Self {
    Self::from_fen_saved(fen, vec![])
  }

  pub fn from_fen_saved(fen: &str, previous_hashes: Vec<u64>) -> Self {
    let fields: Vec<&str> = fen.split(' ').collect();
    let ranks: Vec<&str> = fields[0].split('/').collect();

    let mut bitboards = [0_u64; 14];
    let mut hash: u64 = 0;

    for (y, s) in ranks.iter().enumerate() {
      let mut x = 0;

      for c in s.chars() {
        if c.is_alphabetic() {
          let color = if c.is_uppercase() { WHITE } else { BLACK };

          let piece = match c.to_uppercase().to_string().as_str() {
            "P" => PAWN,
            "N" => KNIGHT,
            "B" => BISHOP,
            "R" => ROOK,
            "Q" => QUEEN,
            "K" => KING,
            _ => panic!("Incorrect fen"),
          };

          let i = 63 - (y * 8 + x);

          set_bit(&mut bitboards[color + piece], i);
          set_bit(&mut bitboards[color], i);

          hash ^= HASH_TABLE.pieces[i][color + piece - 2];

          x += 1;
        } else {
          let digit = c.to_digit(10).expect("Incorrect fen") as usize;
          x += digit;
        }
      }
    }

    let white_to_move = fields[1] == "w";

    if !white_to_move {
      hash ^= HASH_TABLE.black_to_move;
    }

    let white_king_castle = fields[2].contains('K');
    let white_queen_castle = fields[2].contains('Q');
    let black_king_castle = fields[2].contains('k');
    let black_queen_castle = fields[2].contains('q');

    if white_king_castle {
      hash ^= HASH_TABLE.white_king_castle;
    }
    if white_queen_castle {
      hash ^= HASH_TABLE.white_queen_castle;
    }
    if black_king_castle {
      hash ^= HASH_TABLE.black_king_castle;
    }
    if black_queen_castle {
      hash ^= HASH_TABLE.black_queen_castle;
    }

    let en_passant_bitboard = match fields[3] {
      "-" => 0,
      square => square_to_bitboard(square),
    };

    if en_passant_bitboard != 0 {
      let x = get_lsb(en_passant_bitboard) % 8;
      hash ^= HASH_TABLE.en_passant[x];
    }

    let halfmove_clock = fields[4].parse().unwrap();
    let fullmove_counter = fields[5].parse().unwrap();

    let mut board = Board {
      bitboards,
      white_to_move,
      fullmove_counter,
      meta: BoardMeta {
        white_king_castle,
        white_queen_castle,
        black_king_castle,
        black_queen_castle,
        en_passant_bitboard,
        halfmove_clock,
        hash,
      },
      previous_hashes,
      // seen_squares: 0,
      // checked_squares: 0,
      // pinned_hv: 0,
      // pinned_diag: 0,
    };

    // board.seen_squares = board.gen_seen_squares();
    // board.checked_squares = board.gen_checked_squares();

    board
  }

  // Can be used after .make_move
  // fn gen_pinned_hv(&self) -> u64 {
  //   let mut b = 0;
  //   let color = if self.white_to_move { WHITE } else { BLACK };
  //   let other_color = color ^ 1;

  //   let king_bb = self.bitboards[color + KING];
  //   let king_index = get_lsb(king_bb);
  //   let king_x = king_index % 8;
  //   let king_y = king_index / 8;

  //   // TODO: const mask
  //   let h_mask = RANK_1 << king_y;
  //   let v_mask = match king_x {
  //     0 => FILE_A,
  //     1 => FILE_B,
  //     2 => FILE_C,
  //     3 => FILE_D,
  //     4 => FILE_E,
  //     5 => FILE_F,
  //     6 => FILE_G,
  //     7 => FILE_H,
  //     _ => panic!(),
  //   };
  //   let hv_attackers = self.bitboards[other_color + ROOK] ^ self.bitboards[other_color + QUEEN];
  //   let hv_defenders = self.bitboards[color];

  //   let h_mask_pext_index = PINS_H[h_attackers.pext(h_mask)][hv_defenders.pext(h_mask)][king_x];

  //   b
  // }

  // Should be only used once, when the board is created
  // fn gen_checked_squares(&self) -> u64 {
  //   let color = if self.white_to_move { WHITE } else { BLACK };
  //   let in_check = self.in_check(color);

  //   if in_check {
  //     let other_color = color ^ 1;
  //     let blockers = self.bitboards[WHITE] ^ self.bitboards[BLACK];

  //     let king_bb = self.bitboards[color + KING];
  //     let king_index = get_lsb(king_bb);
  //     let mut to_return = 0;

  //     let mut diag = BISHOP_MAGICS[king_index][blockers.pext(BISHOP_BLOCKER_MASKS[king_index]) as usize]
  //       & (self.bitboards[BISHOP + other_color] | self.bitboards[QUEEN + other_color]);

  //     if diag != 0 {
  //       let attack_from = pop_lsb(&mut diag);
  //       if diag != 0 {
  //         return DOUBLE_CHECK;
  //       }
  //       to_return = PATH[attack_from][king_index];
  //     }

  //     let mut hv: u64 = ROOK_MAGICS[king_index][blockers.pext(ROOK_BLOCKER_MASKS[king_index]) as usize]
  //       & (self.bitboards[ROOK + other_color] | self.bitboards[QUEEN + other_color]);

  //     if hv != 0 {
  //       let attack_from = pop_lsb(&mut hv);
  //       if hv != 0 || to_return != 0 {
  //         return DOUBLE_CHECK;
  //       }
  //       to_return = PATH[attack_from][king_index];
  //     }

  //     let mut knights = KNIGHT_MOVES[king_index] & self.bitboards[KNIGHT + other_color];

  //     if knights != 0 {
  //       let attack_from = pop_lsb(&mut knights);
  //       if knights != 0 || to_return != 0 {
  //         return DOUBLE_CHECK;
  //       }
  //       to_return = PATH[attack_from][king_index];
  //     }

  //     if other_color == BLACK {
  //       let mut p = (north_west(king_bb) | north_east(king_bb)) & self.bitboards[BLACK + PAWN];
  //       if p != 0 {
  //         let attack_from = pop_lsb(&mut p);
  //         if p != 0 || to_return != 0 {
  //           return DOUBLE_CHECK;
  //         }
  //         to_return = PATH[attack_from][king_index];
  //       }
  //     } else {
  //       let mut p = (south_west(king_bb) | south_east(king_bb)) & self.bitboards[WHITE + PAWN];
  //       if p != 0 {
  //         let attack_from = pop_lsb(&mut p);
  //         if p != 0 || to_return != 0 {
  //           return DOUBLE_CHECK;
  //         }
  //         to_return = PATH[attack_from][king_index];
  //       }
  //     }

  //     return to_return;
  //   } else {
  //     !0
  //   }
  // }

  // This should be done incrementaly, except for the first time when creating a new board
  // fn gen_seen_squares(&self) -> u64 {
  //   let mut seen_squares: u64 = 0;
  //   let color = if self.white_to_move { WHITE } else { BLACK };
  //   let blockers = self.bitboards[WHITE] ^ self.bitboards[BLACK];

  //   for i in 0..64 {
  //     let mut piece_type = self.piece_on(i);
  //     let piece_color = piece_type % 2;
  //     if piece_type == EMPTY_SQUARE || piece_color == color {
  //       continue;
  //     }
  //     piece_type -= piece_color;
  //     let seen = match piece_type {
  //       PAWN => {
  //         if !self.white_to_move {
  //           north_west(self.bitboards[WHITE + PAWN]) | north_east(self.bitboards[WHITE + PAWN])
  //         } else {
  //           south_west(self.bitboards[BLACK + PAWN]) | south_east(self.bitboards[BLACK + PAWN])
  //         }
  //       }
  //       KNIGHT => {
  //         let mut b: u64 = 0;
  //         let mut knights = self.bitboards[piece_color + KNIGHT];

  //         while knights != 0 {
  //           let from_index = pop_lsb(&mut knights);

  //           b |= KNIGHT_MOVES[from_index];
  //         }
  //         b
  //       }
  //       BISHOP => {
  //         let mut b: u64 = 0;
  //         let mut bishops = self.bitboards[piece_color + BISHOP];
  //         while bishops != 0 {
  //           let from_index = pop_lsb(&mut bishops);
  //           let pext_index = blockers.pext(BISHOP_BLOCKER_MASKS[from_index]) as usize;

  //           b |= BISHOP_MAGICS[from_index][pext_index];
  //         }
  //         b
  //       }
  //       ROOK => {
  //         let mut b: u64 = 0;
  //         let mut rooks = self.bitboards[piece_color + ROOK];
  //         while rooks != 0 {
  //           let from_index = pop_lsb(&mut rooks);
  //           let pext_index = blockers.pext(ROOK_BLOCKER_MASKS[from_index]) as usize;

  //           b |= ROOK_MAGICS[from_index][pext_index];
  //         }
  //         b
  //       }
  //       QUEEN => {
  //         let mut b: u64 = 0;
  //         let mut queens = self.bitboards[piece_color + QUEEN];
  //         while queens != 0 {
  //           let from_index = pop_lsb(&mut queens);
  //           let pext_rook_index = blockers.pext(ROOK_BLOCKER_MASKS[from_index]) as usize;
  //           let pext_bishop_index = blockers.pext(BISHOP_BLOCKER_MASKS[from_index]) as usize;

  //           b |= ROOK_MAGICS[from_index][pext_rook_index] | BISHOP_MAGICS[from_index][pext_bishop_index];
  //         }
  //         b
  //       }
  //       KING => {
  //         let mut king = self.bitboards[piece_color + KING];
  //         let from_index = pop_lsb(&mut king);

  //         KING_MOVES[from_index]
  //       }
  //       _ => panic!(),
  //     };

  //     seen_squares |= seen;
  //   }
  //   seen_squares
  // }

  // Assuming `color` is the same as the side to move
  fn pawn_moves(&self, color: usize, chess_moves: &mut Vec<ChessMove>) {
    if color == WHITE {
      let empty = !(self.bitboards[WHITE] ^ self.bitboards[BLACK]);
      let black = self.bitboards[BLACK];
      let white_pawns = self.bitboards[WHITE + PAWN];

      let mut white_north_pawns = north(white_pawns & (!RANK_7)) & empty;
      let mut white_double_north_pawns = north(white_north_pawns & RANK_3) & empty;

      let mut white_north_west_captures = north_west(white_pawns & (!RANK_7)) & black;
      let mut white_north_east_captures = north_east(white_pawns & (!RANK_7)) & black;

      let white_promotions_pawns = white_pawns & RANK_7;

      let mut white_north_promotions = north(white_promotions_pawns) & empty;
      let mut white_north_west_promotions = north_west(white_promotions_pawns) & black;
      let mut white_north_east_promotions = north_east(white_promotions_pawns) & black;

      if self.meta.en_passant_bitboard != 0 {
        let mut en_passant_pawns =
          (south_west(self.meta.en_passant_bitboard) | south_east(self.meta.en_passant_bitboard)) & white_pawns;

        let to_index = get_lsb(self.meta.en_passant_bitboard);
        while en_passant_pawns != 0 {
          let from_index = pop_lsb(&mut en_passant_pawns);
          chess_moves.push(ChessMove::new(from_index, to_index, NO_PROMOTION, EN_PASSANT_MOVE));
        }
      }

      while white_north_pawns != 0 {
        let to_index = pop_lsb(&mut white_north_pawns);
        let from_index = to_index - 8;
        chess_moves.push(ChessMove::new(from_index, to_index, NO_PROMOTION, NORMAL_MOVE))
      }

      while white_double_north_pawns != 0 {
        let to_index = pop_lsb(&mut white_double_north_pawns);
        let from_index = to_index - 16;
        chess_moves.push(ChessMove::new(from_index, to_index, NO_PROMOTION, NORMAL_MOVE))
      }

      while white_north_west_captures != 0 {
        let to_index = pop_lsb(&mut white_north_west_captures);
        let from_index = to_index - 9;
        chess_moves.push(ChessMove::new(from_index, to_index, NO_PROMOTION, NORMAL_MOVE))
      }

      while white_north_east_captures != 0 {
        let to_index = pop_lsb(&mut white_north_east_captures);
        let from_index = to_index - 7;
        chess_moves.push(ChessMove::new(from_index, to_index, NO_PROMOTION, NORMAL_MOVE))
      }

      while white_north_promotions != 0 {
        let to_index = pop_lsb(&mut white_north_promotions);
        let from_index = to_index - 8;
        chess_moves.push(ChessMove::new(from_index, to_index, KNIGHT_PROMOTION, PROMOTION_MOVE));
        chess_moves.push(ChessMove::new(from_index, to_index, BISHOP_PROMOTION, PROMOTION_MOVE));
        chess_moves.push(ChessMove::new(from_index, to_index, ROOK_PROMOTION, PROMOTION_MOVE));
        chess_moves.push(ChessMove::new(from_index, to_index, QUEEN_PROMOTION, PROMOTION_MOVE));
      }

      while white_north_west_promotions != 0 {
        let to_index = pop_lsb(&mut white_north_west_promotions);
        let from_index = to_index - 9;
        chess_moves.push(ChessMove::new(from_index, to_index, KNIGHT_PROMOTION, PROMOTION_MOVE));
        chess_moves.push(ChessMove::new(from_index, to_index, BISHOP_PROMOTION, PROMOTION_MOVE));
        chess_moves.push(ChessMove::new(from_index, to_index, ROOK_PROMOTION, PROMOTION_MOVE));
        chess_moves.push(ChessMove::new(from_index, to_index, QUEEN_PROMOTION, PROMOTION_MOVE));
      }

      while white_north_east_promotions != 0 {
        let to_index = pop_lsb(&mut white_north_east_promotions);
        let from_index = to_index - 7;
        chess_moves.push(ChessMove::new(from_index, to_index, KNIGHT_PROMOTION, PROMOTION_MOVE));
        chess_moves.push(ChessMove::new(from_index, to_index, BISHOP_PROMOTION, PROMOTION_MOVE));
        chess_moves.push(ChessMove::new(from_index, to_index, ROOK_PROMOTION, PROMOTION_MOVE));
        chess_moves.push(ChessMove::new(from_index, to_index, QUEEN_PROMOTION, PROMOTION_MOVE));
      }
    } else {
      let empty = !(self.bitboards[WHITE] ^ self.bitboards[BLACK]);
      let white = self.bitboards[WHITE];
      let black_pawns = self.bitboards[BLACK + PAWN];

      let mut black_south_pawns = south(black_pawns & (!RANK_2)) & empty;
      let mut black_double_south_pawns = south(black_south_pawns & RANK_6) & empty;

      // TODO: black_pawns & (!RANK_2)
      let mut black_south_west_captures = south_west(black_pawns & (!RANK_2)) & white;
      let mut black_south_east_captures = south_east(black_pawns & (!RANK_2)) & white;

      let black_promotions_pawns = black_pawns & RANK_2;

      let mut black_south_promotions = south(black_promotions_pawns) & empty;
      let mut black_south_west_promotions = south_west(black_promotions_pawns) & white;
      let mut black_south_east_promotions = south_east(black_promotions_pawns) & white;

      if self.meta.en_passant_bitboard != 0 {
        let mut en_passant_pawns =
          (north_west(self.meta.en_passant_bitboard) | north_east(self.meta.en_passant_bitboard)) & black_pawns;

        let to_index = get_lsb(self.meta.en_passant_bitboard);
        while en_passant_pawns != 0 {
          let from_index = pop_lsb(&mut en_passant_pawns);
          chess_moves.push(ChessMove::new(from_index, to_index, NO_PROMOTION, EN_PASSANT_MOVE));
        }
      }

      while black_south_pawns != 0 {
        let to_index = pop_lsb(&mut black_south_pawns);
        let from_index = to_index + 8;
        chess_moves.push(ChessMove::new(from_index, to_index, NO_PROMOTION, NORMAL_MOVE))
      }

      while black_double_south_pawns != 0 {
        let to_index = pop_lsb(&mut black_double_south_pawns);
        let from_index = to_index + 16;
        chess_moves.push(ChessMove::new(from_index, to_index, NO_PROMOTION, NORMAL_MOVE))
      }

      while black_south_west_captures != 0 {
        let to_index = pop_lsb(&mut black_south_west_captures);
        let from_index = to_index + 7;
        chess_moves.push(ChessMove::new(from_index, to_index, NO_PROMOTION, NORMAL_MOVE))
      }

      while black_south_east_captures != 0 {
        let to_index = pop_lsb(&mut black_south_east_captures);
        let from_index = to_index + 9;
        chess_moves.push(ChessMove::new(from_index, to_index, NO_PROMOTION, NORMAL_MOVE))
      }

      while black_south_promotions != 0 {
        let to_index = pop_lsb(&mut black_south_promotions);
        let from_index = to_index + 8;
        chess_moves.push(ChessMove::new(from_index, to_index, KNIGHT_PROMOTION, PROMOTION_MOVE));
        chess_moves.push(ChessMove::new(from_index, to_index, BISHOP_PROMOTION, PROMOTION_MOVE));
        chess_moves.push(ChessMove::new(from_index, to_index, ROOK_PROMOTION, PROMOTION_MOVE));
        chess_moves.push(ChessMove::new(from_index, to_index, QUEEN_PROMOTION, PROMOTION_MOVE));
      }

      while black_south_west_promotions != 0 {
        let to_index = pop_lsb(&mut black_south_west_promotions);
        let from_index = to_index + 7;
        chess_moves.push(ChessMove::new(from_index, to_index, KNIGHT_PROMOTION, PROMOTION_MOVE));
        chess_moves.push(ChessMove::new(from_index, to_index, BISHOP_PROMOTION, PROMOTION_MOVE));
        chess_moves.push(ChessMove::new(from_index, to_index, ROOK_PROMOTION, PROMOTION_MOVE));
        chess_moves.push(ChessMove::new(from_index, to_index, QUEEN_PROMOTION, PROMOTION_MOVE));
      }

      while black_south_east_promotions != 0 {
        let to_index = pop_lsb(&mut black_south_east_promotions);
        let from_index = to_index + 9;
        chess_moves.push(ChessMove::new(from_index, to_index, KNIGHT_PROMOTION, PROMOTION_MOVE));
        chess_moves.push(ChessMove::new(from_index, to_index, BISHOP_PROMOTION, PROMOTION_MOVE));
        chess_moves.push(ChessMove::new(from_index, to_index, ROOK_PROMOTION, PROMOTION_MOVE));
        chess_moves.push(ChessMove::new(from_index, to_index, QUEEN_PROMOTION, PROMOTION_MOVE));
      }
    }
  }

  fn knight_moves(&self, color: usize, chess_moves: &mut Vec<ChessMove>) {
    let target_squares = if color == 0 {
      !self.bitboards[WHITE]
    } else {
      !self.bitboards[BLACK]
    };

    let mut knights = self.bitboards[color + KNIGHT];

    while knights != 0 {
      let from_index = pop_lsb(&mut knights);

      let mut moves = KNIGHT_MOVES[from_index] & target_squares;

      while moves != 0 {
        let to_index = pop_lsb(&mut moves);

        let chess_move = ChessMove::new(from_index, to_index, NO_PROMOTION, NORMAL_MOVE);
        chess_moves.push(chess_move);
      }
    }
  }

  fn bishop_moves(&self, color: usize, chess_moves: &mut Vec<ChessMove>) {
    let target_squares = if color == 0 {
      !self.bitboards[WHITE]
    } else {
      !self.bitboards[BLACK]
    };
    let blockers = self.bitboards[WHITE] ^ self.bitboards[BLACK];

    let mut bishops = self.bitboards[color + BISHOP];

    while bishops != 0 {
      let from_index = pop_lsb(&mut bishops);
      let pext_index = blockers.pext(BISHOP_BLOCKER_MASKS[from_index]) as usize;

      let mut moves = BISHOP_MAGICS[from_index][pext_index] & target_squares;

      while moves != 0 {
        let to_index = pop_lsb(&mut moves);

        let chess_move = ChessMove::new(from_index, to_index, NO_PROMOTION, NORMAL_MOVE);
        chess_moves.push(chess_move);
      }
    }
  }

  fn rook_moves(&self, color: usize, chess_moves: &mut Vec<ChessMove>) {
    let target_squares = if color == 0 {
      !self.bitboards[WHITE]
    } else {
      !self.bitboards[BLACK]
    };
    let blockers = self.bitboards[WHITE] ^ self.bitboards[BLACK];

    let mut rooks = self.bitboards[color + ROOK];

    while rooks != 0 {
      let from_index = pop_lsb(&mut rooks);
      let pext_index = blockers.pext(ROOK_BLOCKER_MASKS[from_index]) as usize;

      let mut moves = ROOK_MAGICS[from_index][pext_index] & target_squares;

      while moves != 0 {
        let to_index = pop_lsb(&mut moves);

        let chess_move = ChessMove::new(from_index, to_index, NO_PROMOTION, NORMAL_MOVE);
        chess_moves.push(chess_move);
      }
    }
  }

  fn queen_moves(&self, color: usize, chess_moves: &mut Vec<ChessMove>) {
    let target_squares = if color == 0 {
      !self.bitboards[WHITE]
    } else {
      !self.bitboards[BLACK]
    };
    let blockers = self.bitboards[WHITE] ^ self.bitboards[BLACK];

    let mut queens = self.bitboards[color + QUEEN];

    while queens != 0 {
      let from_index = pop_lsb(&mut queens);
      let pext_rook_index = blockers.pext(ROOK_BLOCKER_MASKS[from_index]) as usize;
      let pext_bishop_index = blockers.pext(BISHOP_BLOCKER_MASKS[from_index]) as usize;

      let mut moves = (ROOK_MAGICS[from_index][pext_rook_index] | BISHOP_MAGICS[from_index][pext_bishop_index]) & target_squares;

      while moves != 0 {
        let to_index = pop_lsb(&mut moves);

        let chess_move = ChessMove::new(from_index, to_index, NO_PROMOTION, NORMAL_MOVE);
        chess_moves.push(chess_move);
      }
    }
  }

  // TODO: try using an array with a constant size
  // Assuming we have only one king
  fn king_moves(&self, color: usize, chess_moves: &mut Vec<ChessMove>) {
    let target_squares = if color == 0 {
      !self.bitboards[WHITE]
    } else {
      !self.bitboards[BLACK]
    };

    let king = self.bitboards[color + KING];

    let from_index = get_lsb(king);

    let mut moves = KING_MOVES[from_index] & target_squares;

    while moves != 0 {
      let to_index = pop_lsb(&mut moves);

      let chess_move = ChessMove::new(from_index, to_index, NO_PROMOTION, NORMAL_MOVE);
      chess_moves.push(chess_move);
    }
  }

  pub fn castling_moves(&self, color: usize, chess_moves: &mut Vec<ChessMove>) {
    // TODO: const castling moves with maybe rays, idk

    if color == WHITE {
      if self.meta.white_king_castle
        && self.squares_are_empty(F1 | G1)
        && self.square_is_safe(2, WHITE)
        && self.square_is_safe(1, WHITE)
        && self.square_is_safe(3, WHITE)
      // TODO: repeating
      {
        chess_moves.push(ChessMove::new(3, 1, NO_PROMOTION, CASTLING_MOVE));
      }
      if self.meta.white_queen_castle
        && self.squares_are_empty(B1 | C1 | D1)
        && self.square_is_safe(4, WHITE)
        && self.square_is_safe(5, WHITE)
        && self.square_is_safe(3, WHITE)
      // TODO: repeating
      {
        chess_moves.push(ChessMove::new(3, 5, NO_PROMOTION, CASTLING_MOVE));
      }
    } else {
      if self.meta.black_king_castle
        && self.squares_are_empty(F8 | G8)
        && self.square_is_safe(58, BLACK)
        && self.square_is_safe(57, BLACK)
        && self.square_is_safe(59, BLACK)
      {
        chess_moves.push(ChessMove::new(59, 57, NO_PROMOTION, CASTLING_MOVE));
      }
      if self.meta.black_queen_castle
        && self.squares_are_empty(B8 | C8 | D8)
        && self.square_is_safe(60, BLACK)
        && self.square_is_safe(61, BLACK)
        && self.square_is_safe(59, BLACK)
      {
        chess_moves.push(ChessMove::new(59, 61, NO_PROMOTION, CASTLING_MOVE));
      }
    }
  }

  fn squares_are_empty(&self, bitboard: u64) -> bool {
    let all = self.bitboards[WHITE] ^ self.bitboards[BLACK];

    (all & bitboard) == 0
  }

  pub fn pseudo_legal_moves(&self) -> Vec<ChessMove> {
    if self.meta.halfmove_clock >= 100 || self.previous_hashes.iter().filter(|h| **h == self.meta.hash).count() >= 2 {
      return vec![];
    }

    let color: usize = if self.white_to_move { 0 } else { 1 };

    // TODO: try changing to const. also try changing capacity
    let mut chess_moves: Vec<ChessMove> = Vec::with_capacity(50);

    self.queen_moves(color, &mut chess_moves);
    self.castling_moves(color, &mut chess_moves);
    self.knight_moves(color, &mut chess_moves);
    self.rook_moves(color, &mut chess_moves);
    self.bishop_moves(color, &mut chess_moves);
    self.pawn_moves(color, &mut chess_moves);
    self.king_moves(color, &mut chess_moves);

    chess_moves.sort_by_cached_key(|m| -m.evaluate_relative(self));

    chess_moves
  }

  pub fn make_move(&mut self, chess_move: &ChessMove) {
    self.previous_hashes.push(self.meta.hash);

    let color: usize = if self.white_to_move { 0 } else { 1 };

    let from = chess_move.from();
    let to = chess_move.to();

    let previous_white_king_castle = self.meta.white_king_castle;
    let previous_white_queen_castle = self.meta.white_queen_castle;
    let previous_black_king_castle = self.meta.black_king_castle;
    let previous_black_queen_castle = self.meta.black_queen_castle;

    match chess_move.move_type() {
      NORMAL_MOVE => {
        let our_piece_index = self.piece_on_with_color(from, color);
        let to_bb = 1 << to;
        let from_bb = 1 << from;
        let move_bb = from_bb | to_bb;
        self.bitboards[our_piece_index] ^= move_bb;
        self.bitboards[color] ^= move_bb;

        let enemy_piece_index = self.piece_on_with_color(to, color ^ 1);

        if self.meta.en_passant_bitboard != 0 {
          let x = get_lsb(self.meta.en_passant_bitboard) % 8;
          self.meta.hash ^= HASH_TABLE.en_passant[x];
        }
        if (our_piece_index == WHITE + PAWN && to - from == 16) || (our_piece_index == BLACK + PAWN && from - to == 16) {
          self.meta.en_passant_bitboard = 1 << ((to + from) / 2);
          let x = ((to + from) / 2) % 8;
          self.meta.hash ^= HASH_TABLE.en_passant[x];
        } else {
          self.meta.en_passant_bitboard = 0;
        }

        self.meta.hash ^= HASH_TABLE.pieces[from][our_piece_index - 2];
        self.meta.hash ^= HASH_TABLE.pieces[to][our_piece_index - 2];
        if enemy_piece_index != EMPTY_SQUARE {
          self.meta.hash ^= HASH_TABLE.pieces[to][enemy_piece_index - 2];

          self.bitboards[enemy_piece_index] ^= to_bb;
          self.bitboards[color ^ 1] ^= to_bb;
          self.meta.halfmove_clock = 0;
        } else {
          self.meta.halfmove_clock += 1;
        }

        if from_bb == E1 {
          self.meta.white_king_castle = false;
          self.meta.white_queen_castle = false;
        } else if from_bb == E8 {
          self.meta.black_king_castle = false;
          self.meta.black_queen_castle = false;
        }
        if to_bb == H1 || from_bb == H1 {
          self.meta.white_king_castle = false;
        } else if to_bb == A1 || from_bb == A1 {
          self.meta.white_queen_castle = false;
        } else if to_bb == H8 || from_bb == H8 {
          self.meta.black_king_castle = false;
        } else if to_bb == A8 || from_bb == A8 {
          self.meta.black_queen_castle = false;
        }
      }
      PROMOTION_MOVE => {
        // TODO: optimize - we know its a pawn
        let from_bb = 1 << from;
        let to_bb = 1 << to;
        let move_bb = from_bb | to_bb;
        self.bitboards[PAWN + color] ^= from_bb;
        self.bitboards[color] ^= move_bb;

        let enemy_piece_index = self.piece_on_with_color(to, color ^ 1);

        let promotion_piece_index = match chess_move.promotion() {
          KNIGHT_PROMOTION => KNIGHT,
          BISHOP_PROMOTION => BISHOP,
          ROOK_PROMOTION => ROOK,
          QUEEN_PROMOTION => QUEEN,
          _ => panic!(),
        };

        self.meta.hash ^= HASH_TABLE.pieces[from][PAWN + color - 2];

        if enemy_piece_index != EMPTY_SQUARE {
          self.meta.hash ^= HASH_TABLE.pieces[to][enemy_piece_index - 2];
          self.bitboards[enemy_piece_index] ^= to_bb;
          self.bitboards[color ^ 1] ^= to_bb;
        }

        self.bitboards[promotion_piece_index + color] ^= to_bb;
        self.meta.hash ^= HASH_TABLE.pieces[to][promotion_piece_index + color - 2];

        if to_bb == H1 {
          self.meta.white_king_castle = false;
        } else if to_bb == A1 {
          self.meta.white_queen_castle = false;
        } else if to_bb == H8 {
          self.meta.black_king_castle = false;
        } else if to_bb == A8 {
          self.meta.black_queen_castle = false;
        }
        self.meta.en_passant_bitboard = 0;
        self.meta.halfmove_clock = 0;
      }
      EN_PASSANT_MOVE => {
        // TODO: optimize - we know its a pawn
        let from_bb = 1 << from;
        let to_bb = 1 << to;
        let move_bb = from_bb | to_bb;

        let our_piece_index = self.piece_on_with_color(from, color);
        self.bitboards[our_piece_index] ^= move_bb;
        self.bitboards[color] ^= move_bb;

        self.meta.hash ^= HASH_TABLE.pieces[from][our_piece_index - 2];
        self.meta.hash ^= HASH_TABLE.pieces[to][our_piece_index - 2];

        if color == WHITE {
          let to_remove_bb = 1 << (to - 8);
          self.bitboards[BLACK + PAWN] ^= to_remove_bb;
          self.bitboards[BLACK] ^= to_remove_bb;
          self.meta.hash ^= HASH_TABLE.pieces[to - 8][BLACK + PAWN - 2];
        } else {
          let to_remove_bb = 1 << (to + 8);
          self.bitboards[WHITE + PAWN] ^= to_remove_bb;
          self.bitboards[WHITE] ^= to_remove_bb;
          self.meta.hash ^= HASH_TABLE.pieces[to + 8][WHITE + PAWN - 2];
        }

        self.meta.en_passant_bitboard = 0;
        self.meta.halfmove_clock = 0;
      }
      CASTLING_MOVE => {
        // TODO: | -> ^
        if to == 1 {
          // White king castle
          self.bitboards[WHITE + KING] = G1;
          self.bitboards[WHITE + ROOK] ^= F1 | H1;
          self.bitboards[WHITE] ^= E1 | F1 | G1 | H1;

          self.meta.white_king_castle = false;
          self.meta.white_queen_castle = false;

          self.meta.hash ^= HASH_TABLE.pieces[3][WHITE + KING - 2];
          self.meta.hash ^= HASH_TABLE.pieces[1][WHITE + KING - 2];
          self.meta.hash ^= HASH_TABLE.pieces[0][WHITE + ROOK - 2];
          self.meta.hash ^= HASH_TABLE.pieces[2][WHITE + ROOK - 2];
        } else if to == 5 {
          // White queen castle
          self.bitboards[WHITE + KING] = C1;
          self.bitboards[WHITE + ROOK] ^= A1 | D1;
          self.bitboards[WHITE] ^= A1 | C1 | D1 | E1;

          self.meta.white_king_castle = false;
          self.meta.white_queen_castle = false;

          self.meta.hash ^= HASH_TABLE.pieces[3][WHITE + KING - 2];
          self.meta.hash ^= HASH_TABLE.pieces[5][WHITE + KING - 2];
          self.meta.hash ^= HASH_TABLE.pieces[7][WHITE + ROOK - 2];
          self.meta.hash ^= HASH_TABLE.pieces[4][WHITE + ROOK - 2];
        } else if to == 57 {
          // Black king castle
          self.bitboards[BLACK + KING] = G8;
          self.bitboards[BLACK + ROOK] ^= F8 | H8;
          self.bitboards[BLACK] ^= E8 | F8 | G8 | H8;

          self.meta.black_king_castle = false;
          self.meta.black_queen_castle = false;

          self.meta.hash ^= HASH_TABLE.pieces[59][BLACK + KING - 2];
          self.meta.hash ^= HASH_TABLE.pieces[57][BLACK + KING - 2];
          self.meta.hash ^= HASH_TABLE.pieces[56][BLACK + ROOK - 2];
          self.meta.hash ^= HASH_TABLE.pieces[58][BLACK + ROOK - 2];
        } else if to == 61 {
          // Black queen castle
          self.bitboards[BLACK + KING] = C8;
          self.bitboards[BLACK + ROOK] ^= A8 | D8;
          self.bitboards[BLACK] ^= A8 | C8 | D8 | E8;

          self.meta.black_king_castle = false;
          self.meta.black_queen_castle = false;

          self.meta.hash ^= HASH_TABLE.pieces[59][BLACK + KING - 2];
          self.meta.hash ^= HASH_TABLE.pieces[61][BLACK + KING - 2];
          self.meta.hash ^= HASH_TABLE.pieces[63][BLACK + ROOK - 2];
          self.meta.hash ^= HASH_TABLE.pieces[60][BLACK + ROOK - 2];
        } else {
          panic!();
        }

        self.meta.en_passant_bitboard = 0;
        self.meta.halfmove_clock += 1;
      }
      _ => panic!(),
    }

    // If any of the castling rights changed, update the hash
    if previous_white_king_castle != self.meta.white_king_castle {
      self.meta.hash ^= HASH_TABLE.white_king_castle;
    }
    if previous_white_queen_castle != self.meta.white_queen_castle {
      self.meta.hash ^= HASH_TABLE.white_queen_castle;
    }
    if previous_black_king_castle != self.meta.black_king_castle {
      self.meta.hash ^= HASH_TABLE.black_king_castle;
    }
    if previous_black_queen_castle != self.meta.black_queen_castle {
      self.meta.hash ^= HASH_TABLE.black_queen_castle;
    }

    if color == BLACK {
      self.fullmove_counter += 1;
    }
    self.meta.hash ^= HASH_TABLE.black_to_move;

    self.white_to_move = !self.white_to_move;
  }

  // Ignores en passant
  pub fn square_is_safe(&self, index: usize, defending_color: usize) -> bool {
    let attacking_color = defending_color ^ 1;
    let blockers = self.bitboards[WHITE] ^ self.bitboards[BLACK];

    if BISHOP_MAGICS[index][blockers.pext(BISHOP_BLOCKER_MASKS[index]) as usize]
      & (self.bitboards[BISHOP + attacking_color] | self.bitboards[QUEEN + attacking_color])
      != 0
    {
      return false;
    }

    if ROOK_MAGICS[index][blockers.pext(ROOK_BLOCKER_MASKS[index]) as usize]
      & (self.bitboards[ROOK + attacking_color] | self.bitboards[QUEEN + attacking_color])
      != 0
    {
      return false;
    }

    if KNIGHT_MOVES[index] & self.bitboards[KNIGHT + attacking_color] != 0 {
      return false;
    }

    if KING_MOVES[index] & self.bitboards[KING + attacking_color] != 0 {
      return false;
    }

    let square_bb: u64 = 1 << index;

    if attacking_color == BLACK
      && (south_west(self.bitboards[PAWN + BLACK]) | south_east(self.bitboards[PAWN + BLACK])) & square_bb != 0
    {
      return false;
    }

    if attacking_color == WHITE
      && (north_west(self.bitboards[PAWN + WHITE]) | north_east(self.bitboards[PAWN + WHITE])) & square_bb != 0
    {
      return false;
    }

    true
  }

  pub fn piece_on_with_color(&self, index: usize, color: usize) -> usize {
    let bitboard: u64 = 1 << index;

    for i in 0..6 {
      let p = 2 + i * 2 + color;
      if self.bitboards[p] & bitboard != 0 {
        return p;
      }
    }

    EMPTY_SQUARE
  }

  fn piece_on(&self, index: usize) -> usize {
    let bitboard: u64 = 1 << index;

    for i in 2..14 {
      if self.bitboards[i] & bitboard != 0 {
        return i;
      }
    }

    EMPTY_SQUARE
  }

  pub fn in_check(&self, color: usize) -> bool {
    let king_index = get_lsb(self.bitboards[color + KING]);

    // TODO: fn without an index but just bitboard
    !self.square_is_safe(king_index, color)
  }

  pub fn legal_moves(&self) -> Vec<ChessMove> {
    let color: usize = if self.white_to_move { 0 } else { 1 };

    self
      .pseudo_legal_moves()
      .into_iter()
      .filter(|chess_move| {
        let mut new_board = self.clone();
        new_board.make_move(chess_move);
        !new_board.in_check(color)
      })
      .collect()
  }

  // Returns a fen string of the current position. Example fen: rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
  pub fn to_fen(&self) -> String {
    let mut piece_placement: Vec<String> = vec![];

    for y in 0..8 {
      let mut rank = "".to_string();
      let mut empty_in_a_row = 0;

      for x in 0..8 {
        let i = 63 - (y * 8 + x);
        let mut piece = self.piece_on(i);

        if piece == EMPTY_SQUARE {
          empty_in_a_row += 1;
          continue;
        }

        if empty_in_a_row != 0 {
          rank += &empty_in_a_row.to_string();
          empty_in_a_row = 0;
        }

        let color = piece % 2;
        piece -= color;

        let mut piece_char = match piece {
          PAWN => "P",
          KNIGHT => "N",
          BISHOP => "B",
          ROOK => "R",
          QUEEN => "Q",
          KING => "K",
          _ => panic!("This shouldn't panic"),
        }
        .to_string();

        if color == BLACK {
          piece_char = piece_char.to_lowercase();
        }

        rank += &piece_char;
      }

      if empty_in_a_row != 0 {
        rank += &empty_in_a_row.to_string();
      }

      piece_placement.push(rank);
    }

    let piece_placement = piece_placement.join("/");

    let side_to_move = if self.white_to_move { "w" } else { "b" };

    let mut castling_ability = "".to_string();

    if self.meta.white_king_castle {
      castling_ability += "K";
    }
    if self.meta.white_queen_castle {
      castling_ability += "Q";
    }
    if self.meta.black_king_castle {
      castling_ability += "k";
    }
    if self.meta.black_queen_castle {
      castling_ability += "q";
    }
    if castling_ability.is_empty() {
      castling_ability += "-";
    }

    let mut en_passant_target_square = "-".to_string();
    if self.meta.en_passant_bitboard != 0 {
      en_passant_target_square = bitboard_to_square(self.meta.en_passant_bitboard).to_string();
    }

    let halfmove_clock = self.meta.halfmove_clock.to_string();

    let fullmove_counter = self.fullmove_counter.to_string();

    format!("{piece_placement} {side_to_move} {castling_ability} {en_passant_target_square} {halfmove_clock} {fullmove_counter}")
  }

  pub fn evaluate(&self) -> i32 {
    // Assuming we aren't checked
    if self.meta.halfmove_clock >= 100 || self.previous_hashes.iter().filter(|h| **h == self.meta.hash).count() >= 2 {
      return 0;
    }

    let mut eval = 0;

    for index in 0..64 {
      let bb = 1 << index;
      let i = 63 - index;

      eval += (self.bitboards[WHITE + PAWN] & bb).count_ones() as i32 * (PAWN_VALUE + WHITE_PAWN_EVAL[i]);
      eval -= (self.bitboards[BLACK + PAWN] & bb).count_ones() as i32 * (PAWN_VALUE + BLACK_PAWN_EVAL[i]);

      eval += (self.bitboards[WHITE + KNIGHT] & bb).count_ones() as i32 * (KNIGHT_VALUE + WHITE_KNIGHT_EVAL[i]);
      eval -= (self.bitboards[BLACK + KNIGHT] & bb).count_ones() as i32 * (KNIGHT_VALUE + BLACK_KNIGHT_EVAL[i]);

      eval += (self.bitboards[WHITE + BISHOP] & bb).count_ones() as i32 * (BISHOP_VALUE + WHITE_BISHOP_EVAL[i]);
      eval -= (self.bitboards[BLACK + BISHOP] & bb).count_ones() as i32 * (BISHOP_VALUE + BLACK_BISHOP_EVAL[i]);

      eval += (self.bitboards[WHITE + ROOK] & bb).count_ones() as i32 * (ROOK_VALUE + WHITE_ROOK_EVAL[i]);
      eval -= (self.bitboards[BLACK + ROOK] & bb).count_ones() as i32 * (ROOK_VALUE + BLACK_ROOK_EVAL[i]);

      eval += (self.bitboards[WHITE + QUEEN] & bb).count_ones() as i32 * (QUEEN_VALUE + WHITE_QUEEN_EVAL[i]);
      eval -= (self.bitboards[BLACK + QUEEN] & bb).count_ones() as i32 * (QUEEN_VALUE + BLACK_QUEEN_EVAL[i]);

      eval += (self.bitboards[WHITE + KING] & bb).count_ones() as i32 * WHITE_KING_EVAL[i];
      eval -= (self.bitboards[BLACK + KING] & bb).count_ones() as i32 * BLACK_KING_EVAL[i];
    }

    if self.meta.white_king_castle {
      eval += POSSIBLE_CASTLING_VALUE;
    }
    if self.meta.white_queen_castle {
      eval += POSSIBLE_CASTLING_VALUE;
    }
    if self.meta.black_king_castle {
      eval -= POSSIBLE_CASTLING_VALUE;
    }
    if self.meta.black_queen_castle {
      eval -= POSSIBLE_CASTLING_VALUE;
    }

    eval
  }

  pub fn evaluate_relative(&self) -> i32 {
    if self.white_to_move {
      self.evaluate()
    } else {
      -self.evaluate()
    }
  }
}

impl Default for Board {
  fn default() -> Board {
    Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
  }
}

#[cfg(test)]
mod tests {
  use crate::bitboard::{bitboard_to_square, square_to_bitboard, Board, ChessMove, NORMAL_MOVE, NO_PROMOTION};

  #[test]
  fn fen_test() {
    // https://www.chessprogramming.org/Forsyth-Edwards_Notation

    assert_eq!(
      "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
      Board::default().to_fen()
    );

    assert_eq!(
      "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
      Board::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1").to_fen()
    );

    assert_eq!(
      "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2",
      Board::from_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2").to_fen()
    );

    assert_eq!(
      "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2",
      Board::from_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2").to_fen()
    );

    assert_eq!(
      "r1bqk2r/2ppbppp/p1n2n2/1p2p3/4P3/1B3N2/PPPP1PPP/RNBQR1K1 b kq - 0 1",
      Board::from_fen("r1bqk2r/2ppbppp/p1n2n2/1p2p3/4P3/1B3N2/PPPP1PPP/RNBQR1K1 b kq - 0 1").to_fen()
    );

    assert_eq!(
      "rnbqkbnr/ppppp1pp/8/8/4PpP1/7N/PPPP1P1P/RNBQKB1R b KQkq e3 0 1",
      Board::from_fen("rnbqkbnr/ppppp1pp/8/8/4PpP1/7N/PPPP1P1P/RNBQKB1R b KQkq e3 0 1").to_fen()
    );
    assert_eq!("a1", bitboard_to_square(square_to_bitboard("a1")));
    assert_eq!("a3", bitboard_to_square(square_to_bitboard("a3")));
    assert_eq!("a8", bitboard_to_square(square_to_bitboard("a8")));
    assert_eq!("c1", bitboard_to_square(square_to_bitboard("c1")));
    assert_eq!("c3", bitboard_to_square(square_to_bitboard("c3")));
    assert_eq!("c8", bitboard_to_square(square_to_bitboard("c8")));
    assert_eq!("h1", bitboard_to_square(square_to_bitboard("h1")));
    assert_eq!("h3", bitboard_to_square(square_to_bitboard("h3")));
    assert_eq!("h8", bitboard_to_square(square_to_bitboard("h8")));
  }

  #[test]
  fn move_generation_test() {
    let board = Board::from_fen("r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1");
    let mut expected_moves = vec![
      "h2g1", "h2g3", "h2f4", "h2e5", "h2d6", "h2c7", "h2b8", "a1b1", "a1c1", "a1d1", "a1a2", "a1a3", "a1a4", "a1a5", "a1a6",
      "a1a7", "a1a8", "h1f1", "h1g1", "e1d1", "e1f1", "e1d2", "e1e2", "e1f2", "e1g1", "e1c1",
    ];
    let mut generated_moves: Vec<String> = board.legal_moves().iter().map(|m| m.to_fen()).collect();

    expected_moves.sort();
    generated_moves.sort();

    assert_eq!(expected_moves, generated_moves);

    // --------------

    let board = Board::from_fen("k4n2/6P1/8/2pP4/8/8/8/4K2R w K c6 0 1");
    let mut expected_moves = vec![
      "d5d6", "g7f8q", "g7f8r", "g7f8b", "g7f8n", "g7g8q", "g7g8r", "g7g8b", "g7g8n", "d5c6", "h1f1", "h1g1", "h1h2", "h1h3",
      "h1h4", "h1h5", "h1h6", "h1h7", "h1h8", "e1d1", "e1f1", "e1d2", "e1e2", "e1f2", "e1g1",
    ];
    let mut generated_moves: Vec<String> = board.legal_moves().iter().map(|m| m.to_fen()).collect();

    expected_moves.sort();
    generated_moves.sort();

    assert_eq!(expected_moves, generated_moves);

    // --------------

    let board = Board::default();
    let mut expected_moves = vec![
      "a2a3", "b2b3", "c2c3", "d2d3", "e2e3", "f2f3", "g2g3", "h2h3", "a2a4", "b2b4", "c2c4", "d2d4", "e2e4", "f2f4", "g2g4",
      "h2h4", "b1a3", "b1c3", "g1f3", "g1h3",
    ];
    let mut generated_moves: Vec<String> = board.legal_moves().iter().map(|m| m.to_fen()).collect();

    expected_moves.sort();
    generated_moves.sort();

    assert_eq!(expected_moves, generated_moves);

    // --------------

    let board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
    let mut expected_moves = vec![
      "a2a3", "b2b3", "g2g3", "d5d6", "a2a4", "g2g4", "g2h3", "d5e6", "c3b1", "c3d1", "c3a4", "c3b5", "e5d3", "e5c4", "e5g4",
      "e5c6", "e5g6", "e5d7", "e5f7", "d2c1", "d2e3", "d2f4", "d2g5", "d2h6", "e2d1", "e2f1", "e2d3", "e2c4", "e2b5", "e2a6",
      "a1b1", "a1c1", "a1d1", "h1f1", "h1g1", "f3d3", "f3e3", "f3g3", "f3h3", "f3f4", "f3g4", "f3f5", "f3h5", "f3f6", "e1d1",
      "e1f1", "e1g1", "e1c1",
    ];
    let mut generated_moves: Vec<String> = board.legal_moves().iter().map(|m| m.to_fen()).collect();

    expected_moves.sort();
    generated_moves.sort();

    assert_eq!(expected_moves, generated_moves);
  }

  #[test]
  fn perft_test() {
    // https://www.chessprogramming.org/Perft_Results
    fn test_fen<const N: usize>(fen: &str, depth_nodes: [u64; N]) {
      let mut board = Board::from_fen(fen);

      fn perft(depth: usize, board: &mut Board) -> u64 {
        // https://www.chessprogramming.org/Perft

        let mut nodes = 0;

        if depth == 0 {
          return 1;
        }

        let color = if board.white_to_move { 0 } else { 1 };

        for chess_move in board.pseudo_legal_moves() {
          let mut new_board = board.clone();
          new_board.make_move(&chess_move);
          if !new_board.in_check(color) {
            nodes += perft(depth - 1, &mut new_board);
          }
        }

        nodes
      }

      for (i, nodes) in depth_nodes.iter().enumerate() {
        assert_eq!(perft(i, &mut board), *nodes);
      }
    }

    // Position 1
    test_fen::<7>(
      "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
      [1, 20, 400, 8_902, 197_281, 4_865_609, 119_060_324],
    );

    // Position 2
    test_fen::<6>(
      "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
      [1, 48, 2_039, 97_862, 4_085_603, 193_690_690],
    );

    // Position 3
    test_fen::<7>(
      "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
      [1, 14, 191, 2_812, 43_238, 674_624, 11_030_083],
    );

    // Position 4
    test_fen::<6>(
      "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
      [1, 6, 264, 9_467, 422_333, 15_833_292],
    );

    // Position 4 mirrored
    test_fen::<6>(
      "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1",
      [1, 6, 264, 9_467, 422_333, 15_833_292],
    );

    // Position 5
    test_fen::<6>(
      "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
      [1, 44, 1_486, 62_379, 2_103_487, 89_941_194],
    );

    // Position 6
    test_fen::<6>(
      "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
      [1, 46, 2_079, 89_890, 3_894_594, 164_075_551],
    );
  }

  #[test]
  fn hash_test() {
    fn test_hashing(fen: &str) {
      let board = Board::from_fen(fen);
      let expected_hash = board.meta.hash;

      for chess_move in board.pseudo_legal_moves() {
        let mut new_board = board.clone();
        new_board.make_move(&chess_move);
        assert_ne!(new_board.meta.hash, expected_hash);
      }
    }

    test_hashing("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    test_hashing("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
    test_hashing("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    test_hashing("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1");
    test_hashing("r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1");
    test_hashing("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8");
    test_hashing("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10");
    test_hashing("r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1");

    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    board.make_move(&ChessMove::new(1, 18, NO_PROMOTION, NORMAL_MOVE));
    assert_eq!(
      board.meta.hash,
      Board::from_fen("rnbqkbnr/pppppppp/8/8/8/5N2/PPPPPPPP/RNBQKB1R b KQkq - 0 1")
        .meta
        .hash
    );

    board.make_move(&ChessMove::new(52, 44, NO_PROMOTION, NORMAL_MOVE));
    assert_eq!(
      board.meta.hash,
      Board::from_fen("rnbqkbnr/ppp1pppp/3p4/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 1")
        .meta
        .hash
    );

    board.make_move(&ChessMove::new(0, 1, NO_PROMOTION, NORMAL_MOVE));
    assert_eq!(
      board.meta.hash,
      Board::from_fen("rnbqkbnr/ppp1pppp/3p4/8/8/5N2/PPPPPPPP/RNBQKBR1 b Qkq - 0 1")
        .meta
        .hash
    );

    board.make_move(&ChessMove::new(54, 38, NO_PROMOTION, NORMAL_MOVE));
    assert_eq!(
      board.meta.hash,
      Board::from_fen("rnbqkbnr/p1p1pppp/3p4/1p6/8/5N2/PPPPPPPP/RNBQKBR1 w Qkq b6 0 1")
        .meta
        .hash
    );

    board.make_move(&ChessMove::new(13, 29, NO_PROMOTION, NORMAL_MOVE));
    assert_eq!(
      board.meta.hash,
      Board::from_fen("rnbqkbnr/p1p1pppp/3p4/1p6/2P5/5N2/PP1PPPPP/RNBQKBR1 b Qkq c3 0 1")
        .meta
        .hash
    );

    board.make_move(&ChessMove::new(59, 52, NO_PROMOTION, NORMAL_MOVE));
    assert_eq!(
      board.meta.hash,
      Board::from_fen("rnbq1bnr/p1pkpppp/3p4/1p6/2P5/5N2/PP1PPPPP/RNBQKBR1 w Q - 0 1")
        .meta
        .hash
    );

    // board.make_move(&Move::capture(
    //   square_to_index("c4"),
    //   square_to_index("b5"),
    //   -crate::board::PAWN,
    // ));
    // assert_eq!(
    //   board.meta.hash,
    //   Board::from_fen("rnbq1bnr/p1pkpppp/3p4/1P6/8/5N2/PP1PPPPP/RNBQKBR1 b Q - 0 1")
    //     .meta
    //     .hash
    // );

    // board.make_move(&Move::pawn_push(square_to_index("g7"), square_to_index("g6")));
    // assert_eq!(
    //   board.meta.hash,
    //   Board::from_fen("rnbq1bnr/p1pkpp1p/3p2p1/1P6/8/5N2/PP1PPPPP/RNBQKBR1 w Q - 0 1")
    //     .meta
    //     .hash
    // );

    // board.make_move(&Move::double_pawn_push(square_to_index("h2"), square_to_index("h4")));
    // assert_eq!(
    //   board.meta.hash,
    //   Board::from_fen("rnbq1bnr/p1pkpp1p/3p2p1/1P6/7P/5N2/PP1PPPP1/RNBQKBR1 b Q h3 0 1")
    //     .meta
    //     .hash
    // );

    // board.make_move(&Move::double_pawn_push(square_to_index("c7"), square_to_index("c5")));
    // assert_eq!(
    //   board.meta.hash,
    //   Board::from_fen("rnbq1bnr/p2kpp1p/3p2p1/1Pp5/7P/5N2/PP1PPPP1/RNBQKBR1 w Q c6 0 1")
    //     .meta
    //     .hash
    // );

    // board.make_move(&Move::en_passant(
    //   square_to_index("b5"),
    //   square_to_index("c6"),
    //   square_to_index("c5"),
    //   -crate::board::PAWN,
    // ));
    // assert_eq!(
    //   board.meta.hash,
    //   Board::from_fen("rnbq1bnr/p2kpp1p/2Pp2p1/8/7P/5N2/PP1PPPP1/RNBQKBR1 b Q - 0 1")
    //     .meta
    //     .hash
    // );
  }
}
