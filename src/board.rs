const PAWN: i8 = 1;
const KNIGHT: i8 = 2;
const BISHOP: i8 = 3;
const ROOK: i8 = 4;
const QUEEN: i8 = 5;
const KING: i8 = 6;
const EMPTY: i8 = 7;
const OUTSIDE: i8 = 0;

// Indices of the 10x12 board. Displayed as an 8x8 board.
const BOARD_INDICES: [i8; 64] = [
  21, 22, 23, 24, 25, 26, 27, 28, //
  31, 32, 33, 34, 35, 36, 37, 38, //
  41, 42, 43, 44, 45, 46, 47, 48, //
  51, 52, 53, 54, 55, 56, 57, 58, //
  61, 62, 63, 64, 65, 66, 67, 68, //
  71, 72, 73, 74, 75, 76, 77, 78, //
  81, 82, 83, 84, 85, 86, 87, 88, //
  91, 92, 93, 94, 95, 96, 97, 98, //
];

// Gives an index of the specified square (of the 10x12 board)
// For example: square_to_index("a8") == 21
fn square_to_index(square: &str) -> usize {
  let file = *square.as_bytes().first().unwrap() as u32;
  let rank = square.chars().nth(1).unwrap().to_digit(10).unwrap();

  let x = file - 97 + 1;
  let y = 10 - rank;

  (y * 10 + x).try_into().unwrap()
}

// Gives the square of the specified index (of the 10x12 board)
// For example: index_to_square(21) == "a8"
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

// Maybe returns a color of a piece.
// If it's empty or outside then it returns None.
fn get_color(square: i8) -> Option<Color> {
  if square == OUTSIDE || square == EMPTY {
    None
  } else if square < 0 {
    Some(Color::Black)
  } else {
    Some(Color::White)
  }
}

#[derive(Clone, Debug, PartialEq)]
enum Color {
  White,
  Black,
}

#[derive(Clone, Debug)]
enum CastlingSide {
  WhiteKing,
  WhiteQueen,
  BlackKing,
  BlackQueen,
}

#[derive(Clone, Debug)]
enum Move {
  Normal {
    from: usize,
    to: usize,
  },
  Capture {
    from: usize,
    to: usize,
    captured_piece: i8,
  },
  PawnPush {
    from: usize,
    to: usize,
  },
  DoublePawnPush {
    from: usize,
    to: usize,
  },
  EnPassant {
    from: usize,
    to: usize,
  },
  Promotion {
    from: usize,
    to: usize,
    selected_piece: i8,
  },
  PromotionWithCapture {
    from: usize,
    to: usize,
    selected_piece: i8,
    captured_piece: i8,
  },
  Castling(CastlingSide),
}

impl Move {
  fn normal(from: usize, to: usize) -> Move {
    Move::Normal { from, to }
  }

  fn capture(from: usize, to: usize, captured_piece: i8) -> Move {
    Move::Capture {
      from,
      to,
      captured_piece,
    }
  }

  fn pawn_push(from: usize, to: usize) -> Move {
    Move::PawnPush { from, to }
  }

  fn double_pawn_push(from: usize, to: usize) -> Move {
    Move::DoublePawnPush { from, to }
  }

  fn en_passant(from: usize, to: usize) -> Move {
    Move::EnPassant { from, to }
  }

  fn promotion(from: usize, to: usize, selected_piece: i8) -> Move {
    Move::Promotion {
      from,
      to,
      selected_piece,
    }
  }

  fn promotion_with_capture(from: usize, to: usize, selected_piece: i8, captured_piece: i8) -> Move {
    Move::PromotionWithCapture {
      from,
      to,
      selected_piece,
      captured_piece,
    }
  }

  fn castling(corner: CastlingSide) -> Move {
    Move::Castling(corner)
  }

  fn to_fen(&self) -> String {
    todo!()
  }
}

// Used for the make-unmake approach
#[derive(Clone, Debug)]
struct UndoMove {
  // TODO: see if u8 with casting is faster (especially with the copy-make approach)
  piece_move: Move,

  // Stores board information before the move
  meta: BoardMeta,
}

// Stores additional board
#[derive(Clone, Debug)]
struct BoardMeta {
  // Castling rights. It is true if it is legal to do it.
  white_king_castle: bool,
  white_queen_castle: bool,
  black_king_castle: bool,
  black_queen_castle: bool,

