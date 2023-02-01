#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod board;

// TODOs:
// https://www.chessprogramming.org/Perft

use itertools::Itertools;
use std::{collections::HashMap, time::Instant};

type BoardHash = (u64, u64, u64, u64, u64, u64, u64, u64);

const PAWN: i8 = 1;
const KNIGHT: i8 = 2;
const BISHOP: i8 = 3;
const ROOK: i8 = 4;
const QUEEN: i8 = 5;
const KING: i8 = 6;
const EMPTY: i8 = 7;
const OUTSIDE: i8 = 0;

const NOTHING: i8 = 0;
const CHECK: i8 = 1;
const CHECKMATE: i8 = 2;
const STALEMATE: i8 = 2;

const MAX: i32 = 99999;
const MIN: i32 = -99999;

// convert from 8x8 to 12x10
const BOARD_OFFSET: [i8; 64] = [
  21, 22, 23, 24, 25, 26, 27, 28, //
  31, 32, 33, 34, 35, 36, 37, 38, //
  41, 42, 43, 44, 45, 46, 47, 48, //
  51, 52, 53, 54, 55, 56, 57, 58, //
  61, 62, 63, 64, 65, 66, 67, 68, //
  71, 72, 73, 74, 75, 76, 77, 78, //
  81, 82, 83, 84, 85, 86, 87, 88, //
  91, 92, 93, 94, 95, 96, 97, 98, //
];

// TODO
const BOARD_VALUES: [i8; 64] = [
  21, 22, 23, 24, 25, 26, 27, 28, //
  31, 32, 33, 34, 35, 36, 37, 38, //
  41, 42, 43, 44, 45, 46, 47, 48, //
  51, 52, 53, 54, 55, 56, 57, 58, //
  61, 62, 63, 64, 65, 66, 67, 68, //
  71, 72, 73, 74, 75, 76, 77, 78, //
  81, 82, 83, 84, 85, 86, 87, 88, //
  91, 92, 93, 94, 95, 96, 97, 98, //
];

#[derive(Clone, Copy, Debug)]
struct PieceMove {
  from: usize,
  to: usize,
  capture: bool,
  replace_piece_1: Option<i8>,
  replace_piece_index_1: Option<usize>,
  replace_piece_2: Option<i8>,
  replace_piece_index_2: Option<usize>,
}

impl ToString for PieceMove {
  fn to_string(&self) -> String {
    format!(
      "{} -> {}, {}",
      Board::index_to_square(self.from),
      Board::index_to_square(self.to),
      self.capture
    )
  }
}

impl PieceMove {
  fn new(from: usize, to: usize, capture: bool) -> PieceMove {
    PieceMove {
      from,
      to,
      capture,
      replace_piece_1: None,
      replace_piece_index_1: None,
      replace_piece_2: None,
      replace_piece_index_2: None,
    }
  }

  fn with_one_replace(
    from: usize,
    to: usize,
    capture: bool,
    replace_piece: i8,
    replace_piece_index: usize,
  ) -> PieceMove {
    PieceMove {
      from,
      to,
      capture,
      replace_piece_1: Some(replace_piece),
      replace_piece_index_1: Some(replace_piece_index),
      replace_piece_2: None,
      replace_piece_index_2: None,
    }
  }

  fn with_two_replaces(
    from: usize,
    to: usize,
    capture: bool,
    replace_piece_1: i8,
    replace_piece_index_1: usize,
    replace_piece_2: i8,
    replace_piece_index_2: usize,
  ) -> PieceMove {
    PieceMove {
      from,
      to,
      capture,
      replace_piece_1: Some(replace_piece_1),
      replace_piece_index_1: Some(replace_piece_index_1),
      replace_piece_2: Some(replace_piece_2),
      replace_piece_index_2: Some(replace_piece_index_2),
    }
  }
}

#[derive(Clone, Copy, Debug)]
struct Board {
  pieces: [i8; 120],

  white_to_move: bool,

  white_king_castle: bool,
  white_queen_castle: bool,
  black_king_castle: bool,
  black_queen_castle: bool,

  en_passant_index: Option<usize>,

  halfmove_clock: u8,
  fullmove_counter: u32,
}

impl Default for Board {
  fn default() -> Board {
    Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
  }
}

impl ToString for Board {
  fn to_string(&self) -> String {
    let mut s = String::new();

    for y in 2..=9 {
      for x in 1..=8 {
        let mut c = match self.pieces[y * 10 + x].abs() {
          PAWN => " P ".to_string(),
          KNIGHT => " N ".to_string(),
          BISHOP => " B ".to_string(),
          ROOK => " R ".to_string(),
          QUEEN => " Q ".to_string(),
          KING => " K ".to_string(),
          _ => "   ".to_string(),
        };

        if self.pieces[y * 10 + x] < 0 {
          c = c.to_lowercase();
        }

        s += &c;

        if x != 8 {
          s += "|";
        }
      }
      if y != 9 {
        s += "\n---+---+---+---+---+---+---+---\n";
      }
    }

    s
  }
}

