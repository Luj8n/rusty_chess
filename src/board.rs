const PAWN: i8 = 1;
const KNIGHT: i8 = 2;
const BISHOP: i8 = 3;
const ROOK: i8 = 4;
const QUEEN: i8 = 5;
const KING: i8 = 6;
const EMPTY: i8 = 7;
const OUTSIDE: i8 = 0;

// For example: square_to_index("a1") == 21
pub fn square_to_index(square: &str) -> usize {
  let file = *square.as_bytes().first().unwrap() as u32;
  let rank = square.chars().nth(1).unwrap().to_digit(10).unwrap();

  let x = file - 97 + 1;
  let y = 10 - rank;

  (y * 10 + x).try_into().unwrap()
}

pub fn index_to_square(index: usize) -> String {
  let x = index % 10;
  let y = index / 10;

  let file = (x - 1 + 97) as u8 as char;
  let rank = (9 - (y - 1)).to_string();

  let mut o = String::new();

  o.push(file);
  o += &rank;

  o
}

#[derive(Clone, Debug)]
struct Board {
  // The actual piece board. It is 10x12.
  // 2 vertical and 1 horizontal padding.
  // Top left corner (index 21) is a8, and the bottom left corner (index 98) is h1.
  // Basically, it looks like a real chess board would.
  // If a value is negative, then it is black's piece.
  // Otherwise, it is either empty, outside (in the padding) or white's piece.
  pieces: [i8; 120],

  // Whether it's white turn to move
  white_to_move: bool,

  // Castling right. It is true if it is legal to do it.
  white_king_castle: bool,
  white_queen_castle: bool,
  black_king_castle: bool,
  black_queen_castle: bool,

  // The en passant target square is specified after a double push of a pawn,
  // no matter whether an en passant capture is really possible or not
  en_passant_index: Option<usize>,

  // The halfmove clock specifies a decimal number of half moves with respect to the 50 move draw rule.
  // It is reset to zero after a capture or a pawn move and incremented otherwise.
  halfmove_clock: u8,

  // The number of the full moves in a game.
  // It starts at 1, and is incremented after each Black's move.
  fullmove_counter: u32,
}

impl Default for Board {
  fn default() -> Board {
    Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
  }
}

impl Board {
  fn from_fen(fen: &str) -> Board {
    let fields: Vec<&str> = fen.split(' ').collect();
    let ranks: Vec<&str> = fields[0].split('/').collect();

    let mut pieces = [OUTSIDE; 120];

    for (i, s) in ranks.iter().enumerate() {
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
      white_to_move: fields[1] == "w",
      white_king_castle: fields[2].contains('K'),
      white_queen_castle: fields[2].contains('Q'),
      black_king_castle: fields[2].contains('k'),
      black_queen_castle: fields[2].contains('q'),
      en_passant_index: {
        match fields[3] {
          "-" => None,
          _ => Some(square_to_index(fields[3])),
        }
      },
      halfmove_clock: fields[4].parse().unwrap(),
      fullmove_counter: fields[5].parse().unwrap(),
    }
  }

  fn to_table(&self) -> String {
    let mut s = String::new();

    for y in 2..=9 {
      for x in 1..=8 {
        let mut c = match self.pieces[y * 10 + x].abs() {
          PAWN => " P ",
          KNIGHT => " N ",
          BISHOP => " B ",
          ROOK => " R ",
          QUEEN => " Q ",
          KING => " K ",
          _ => "   ",
        }
        .to_string();

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