  // The en passant target square is specified after a double push of a pawn,
  // no matter whether an en passant capture is really possible or not
  // Note: the rank will always be either 3 or 6.
  en_passant_index: Option<usize>,

  // The halfmove clock specifies a decimal number of half moves with respect to the 50 move draw rule.
  // It is reset to zero after a capture or a pawn move and incremented otherwise.
  halfmove_clock: u8,
  // TODO: Some kind of hash. Probably recalculated after every move
  // hash: _,
}

// Stores all the game information
#[derive(Clone, Debug)]
struct Board {
  // The actual piece board. It is 10x12.
  // 2 vertical and 1 horizontal padding.
  // Top left corner (index 21) is a8, and the bottom left corner (index 98) is h1.
  // Basically, it looks like a real chess board would.
  // If a value is negative, then it is black's piece.
  // Otherwise, it is either empty, outside (in the padding) or white's piece.
  pieces: [i8; 120],

  // TODO
  undo_list: Vec<UndoMove>,

  // Whether it's white's turn to move
  side_to_move: Color,

  // The number of the full moves in a game.
  // It starts at 1, and is incremented after each Black's move.
  fullmove_counter: u32,

  // Stores additional information
  // Gets updated with every move
  meta: BoardMeta,
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
      side_to_move: if fields[1] == "w" { Color::White } else { Color::Black },
      fullmove_counter: fields[5].parse().unwrap(),
      undo_list: vec![],
      meta: BoardMeta {
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
      },
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

  // TODO: maybe don't return a vector but instead mutate it
  // Returns pseudo legal moves for a king from a specified square
  fn king_moves(&self, from: usize, color: Color) -> Vec<Move> {
    let mut piece_moves: Vec<Move> = Vec::with_capacity(8);

    let positions: [usize; 8] = [
      from - 11,
      from - 10,
      from - 9,
      from - 1,
      from + 1,
      from + 9,
      from + 10,
      from + 11,
    ];

    for position in positions {
      let square = self.pieces[position];
      if square == OUTSIDE {
        continue;
      }
      if square == EMPTY {
        piece_moves.push(Move::normal(from, position));
      } else if (square < 0 && matches!(color, Color::White)) || (square > 0 && matches!(color, Color::Black)) {
        piece_moves.push(Move::capture(from, position, square));
      }
    }

    piece_moves
  }

  fn knight_moves(&self, from: usize, color: Color) -> Vec<Move> {
    let mut piece_moves: Vec<Move> = Vec::with_capacity(8);

    let positions: [usize; 8] = [
      from - 21,
      from - 19,
      from - 12,
      from - 8,
      from + 8,
      from + 12,
      from + 19,
      from + 21,
    ];

    for position in positions {
      let square = self.pieces[position];
      if square == OUTSIDE {
        continue;
      }
      if square == EMPTY {
        piece_moves.push(Move::normal(from, position));
      } else if (square < 0 && matches!(color, Color::White)) || (square > 0 && matches!(color, Color::Black)) {
        piece_moves.push(Move::capture(from, position, square));
      }
    }

    piece_moves
  }

  fn bishop_moves(&self, from: usize, color: Color) -> Vec<Move> {
    let mut piece_moves: Vec<Move> = Vec::with_capacity(13);

    let directions: [i8; 4] = [-11, -9, 9, 11];

    for direction in directions {
      let mut position: i8 = from as i8;

      loop {
        position += direction;

        let square = self.pieces[position as usize];

        if square == OUTSIDE {
          break;
        }

        if square == EMPTY {
          piece_moves.push(Move::normal(from, position as usize));
        } else {
          if (square < 0 && matches!(color, Color::White)) || (square > 0 && matches!(color, Color::Black)) {
            piece_moves.push(Move::capture(from, position as usize, square));
          }

          break;
        }
      }
    }

    piece_moves
  }