impl Board {
  fn square_to_index(square: &str) -> usize {
    let file = square.bytes().nth(0).unwrap() as u32;
    let rank = square.chars().nth(1).unwrap().to_digit(10).unwrap();

    let x = file - 97 + 1;
    let y = 10 - rank;

    (y * 10 + x).try_into().unwrap()
  }

  fn index_to_square(index: usize) -> String {
    let x = index % 10;
    let y = index / 10;

    let file = (x - 1 + 97) as u8 as char;
    let rank = (9 - (y - 1)).to_string();

    let mut o = String::new();

    o.push(file);
    o += &rank;

    o
  }

  fn from_fen(fen: &str) -> Board {
    let a: Vec<&str> = fen.split(' ').collect();
    let p: Vec<&str> = a[0].split('/').collect();

    let mut pieces = [OUTSIDE; 120];

    for (i, s) in p.iter().enumerate() {
      let y = 2 + i;
      let mut x = 1;
      for c in s.chars() {
        if c.is_alphabetic() {
          pieces[y * 10 + x] = {
            match c.to_uppercase().to_string().as_str() {
              "P" => PAWN,
              "N" => KNIGHT,
              "B" => BISHOP,
              "R" => ROOK,
              "Q" => QUEEN,
              "K" => KING,
              _ => panic!("Incorrect fen"),
            }
          };

          if c.is_lowercase() {
            pieces[y * 10 + x] *= -1;
          }

          x += 1;
        } else {
          let digit = c.to_digit(10).expect("Incorrect fen");
          for _ in 0..digit {
            pieces[y * 10 + x] = EMPTY;
            x += 1;
          }
        }
      }
    }

    Board {
      pieces,
      white_to_move: a[1] == "w",
      white_king_castle: a[2].contains('K'),
      white_queen_castle: a[2].contains('Q'),
      black_king_castle: a[2].contains('k'),
      black_queen_castle: a[2].contains('q'),
      en_passant_index: {
        match a[3] {
          "-" => None,
          _ => Some(Board::square_to_index(a[3])),
        }
      },
      halfmove_clock: a[4].parse().unwrap(),
      fullmove_counter: a[5].parse().unwrap(),
    }
  }

  fn generate_fen(&self) -> String {
    let mut out = String::new();

    for y in 2..=9 {
      let mut empty_count = 0;
      for x in 1..=8 {
        let mut c = match self.pieces[y * 10 + x].abs() {
          PAWN => "P".to_string(),
          KNIGHT => "N".to_string(),
          BISHOP => "B".to_string(),
          ROOK => "R".to_string(),
          QUEEN => "Q".to_string(),
          KING => "K".to_string(),
          _ => "".to_string(),
        };

        if self.pieces[y * 10 + x] < 0 {
          c = c.to_lowercase();
        }

        if !c.is_empty() {
          if empty_count > 0 {
            out += &empty_count.to_string();
            empty_count = 0;
          }
          out += &c;
        } else {
          empty_count += 1;
        }
      }

      if empty_count > 0 {
        out += &empty_count.to_string();
      }
      if y != 9 {
        out += "/";
      }
    }

    if self.white_to_move {
      out += " w ";
    } else {
      out += " b ";
    }

    let mut castling = String::new();

    if self.white_king_castle {
      castling += "K";
    }
    if self.white_queen_castle {
      castling += "Q";
    }
    if self.black_king_castle {
      castling += "k";
    }
    if self.black_queen_castle {
      castling += "q";
    }

    if castling.is_empty() {
      castling = "-".to_string();
    }

    out += &castling;

    if let Some(index) = self.en_passant_index {
      out += &format!(" {}", Board::index_to_square(index));
    } else {
      out += " -";
    }

    out += &format!(" {}", self.halfmove_clock);
    out += &format!(" {}", self.fullmove_counter);

    out
  }

