use std::ops::{Range, RangeInclusive};

const PAWN: i8 = 1;
const KNIGHT: i8 = 2;
const BISHOP: i8 = 3;
const ROOK: i8 = 4;
const QUEEN: i8 = 5;
const KING: i8 = 6;
const EMPTY: i8 = 7;
const OUTSIDE: i8 = 0;

#[derive(Clone, Copy, Debug)]
struct PieceMove {
  from: usize,
  to: usize,
  capture: bool,
}

impl PieceMove {
  fn new(from: usize, to: usize, capture: bool) -> PieceMove {
    PieceMove { from, to, capture }
  }
}

#[derive(Clone, Debug)]
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
    let y = rank - 1 + 2;

    (y * 10 + x).try_into().unwrap()
  }

  fn index_to_square(index: usize) -> String {
    let x = index % 10;
    let y = index / 10;

    let file = (x - 1 + 97) as u8 as char;
    let rank = (8 - (y - 2)).to_string().chars().nth(0).unwrap();

    let mut o = String::new();

    o.push(file);
    o.push(rank);

    o
  }

  fn from_fen(fen: &str) -> Board {
    let a: Vec<&str> = fen.split(" ").collect();
    let p: Vec<&str> = a[0].split("/").collect();

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

  fn generate_moves(&self) {
    // knight, bishop, rook, queen, king

    let can_slide = [false, true, true, true, false];
    let offset_counts = [8, 4, 4, 8, 8];
    let offset = [
      [-21, -19, -12, -8, 8, 12, 19, 21], // KNIGHT
      [-11, -9, 9, 11, 0, 0, 0, 0],       // BISHOP
      [-10, -1, 1, 10, 0, 0, 0, 0],       // ROOK
      [-11, -10, -9, -1, 1, 9, 10, 11],   // QUEEN
      [-11, -10, -9, -1, 1, 9, 10, 11],   // KING
    ];

    // convert from 12x10 to 8x8
    let board_offset = [
      21, 22, 23, 24, 25, 26, 27, 28, //
      31, 32, 33, 34, 35, 36, 37, 38, //
      41, 42, 43, 44, 45, 46, 47, 48, //
      51, 52, 53, 54, 55, 56, 57, 58, //
      61, 62, 63, 64, 65, 66, 67, 68, //
      71, 72, 73, 74, 75, 76, 77, 78, //
      81, 82, 83, 84, 85, 86, 87, 88, //
      91, 92, 93, 94, 95, 96, 97, 98, //
    ];

    let moves: Vec<PieceMove> = Vec::new();

    for i in 0..64 {
      let piece = self.pieces[board_offset[i]];

      if piece == 0 || piece == 7 {
        continue;
      }

      if (self.white_to_move && piece > 0) || (!self.white_to_move && piece < 0) {
        // TODO here
      }
    }

    todo!()
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
      self.black_queen_castle = false;
    }
    if piece_move.from == h1 || piece_move.to == h1 {
      self.black_king_castle = false;
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

    self.pieces[piece_move.to] = from_piece;
    self.pieces[piece_move.from] = EMPTY;
  }

  fn hash(&self) -> u64 {
    todo!()
  }
}

fn main() {
  let mut board = Board::default();
  // board.move_piece(&PieceMove::new(33, 53, false));
  // let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
  println!("{}", board.to_string());
  println!("{}", board.generate_fen());
}