  fn rook_moves(&self, from: usize, color: Color) -> Vec<Move> {
    let mut piece_moves: Vec<Move> = Vec::with_capacity(14);

    let directions: [i8; 4] = [-10, -1, 1, 10];

    for direction in directions {
      let mut position: i8 = from as i8;

      loop {
        position += direction;

        let square = self.pieces[position as usize];

        if square == OUTSIDE {
          break;
        }

        if square == EMPTY {
          piece_moves.push(Move::normal(from, position as usize));
        } else {
          if (square < 0 && matches!(color, Color::White)) || (square > 0 && matches!(color, Color::Black)) {
            piece_moves.push(Move::capture(from, position as usize, square));
          }

          break;
        }
      }
    }

    piece_moves
  }

  fn queen_moves(&self, from: usize, color: Color) -> Vec<Move> {
    let mut piece_moves: Vec<Move> = Vec::with_capacity(27);

    let directions: [i8; 8] = [-11, -10, -9, -1, 1, 9, 10, 11];

    for direction in directions {
      let mut position: i8 = from as i8;

      loop {
        position += direction;

        let square = self.pieces[position as usize];

        if square == OUTSIDE {
          break;
        }

        if square == EMPTY {
          piece_moves.push(Move::normal(from, position as usize));
        } else {
          if (square < 0 && matches!(color, Color::White)) || (square > 0 && matches!(color, Color::Black)) {
            piece_moves.push(Move::capture(from, position as usize, square));
          }

          break;
        }
      }
    }

    piece_moves
  }

  fn pawn_moves(&self, from: usize, color: Color) -> Vec<Move> {
    let mut piece_moves: Vec<Move> = Vec::with_capacity(12);

    let in_second_rank = (81..=88).contains(&from);
    let in_seventh_rank = (31..=38).contains(&from);

    if matches!(color, Color::White) {
      let one_up = from - 10;
      let one_up_square = self.pieces[one_up];

      let up_left = from - 11;
      let up_left_square = self.pieces[up_left];

      let up_right = from - 9;
      let up_right_square = self.pieces[up_right];

      if self.meta.en_passant_index == Some(up_left) {
        piece_moves.push(Move::en_passant(from, up_left));
      }

      if self.meta.en_passant_index == Some(up_right) {
        piece_moves.push(Move::en_passant(from, up_right));
      }

      if in_seventh_rank {
        if one_up_square == EMPTY {
          piece_moves.push(Move::promotion(from, one_up, QUEEN));
          piece_moves.push(Move::promotion(from, one_up, KNIGHT));
          piece_moves.push(Move::promotion(from, one_up, ROOK));
          piece_moves.push(Move::promotion(from, one_up, BISHOP));
        }

        if let Some(Color::Black) = get_color(up_left_square) {
          piece_moves.push(Move::promotion_with_capture(from, up_left, QUEEN, up_left_square));
          piece_moves.push(Move::promotion_with_capture(from, up_left, KNIGHT, up_left_square));
          piece_moves.push(Move::promotion_with_capture(from, up_left, ROOK, up_left_square));
          piece_moves.push(Move::promotion_with_capture(from, up_left, BISHOP, up_left_square));
        }

        if let Some(Color::Black) = get_color(up_right_square) {
          piece_moves.push(Move::promotion_with_capture(from, up_right, QUEEN, up_right_square));
          piece_moves.push(Move::promotion_with_capture(from, up_right, KNIGHT, up_right_square));
          piece_moves.push(Move::promotion_with_capture(from, up_right, ROOK, up_right_square));
          piece_moves.push(Move::promotion_with_capture(from, up_right, BISHOP, up_right_square));
        }
      } else {
        if one_up_square == EMPTY {
          piece_moves.push(Move::pawn_push(from, one_up));

          let in_first_row = (81..=88).contains(&from);

          if in_first_row {
            let two_up = from - 20;
            let two_up_square = self.pieces[two_up];

            if two_up_square == EMPTY {
              piece_moves.push(Move::double_pawn_push(from, two_up));
            }
          }
        }

        if let Some(Color::Black) = get_color(up_left_square) {
          piece_moves.push(Move::capture(from, up_left, up_left_square));
        }

        if let Some(Color::Black) = get_color(up_right_square) {
          piece_moves.push(Move::capture(from, up_right, up_right_square));
        }
      }
    } else {
      let one_down = from + 10;
      let one_down_square = self.pieces[one_down];

      let down_left = from + 9;
      let down_left_square = self.pieces[down_left];

      let down_right = from + 11;
      let down_right_square = self.pieces[down_right];

      if self.meta.en_passant_index == Some(down_left) {
        piece_moves.push(Move::en_passant(from, down_left));
      }

      if self.meta.en_passant_index == Some(down_right) {
        piece_moves.push(Move::en_passant(from, down_right));
      }

      if in_second_rank {
        if one_down_square == EMPTY {
          piece_moves.push(Move::promotion(from, one_down, -QUEEN));
          piece_moves.push(Move::promotion(from, one_down, -KNIGHT));
          piece_moves.push(Move::promotion(from, one_down, -ROOK));
          piece_moves.push(Move::promotion(from, one_down, -BISHOP));
        }

        if let Some(Color::White) = get_color(down_left_square) {
          piece_moves.push(Move::promotion_with_capture(from, down_left, -QUEEN, down_left_square));
          piece_moves.push(Move::promotion_with_capture(from, down_left, -KNIGHT, down_left_square));
          piece_moves.push(Move::promotion_with_capture(from, down_left, -ROOK, down_left_square));
          piece_moves.push(Move::promotion_with_capture(from, down_left, -BISHOP, down_left_square));
        }

        if let Some(Color::White) = get_color(down_left_square) {
          piece_moves.push(Move::promotion_with_capture(from, down_right, -QUEEN, down_left_square));
          piece_moves.push(Move::promotion_with_capture(from, down_right, -KNIGHT, down_left_square));
          piece_moves.push(Move::promotion_with_capture(from, down_right, -ROOK, down_left_square));
          piece_moves.push(Move::promotion_with_capture(from, down_right, -BISHOP, down_left_square));
        }
      } else {
        if one_down_square == EMPTY {
          piece_moves.push(Move::pawn_push(from, one_down));

          if in_seventh_rank {
            let two_down = from + 20;
            let two_down_square = self.pieces[two_down];

            if two_down_square == EMPTY {
              piece_moves.push(Move::double_pawn_push(from, two_down));
            }
          }
        }

        if let Some(Color::White) = get_color(down_left_square) {
          piece_moves.push(Move::capture(from, down_left, down_left_square));
        }

        if let Some(Color::White) = get_color(down_right_square) {
          piece_moves.push(Move::capture(from, down_right, down_right_square));
        }
      }
    }

    piece_moves
  }