  fn generate_moves(&mut self) -> (Vec<PieceMove>, i8) {
    // knight, bishop, rook, queen, king

    let can_slide: [bool; 5] = [false, true, true, true, false];
    let offset_counts: [i8; 5] = [8, 4, 4, 8, 8];
    let offsets: [[i8; 8]; 5] = [
      [-21, -19, -12, -8, 8, 12, 19, 21], // KNIGHT
      [-11, -9, 9, 11, 0, 0, 0, 0],       // BISHOP
      [-10, -1, 1, 10, 0, 0, 0, 0],       // ROOK
      [-11, -10, -9, -1, 1, 9, 10, 11],   // QUEEN
      [-11, -10, -9, -1, 1, 9, 10, 11],   // KING
    ];

    let mut piece_moves: Vec<PieceMove> = Vec::new();

    let mut king_index = 0;

    for &from_index in &BOARD_OFFSET {
      let piece = self.pieces[from_index as usize];

      if piece == EMPTY || piece == OUTSIDE {
        continue;
      }

      if (self.white_to_move && piece > 0) || (!self.white_to_move && piece < 0) {
        if piece.abs() == KING {
          king_index = from_index;
        }

        if piece.abs() != PAWN {
          for offset in 0..offset_counts[(piece.abs() - 2) as usize] {
            let mut to_index = from_index;

            loop {
              to_index += offsets[(piece.abs() - 2) as usize][offset as usize];
              if self.pieces[to_index as usize] == OUTSIDE {
                break;
              }
              if self.pieces[to_index as usize] != EMPTY {
                if (self.white_to_move && self.pieces[to_index as usize] < 0)
                  || (!self.white_to_move && self.pieces[to_index as usize] > 0)
                {
                  piece_moves.push(PieceMove::new(from_index as usize, to_index as usize, true));
                }
                break;
              }
              piece_moves.push(PieceMove::new(from_index as usize, to_index as usize, false));
              if !can_slide[(piece.abs() - 2) as usize] {
                break;
              }
            }
          }
        } else {
          // TODO: clean this up
          if self.white_to_move {
            if self.pieces[(from_index - 10) as usize] == EMPTY {
              if from_index / 10 == 3 {
                // push to promote from 7th rank
                let from = from_index as usize;
                let to = (from_index - 10) as usize;
                let capture = false;
                let replace_piece_index = to;

                piece_moves.push(PieceMove::with_one_replace(
                  from,
                  to,
                  capture,
                  KNIGHT,
                  replace_piece_index,
                ));
                piece_moves.push(PieceMove::with_one_replace(
                  from,
                  to,
                  capture,
                  BISHOP,
                  replace_piece_index,
                ));
                piece_moves.push(PieceMove::with_one_replace(
                  from,
                  to,
                  capture,
                  ROOK,
                  replace_piece_index,
                ));
                piece_moves.push(PieceMove::with_one_replace(
                  from,
                  to,
                  capture,
                  QUEEN,
                  replace_piece_index,
                ));
              } else {
                // normal push
                piece_moves.push(PieceMove::new(from_index as usize, (from_index - 10) as usize, false));
              }
              if from_index / 10 == 8 && self.pieces[(from_index - 20) as usize] == EMPTY {
                // double push from 2nd rank
                piece_moves.push(PieceMove::new(from_index as usize, (from_index - 20) as usize, false));
              }
            }

            let diag_piece_1 = self.pieces[(from_index - 9) as usize];
            let diag_piece_2 = self.pieces[(from_index - 11) as usize];

            if self.en_passant_index == Some((from_index - 9) as usize) {
              // en passant's square
              piece_moves.push(PieceMove::with_one_replace(
                from_index as usize,
                (from_index - 9) as usize,
                true,
                EMPTY,
                (from_index + 1) as usize,
              ));
            } else if diag_piece_1 != EMPTY && diag_piece_1 != OUTSIDE && diag_piece_1 < 0 {
              // black's piece
              if from_index / 10 == 3 {
                // promote by capturing from 7th rank
                let from = from_index as usize;
                let to = (from_index - 9) as usize;
                let capture = true;
                let replace_piece_index = to;

                piece_moves.push(PieceMove::with_one_replace(
                  from,
                  to,
                  capture,
                  KNIGHT,
                  replace_piece_index,
                ));
                piece_moves.push(PieceMove::with_one_replace(
                  from,
                  to,
                  capture,
                  BISHOP,
                  replace_piece_index,
                ));
                piece_moves.push(PieceMove::with_one_replace(
                  from,
                  to,
                  capture,
                  ROOK,
                  replace_piece_index,
                ));
                piece_moves.push(PieceMove::with_one_replace(
                  from,
                  to,
                  capture,
                  QUEEN,
                  replace_piece_index,
                ));
              } else {
                piece_moves.push(PieceMove::new(from_index as usize, (from_index - 9) as usize, true));
              }
            }

            if self.en_passant_index == Some((from_index - 11) as usize) {
              // en passant's square
              piece_moves.push(PieceMove::with_one_replace(
                from_index as usize,
                (from_index - 11) as usize,
                true,
                EMPTY,
                (from_index - 1) as usize,
              ));
            } else if diag_piece_2 != EMPTY && diag_piece_2 != OUTSIDE && diag_piece_2 < 0 {
              // black's piece
              if from_index / 10 == 3 {
                // promote by capturing from 7th rank
                let from = from_index as usize;
                let to = (from_index - 11) as usize;
                let capture = true;
                let replace_piece_index = to;

                piece_moves.push(PieceMove::with_one_replace(
                  from,
                  to,
                  capture,
                  KNIGHT,
                  replace_piece_index,
                ));
                piece_moves.push(PieceMove::with_one_replace(
                  from,
                  to,
                  capture,
                  BISHOP,
                  replace_piece_index,
                ));
                piece_moves.push(PieceMove::with_one_replace(
                  from,
                  to,
                  capture,
                  ROOK,
                  replace_piece_index,
                ));
                piece_moves.push(PieceMove::with_one_replace(
                  from,
                  to,
                  capture,
                  QUEEN,
                  replace_piece_index,
                ));
              } else {
                piece_moves.push(PieceMove::new(from_index as usize, (from_index - 11) as usize, true));
              }
            }
          } else {
            // black's turn
            if self.pieces[(from_index + 10) as usize] == EMPTY {
              if from_index / 10 == 8 {
                // push to promote from 2nd rank
                let from = from_index as usize;
                let to = (from_index + 10) as usize;
                let capture = false;
                let replace_piece_index = to;

                piece_moves.push(PieceMove::with_one_replace(
                  from,
                  to,
                  capture,
                  -KNIGHT,
                  replace_piece_index,
                ));
                piece_moves.push(PieceMove::with_one_replace(
                  from,
                  to,
                  capture,
                  -BISHOP,
                  replace_piece_index,
                ));
                piece_moves.push(PieceMove::with_one_replace(
                  from,
                  to,
                  capture,
                  -ROOK,
                  replace_piece_index,
                ));
                piece_moves.push(PieceMove::with_one_replace(
                  from,
                  to,
                  capture,
                  -QUEEN,
                  replace_piece_index,
                ));
              } else {
                // normal push
                piece_moves.push(PieceMove::new(from_index as usize, (from_index + 10) as usize, false));
              }

              if from_index / 10 == 3 && self.pieces[(from_index + 20) as usize] == EMPTY {
                // double push from 7th rank
                piece_moves.push(PieceMove::new(from_index as usize, (from_index + 20) as usize, false));
              }
            }

            let diag_piece_1 = self.pieces[(from_index + 9) as usize];
            let diag_piece_2 = self.pieces[(from_index + 11) as usize];

            if self.en_passant_index == Some((from_index + 9) as usize) {
              // en passant's square
              piece_moves.push(PieceMove::with_one_replace(
                from_index as usize,
                (from_index + 9) as usize,
                true,
                EMPTY,
                (from_index - 1) as usize,
              ));
            } else if diag_piece_1 != EMPTY && diag_piece_1 != OUTSIDE && diag_piece_1 > 0 {
              // white's piece
              if from_index / 10 == 8 {
                // promote by capturing from 2nd rank
                let from = from_index as usize;
                let to = (from_index + 9) as usize;
                let capture = true;
                let replace_piece_index = to;

                piece_moves.push(PieceMove::with_one_replace(
                  from,
                  to,
                  capture,
                  -KNIGHT,
                  replace_piece_index,
                ));
                piece_moves.push(PieceMove::with_one_replace(
                  from,
                  to,
                  capture,
                  -BISHOP,
                  replace_piece_index,
                ));
                piece_moves.push(PieceMove::with_one_replace(
                  from,
                  to,
                  capture,
                  -ROOK,
                  replace_piece_index,
                ));
                piece_moves.push(PieceMove::with_one_replace(
                  from,
                  to,
                  capture,
                  -QUEEN,
                  replace_piece_index,
                ));
              } else {
                piece_moves.push(PieceMove::new(from_index as usize, (from_index + 9) as usize, true));
              }
            }

            if self.en_passant_index == Some((from_index + 11) as usize) {
              // en passant's square
              piece_moves.push(PieceMove::with_one_replace(
                from_index as usize,
                (from_index + 11) as usize,
                true,
                EMPTY,
                (from_index + 1) as usize,
              ));
            } else if diag_piece_2 != EMPTY && diag_piece_2 != OUTSIDE && diag_piece_2 > 0 {
              // white's piece
              if from_index / 10 == 8 {
                // promote by capturing from 2nd rank
                let from = from_index as usize;
                let to = (from_index + 11) as usize;
                let capture = true;
                let replace_piece_index = to;

                piece_moves.push(PieceMove::with_one_replace(
                  from,
                  to,
                  capture,
                  -KNIGHT,
                  replace_piece_index,
                ));
                piece_moves.push(PieceMove::with_one_replace(
                  from,
                  to,
                  capture,
                  -BISHOP,
                  replace_piece_index,
                ));
                piece_moves.push(PieceMove::with_one_replace(
                  from,
                  to,
                  capture,
                  -ROOK,
                  replace_piece_index,
                ));
                piece_moves.push(PieceMove::with_one_replace(
                  from,
                  to,
                  capture,
                  -QUEEN,
                  replace_piece_index,
                ));
              } else {
                piece_moves.push(PieceMove::new(from_index as usize, (from_index + 11) as usize, true));
              }
            }
          }
        }
      }
    }

    // castle moves
    if self.white_to_move && (self.white_queen_castle || self.white_king_castle) {
      // if it's white's turn and their king is safe
      if self.square_is_safe(95) {
        if self.white_queen_castle {
          if self.pieces[94] == EMPTY
            && self.pieces[93] == EMPTY
            && self.pieces[92] == EMPTY
            && self.square_is_safe(94)
            && self.square_is_safe(93)
          {
            piece_moves.push(PieceMove::with_two_replaces(95, 93, false, EMPTY, 91, ROOK, 94))
          }
        }
        if self.white_king_castle {
          if self.pieces[96] == EMPTY && self.pieces[97] == EMPTY && self.square_is_safe(96) && self.square_is_safe(97)
          {
            piece_moves.push(PieceMove::with_two_replaces(95, 97, false, EMPTY, 98, ROOK, 96))
          }
        }
      }
    } else if !self.white_to_move && (self.black_queen_castle || self.black_king_castle) {
      // if it's blacks's turn and their king is safe
      if self.square_is_safe(25) {
        if self.black_queen_castle {
          if self.pieces[24] == EMPTY
            && self.pieces[23] == EMPTY
            && self.pieces[22] == EMPTY
            && self.square_is_safe(24)
            && self.square_is_safe(23)
          {
            piece_moves.push(PieceMove::with_two_replaces(25, 23, false, EMPTY, 21, -ROOK, 24))
          }
        }
        if self.black_king_castle {
          if self.pieces[26] == EMPTY && self.pieces[27] == EMPTY && self.square_is_safe(26) && self.square_is_safe(27)
          {
            piece_moves.push(PieceMove::with_two_replaces(25, 27, false, EMPTY, 28, -ROOK, 26))
          }
        }
      }
    }

    // filter out all illegal moves
    let legal_piece_moves = piece_moves
      .into_iter()
      .filter(|m| self.move_is_legal(m))
      .sorted_by_key(|m| !m.capture)
      .sorted_by_key(|m| !m.replace_piece_1.is_some())
      .collect_vec();

    let king_is_safe = self.square_is_safe(king_index as usize);
    let empty_vec = vec![];

    // TODO: check for repetition of moves

    if !king_is_safe && legal_piece_moves.is_empty() {
      // king in check and no moves, so checkmate
      return (empty_vec, CHECKMATE);
    } else if self.halfmove_clock == 49 || (king_is_safe && legal_piece_moves.is_empty()) {
      // no checkmate and the halfmove clock will reach 50, so stalemate
      // or
      // king is in not in check and no moves, so stalemate
      return (empty_vec, STALEMATE);
    } else if !king_is_safe {
      // king is in check and has moves, so check
      return (legal_piece_moves, CHECK);
    }

    (legal_piece_moves, NOTHING)
  }

  fn move_is_legal(&mut self, piece_move: &PieceMove) -> bool {
    let from_piece = self.pieces[piece_move.from];
    let to_piece = self.pieces[piece_move.to];

    let mut replaced_piece_1: Option<i8> = None;
    let mut replaced_piece_2: Option<i8> = None;

    // move pieces (undo later)
    self.pieces[piece_move.from] = EMPTY;
    self.pieces[piece_move.to] = from_piece;

    if let Some(replace_piece_1) = piece_move.replace_piece_1 {
      if let Some(replace_piece_index_1) = piece_move.replace_piece_index_1 {
        replaced_piece_1 = Some(self.pieces[replace_piece_index_1]);
        self.pieces[replace_piece_index_1] = replace_piece_1;
      }
    }

    if let Some(replace_piece_2) = piece_move.replace_piece_2 {
      if let Some(replace_piece_index_2) = piece_move.replace_piece_index_2 {
        replaced_piece_2 = Some(self.pieces[replace_piece_index_2]);
        self.pieces[replace_piece_index_2] = replace_piece_2;
      }
    }

    let mut is_legal = true;

    for &from_index in &BOARD_OFFSET {
      let piece = self.pieces[from_index as usize];

      if (piece == KING && self.white_to_move) || (piece == -KING && !self.white_to_move) {
        // your own king can't be in check after your move
        is_legal = self.square_is_safe(from_index as usize);

        break; // already found king
      }
    }

    // undo all changes to board
    if let Some(replaced_piece_2) = replaced_piece_2 {
      if let Some(replace_piece_index_2) = piece_move.replace_piece_index_2 {
        self.pieces[replace_piece_index_2] = replaced_piece_2;
      }
    }
    if let Some(replaced_piece_1) = replaced_piece_1 {
      if let Some(replace_piece_index_1) = piece_move.replace_piece_index_1 {
        self.pieces[replace_piece_index_1] = replaced_piece_1;
      }
    }

    self.pieces[piece_move.from] = from_piece;
    self.pieces[piece_move.to] = to_piece;

    return is_legal;
  }