  fn generate_moves(&self) -> Vec<Move> {
    let mut piece_moves: Vec<Move> = Vec::new();

    for from in BOARD_INDICES {
      let square = self.pieces[from as usize];

      if let Some(square_color) = get_color(square) {
        if square_color != self.side_to_move {
          continue;
        }

        match square.abs() {
          PAWN => piece_moves.append(&mut self.pawn_moves(from as usize, square_color)),
          KNIGHT => piece_moves.append(&mut self.knight_moves(from as usize, square_color)),
          BISHOP => piece_moves.append(&mut self.bishop_moves(from as usize, square_color)),
          ROOK => piece_moves.append(&mut self.rook_moves(from as usize, square_color)),
          QUEEN => piece_moves.append(&mut self.queen_moves(from as usize, square_color)),
          KING => piece_moves.append(&mut self.king_moves(from as usize, square_color)),
          _ => {
            panic!("This shouldn't panic");
          }
        }
      }
    }

    piece_moves
  }

  fn make_move(&mut self, chess_move: &Move) {
    todo!()
  }

  fn undo_move(&mut self, chess_move: &Move) {
    todo!()
  }

  fn to_fen(&self) -> String {
    todo!()
  }
}

#[cfg(test)]
mod tests {
  use super::Board;

  #[test]
  fn perft_test() {
    // https://www.chessprogramming.org/Perft_Results
    fn test_fen<const N: usize>(fen: &str, depth_nodes: [u64; N]) {
      let mut board = Board::from_fen(fen);

      // TODO: include capture, en passant, castle, promotion, check, discovery check, double check and checkmate counts
      fn perft(depth: usize, board: &mut Board) -> u64 {
        // https://www.chessprogramming.org/Perft

        let mut nodes = 0;

        if depth == 0 {
          return 1;
        }

        for chess_move in board.generate_moves() {
          board.make_move(&chess_move);
          nodes += perft(depth - 1, board);
          board.undo_move(&chess_move);
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
}