  fn square_is_safe(&self, square_index: usize) -> bool {
    // a safe square is a square, which is not under attack by any opponent pieces

    let offset_counts: [usize; 3] = [8, 4, 4];
    let offsets: [[i8; 8]; 3] = [
      [-21, -19, -12, -8, 8, 12, 19, 21], // KNIGHT
      [-11, -9, 9, 11, 0, 0, 0, 0],       // BISHOP/QUEEN or maybe KING
      [-10, -1, 1, 10, 0, 0, 0, 0],       // ROOK/QUEEN or maybe KING
    ];

    // check pawn attacks. except en passant
    if self.white_to_move {
      // check for black pawns
      if self.pieces[(square_index - 9) as usize] == -PAWN || self.pieces[(square_index - 11) as usize] == -PAWN {
        return false;
      }
    } else {
      // check for white pawns
      if self.pieces[(square_index + 9) as usize] == PAWN || self.pieces[(square_index + 11) as usize] == PAWN {
        return false;
      }
    }

    for (type_index, offset_type) in offsets.iter().enumerate() {
      for offset_index in 0..offset_counts[type_index] {
        let offset = offset_type[offset_index];
        let mut new_index = square_index as i8;
        let mut one_away = true;

        loop {
          new_index += offset;
          let new_piece = self.pieces[new_index as usize];
          if new_piece == OUTSIDE {
            break;
          }
          if new_piece != EMPTY {
            if new_piece > 0 {
              // white's piece
              if self.white_to_move {
                // white's turn
                break;
              } else {
                if (type_index == 0 && new_piece == KNIGHT)
                  || (type_index == 1 && (new_piece == BISHOP || new_piece == QUEEN || (new_piece == KING && one_away)))
                  || (type_index == 2 && (new_piece == ROOK || new_piece == QUEEN || (new_piece == KING && one_away)))
                {
                  return false;
                }
              }
            } else {
              // black's piece
              if self.white_to_move {
                if (type_index == 0 && new_piece == -KNIGHT)
                  || (type_index == 1
                    && (new_piece == -BISHOP || new_piece == -QUEEN || (new_piece == -KING && one_away)))
                  || (type_index == 2
                    && (new_piece == -ROOK || new_piece == -QUEEN || (new_piece == -KING && one_away)))
                {
                  return false;
                }
              } else {
                // black's turn
                break;
              }
            }
            break;
          }
          if type_index == 0 {
            // horses can only jump once
            break;
          }
          one_away = false;
        }
      }
    }

    return true;
  }

  fn move_piece(&mut self, piece_move: &PieceMove) {
    let a8 = 21;
    let h8 = 28;
    let a1 = 91;
    let h1 = 98;

    let from_piece = self.pieces[piece_move.from];

    if from_piece == KING {
      self.white_king_castle = false;
      self.white_queen_castle = false;
    } else if from_piece == -KING {
      self.black_king_castle = false;
      self.black_queen_castle = false;
    }

    if piece_move.from == a8 || piece_move.to == a8 {
      self.black_queen_castle = false;
    }
    if piece_move.from == h8 || piece_move.to == h8 {
      self.black_king_castle = false;
    }
    if piece_move.from == a1 || piece_move.to == a1 {
      self.white_queen_castle = false;
    }
    if piece_move.from == h1 || piece_move.to == h1 {
      self.white_king_castle = false;
    }

    if from_piece == PAWN && piece_move.from - piece_move.to == 20 {
      self.en_passant_index = Some(piece_move.from - 10);
    } else if from_piece == -PAWN && piece_move.to - piece_move.from == 20 {
      self.en_passant_index = Some(piece_move.from + 10);
    } else {
      self.en_passant_index = None;
    }

    if piece_move.capture || from_piece.abs() == PAWN {
      self.halfmove_clock = 0;
    } else {
      self.halfmove_clock += 1;
    }

    if !self.white_to_move {
      self.fullmove_counter += 1
    };

    self.white_to_move = !self.white_to_move;

    self.pieces[piece_move.from] = EMPTY;
    self.pieces[piece_move.to] = from_piece;

    if let Some(replace_piece_1) = piece_move.replace_piece_1 {
      if let Some(replace_piece_index_1) = piece_move.replace_piece_index_1 {
        self.pieces[replace_piece_index_1] = replace_piece_1;
      }
    }

    if let Some(replace_piece_2) = piece_move.replace_piece_2 {
      if let Some(replace_piece_index_2) = piece_move.replace_piece_index_2 {
        self.pieces[replace_piece_index_2] = replace_piece_2;
      }
    }
  }

  fn hash(&self) -> BoardHash {
    let mut pawns_hash = 0_u64;
    let mut knights_hash = 0_u64;
    let mut bishops_hash = 0_u64;
    let mut rooks_hash = 0_u64;
    let mut queens_hash = 0_u64;
    let mut kings_hash = 0_u64;
    let mut colors_hash = 0_u64;
    let mut misc = 0_u64;

    let mut place = 1;

    for &from_index in &BOARD_OFFSET {
      let piece = self.pieces[from_index as usize];
      if piece != EMPTY {
        if piece > 0 {
          colors_hash |= place;
        }
        if piece.abs() == PAWN {
          pawns_hash |= place;
        } else if piece.abs() == KNIGHT {
          knights_hash |= place;
        } else if piece.abs() == BISHOP {
          bishops_hash |= place;
        } else if piece.abs() == ROOK {
          rooks_hash |= place;
        } else if piece.abs() == QUEEN {
          queens_hash |= place;
        } else if piece.abs() == KING {
          kings_hash |= place;
        }
      }
      place <<= 1;
    }

    if self.white_king_castle {
      misc |= 1;
    }
    if self.white_queen_castle {
      misc |= 1 << 1;
    }
    if self.black_king_castle {
      misc |= 1 << 2;
    }
    if self.black_queen_castle {
      misc |= 1 << 3;
    }
    if self.white_to_move {
      misc |= 1 << 4;
    }
    if self.halfmove_clock > 40 {
      misc |= 1 << 5;
    }
    if let Some(en_passant_index) = self.en_passant_index {
      let x = en_passant_index % 10;
      let y = en_passant_index / 10;
      let index = (y - 2) * 8 + (x - 1);

      misc |= 1 << index;
    }

    (
      pawns_hash,
      knights_hash,
      bishops_hash,
      rooks_hash,
      queens_hash,
      kings_hash,
      colors_hash,
      misc,
    )
  }

  fn evaluate(&self) -> i32 {
    let mut score: i32 = 0;

    for &from_index in &BOARD_OFFSET {
      let piece = self.pieces[from_index as usize];

      if piece != EMPTY {
        score += piece.signum() as i32 * {
          match piece.abs() {
            PAWN => 1,
            KNIGHT => 3,
            BISHOP => 3,
            ROOK => 5,
            QUEEN => 9,
            _ => 0,
          }
        };
      }
    }

    score
  }
}

fn main() {
  let current_time = Instant::now();

  // let board = Board::default();
  // let board = Board::from_fen("2Q5/5q1r/3Q1pkn/P3p2p/6p1/2NP4/4PPPP/2KR1BNR w - - 3 33");
  // let board = Board::from_fen("6n1/Q2bkq2/5p1r/P3p2p/6p1/3P4/4PPPP/RN1QKBNR w KQ - 10 27");
  // let board = Board::from_fen("r3k2r/p1ppqpb1/1n2Pnp1/4N3/1p2P3/2N2Q1p/PPPBbPPP/R3K2R w KQkq - 0 2");
  // let board = Board::from_fen("5BN1/8/8/1p6/1N6/kP6/2K5/8 w - - 0 2"); // ??????? probably found bug. after the recommended move it's stalemate, even though it says its checkmate

  // let mut hashmap: HashMap<BoardHash, i32> = HashMap::new();

  // const DEPTH: i32 = 8;

  // idk, something stupid
  // fn capture_search(board: Board, hashmap: &mut HashMap<BoardHash, i32>, alpha: i32, beta: i32) -> i32 {
  //   let mut board = board;
  //   let mut alpha = alpha;
  //   let mut beta = beta;

  //   let board_eval = board.evaluate();

  //   if board.white_to_move {
  //     if board_eval >= alpha {
  //       return alpha;
  //     }
  //     if beta < board_eval {
  //       beta = board_eval;
  //     }
  //   } else {
  //     if board_eval >= beta {
  //       return beta;
  //     }
  //     if alpha < board_eval {
  //       alpha = board_eval;
  //     }
  //   }

  //   let (mut available_moves, result) = board.generate_moves();
  //   available_moves = available_moves.into_iter().filter(|m| m.capture).collect();

  //   if result == CHECKMATE {
  //     return if board.white_to_move { -1000 } else { 1000 };
  //   } else if result == STALEMATE {
  //     return 0;
  //   }

  //   if board.white_to_move {
  //     for available_move in available_moves {
  //       let mut new_board = board.clone();
  //       new_board.move_piece(&available_move);

  //       let eval = capture_search(new_board, hashmap, alpha, beta);

  //       if eval >= alpha {
  //         return alpha;
  //       }
  //       if eval > beta {
  //         beta = eval;
  //       }
  //     }

  //     return beta;
  //   } else {
  //     for available_move in available_moves {
  //       let mut new_board = board.clone();
  //       new_board.move_piece(&available_move);

  //       let eval = capture_search(new_board, hashmap, alpha, beta);

  //       if eval >= beta {
  //         return beta;
  //       }
  //       if eval > alpha {
  //         alpha = eval;
  //       }
  //     }

  //     return alpha;
  //   }
  // }

  // fn search(
  //   board: Board,
  //   depth: i32,
  //   hashmap: &mut HashMap<BoardHash, i32>,
  //   alpha: i32,
  //   beta: i32,
  // ) -> (i32, Option<PieceMove>) {
  //   let mut board = board;
  //   let mut alpha = alpha;
  //   let mut beta = beta;

  //   let board_hash = board.hash();
  //   if let Some(v) = hashmap.get(&board_hash) {
  //     return (*v, None);
  //   }

  //   let (mut available_moves, result) = board.generate_moves();
  //   // if depth < 0 {
  //   //   available_moves = available_moves.into_iter().filter(|m| m.capture).collect();
  //   // }

  //   if result == CHECKMATE {
  //     // if !board.white_to_move {
  //     //   dbg!("ree");
  //     // }
  //     return if board.white_to_move {
  //       (-1000, None)
  //     } else {
  //       (1000, None)
  //     };
  //   } else if result == STALEMATE {
  //     return (0, None);
  //   }

  //   if depth == 0 {
  //     return (board.evaluate(), None);
  //   }

  //   // if available_moves.is_empty() || depth < -5 {
  //   //   return (board.evaluate(), None);

  //   //   // return (capture_search(board, hashmap, alpha, beta), None);
  //   // }

  //   if board.white_to_move {
  //     let mut max_eval = MIN;
  //     let mut max_eval_move = None;

  //     for available_move in available_moves {
  //       let mut new_board = board.clone();
  //       new_board.move_piece(&available_move);

  //       let (eval, _) = search(new_board, depth - 1, hashmap, alpha, beta);

  //       if eval > max_eval {
  //         max_eval = eval;
  //         max_eval_move = Some(available_move);
  //       }
  //       if max_eval > alpha {
  //         alpha = eval;
  //       }
  //       if beta <= alpha {
  //         break;
  //       }
  //     }

  //     hashmap.insert(board_hash, max_eval);

  //     return (max_eval, max_eval_move);
  //   } else {
  //     let mut min_eval = MAX;
  //     let mut min_eval_move = None;

  //     for available_move in available_moves {
  //       let mut new_board = board.clone();
  //       new_board.move_piece(&available_move);

  //       let (eval, _) = search(new_board, depth - 1, hashmap, alpha, beta);

  //       if eval < min_eval {
  //         min_eval = eval;
  //         min_eval_move = Some(available_move);
  //       }
  //       if min_eval < beta {
  //         beta = eval;
  //       }
  //       if beta <= alpha {
  //         break;
  //       }
  //     }

  //     hashmap.insert(board_hash, min_eval);

  //     return (min_eval, min_eval_move);
  //   }
  // }

  // let r = search(board, DEPTH, &mut hashmap, MIN, MAX);

  // println!(
  //   "{}, {} | {} -> {}",
  //   r.0,
  //   r.1.unwrap().replace_piece_1.unwrap_or(0),
  //   Board::index_to_square(r.1.unwrap().from),
  //   Board::index_to_square(r.1.unwrap().to),
  // );

  // #[derive(Debug)]
  // struct State {
  //   nodes: [u64; DEPTH + 1],
  //   captures: [u64; DEPTH + 1],
  //   checks: [u64; DEPTH + 1],
  //   checkmates: [u64; DEPTH + 1],
  // }

  // let mut state = State {
  //   nodes: [0; DEPTH + 1],
  //   captures: [0; DEPTH + 1],
  //   checks: [0; DEPTH + 1],
  //   checkmates: [0; DEPTH + 1],
  // };

  // fn recurse(board: Board, state: &mut State, depth: usize) {
  //   state.nodes[depth] += 1;

  //   let mut board = board;
  //   let (available_moves, result) = board.generate_moves();

  //   if result == CHECK {
  //     state.checks[depth] += 1;
  //   } else if result == CHECKMATE {
  //     state.checks[depth] += 1;
  //     state.checkmates[depth] += 1;
  //     return;
  //   } else if result == STALEMATE {
  //     return;
  //   }

  //   if depth == 0 {
  //     return;
  //   }

  //   for available_move in available_moves {
  //     if available_move.capture {
  //       state.captures[depth - 1] += 1;
  //     }

  //     let mut new_board = board.clone();
  //     new_board.move_piece(&available_move);

  //     recurse(new_board, state, depth - 1);
  //   }
  // }

  // recurse(board, &mut state, DEPTH);

  // dbg!(state);

  let x = board::square_to_index("a8");
  let y = board::index_to_square(98);
  dbg!(x, y);

  println!("Time taken: {:?}", current_time.elapsed());
}
