const PAWN: i8 = 1;
const KNIGHT: i8 = 2;
const BISHOP: i8 = 3;
const ROOK: i8 = 4;
const QUEEN: i8 = 5;
const KING: i8 = 6;
const EMPTY: i8 = 7;
const OUTSIDE: i8 = 0;

// TODO: check if changing all usizes to i8 and then casting to usize increases performance
// TODO: check if storing information more densely (storing multiple things inside an i8) increases performance
// TODO: check if make-unmake is faster than copy-make

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

#[derive(Clone, Debug, PartialEq)]
enum CastlingSide {
  WhiteKing,
  WhiteQueen,
  BlackKing,
  BlackQueen,
}

#[derive(Clone, Debug, PartialEq)]
enum MoveResult {
  Legal,
  Illegal,
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
    captured_index: usize,
    captured_piece: i8,
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

  fn en_passant(from: usize, to: usize, captured_index: usize, captured_piece: i8) -> Move {
    Move::EnPassant {
      from,
      to,
      captured_index,
      captured_piece,
    }
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
    match *self {
      Move::Normal { from, to }
      | Move::Capture {
        from,
        to,
        captured_piece: _,
      }
      | Move::PawnPush { from, to }
      | Move::DoublePawnPush { from, to }
      | Move::EnPassant {
        from,
        to,
        captured_index: _,
        captured_piece: _,
      } => index_to_square(from) + &index_to_square(to),
      Move::Promotion {
        from,
        to,
        selected_piece,
      }
      | Move::PromotionWithCapture {
        from,
        to,
        selected_piece,
        captured_piece: _,
      } => {
        index_to_square(from)
          + &index_to_square(to)
          + match selected_piece.abs() {
            KNIGHT => "n",
            BISHOP => "b",
            ROOK => "r",
            QUEEN => "q",
            _ => panic!("This shouldn't panic"),
          }
      }
      Move::Castling(CastlingSide::WhiteKing) => "e1g1".to_string(),
      Move::Castling(CastlingSide::WhiteQueen) => "e1c1".to_string(),
      Move::Castling(CastlingSide::BlackKing) => "e8g8".to_string(),
      Move::Castling(CastlingSide::BlackQueen) => "e8c8".to_string(),
    }
  }
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
  undo_list: Vec<BoardMeta>,

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
  fn king_moves(&self, index: usize, color: Color) -> Vec<Move> {
    let mut piece_moves: Vec<Move> = Vec::with_capacity(8);

    let positions: [usize; 8] = [
      index - 11,
      index - 10,
      index - 9,
      index - 1,
      index + 1,
      index + 9,
      index + 10,
      index + 11,
    ];

    for position in positions {
      let square = self.pieces[position];
      if square == OUTSIDE {
        continue;
      }
      if square == EMPTY {
        piece_moves.push(Move::normal(index, position));
      } else if (square < 0 && matches!(color, Color::White)) || (square > 0 && matches!(color, Color::Black)) {
        piece_moves.push(Move::capture(index, position, square));
      }
    }

    piece_moves
  }

  fn knight_moves(&self, index: usize, color: Color) -> Vec<Move> {
    let mut piece_moves: Vec<Move> = Vec::with_capacity(8);

    let positions: [usize; 8] = [
      index - 21,
      index - 19,
      index - 12,
      index - 8,
      index + 8,
      index + 12,
      index + 19,
      index + 21,
    ];

    for position in positions {
      let square = self.pieces[position];
      if square == OUTSIDE {
        continue;
      }
      if square == EMPTY {
        piece_moves.push(Move::normal(index, position));
      } else if (square < 0 && matches!(color, Color::White)) || (square > 0 && matches!(color, Color::Black)) {
        piece_moves.push(Move::capture(index, position, square));
      }
    }

    piece_moves
  }

  fn bishop_moves(&self, index: usize, color: Color) -> Vec<Move> {
    let mut piece_moves: Vec<Move> = Vec::with_capacity(13);

    let directions: [i8; 4] = [-11, -9, 9, 11];

    for direction in directions {
      let mut position: i8 = index as i8;

      loop {
        position += direction;

        let square = self.pieces[position as usize];

        if square == OUTSIDE {
          break;
        }

        if square == EMPTY {
          piece_moves.push(Move::normal(index, position as usize));
        } else {
          if (square < 0 && matches!(color, Color::White)) || (square > 0 && matches!(color, Color::Black)) {
            piece_moves.push(Move::capture(index, position as usize, square));
          }

          break;
        }
      }
    }

    piece_moves
  }

  fn rook_moves(&self, index: usize, color: Color) -> Vec<Move> {
    let mut piece_moves: Vec<Move> = Vec::with_capacity(14);

    let directions: [i8; 4] = [-10, -1, 1, 10];

    for direction in directions {
      let mut position: i8 = index as i8;

      loop {
        position += direction;

        let square = self.pieces[position as usize];

        if square == OUTSIDE {
          break;
        }

        if square == EMPTY {
          piece_moves.push(Move::normal(index, position as usize));
        } else {
          if (square < 0 && matches!(color, Color::White)) || (square > 0 && matches!(color, Color::Black)) {
            piece_moves.push(Move::capture(index, position as usize, square));
          }

          break;
        }
      }
    }

    piece_moves
  }

  fn queen_moves(&self, index: usize, color: Color) -> Vec<Move> {
    let mut piece_moves: Vec<Move> = Vec::with_capacity(27);

    let directions: [i8; 8] = [-11, -10, -9, -1, 1, 9, 10, 11];

    for direction in directions {
      let mut position: i8 = index as i8;

      loop {
        position += direction;

        let square = self.pieces[position as usize];

        if square == OUTSIDE {
          break;
        }

        if square == EMPTY {
          piece_moves.push(Move::normal(index, position as usize));
        } else {
          if (square < 0 && matches!(color, Color::White)) || (square > 0 && matches!(color, Color::Black)) {
            piece_moves.push(Move::capture(index, position as usize, square));
          }

          break;
        }
      }
    }

    piece_moves
  }

  fn pawn_moves(&self, index: usize, color: Color) -> Vec<Move> {
    let mut piece_moves: Vec<Move> = Vec::with_capacity(12);

    let in_second_rank = (81..=88).contains(&index);
    let in_seventh_rank = (31..=38).contains(&index);

    let one_left = index - 1;
    let one_left_square = self.pieces[one_left];

    let one_right = index + 1;
    let one_right_square = self.pieces[one_right];

    if matches!(color, Color::White) {
      let one_up = index - 10;
      let one_up_square = self.pieces[one_up];

      let up_left = index - 11;
      let up_left_square = self.pieces[up_left];

      let up_right = index - 9;
      let up_right_square = self.pieces[up_right];

      if self.meta.en_passant_index == Some(up_left) {
        piece_moves.push(Move::en_passant(index, up_left, one_left, one_left_square));
      }

      if self.meta.en_passant_index == Some(up_right) {
        piece_moves.push(Move::en_passant(index, up_right, one_right, one_right_square));
      }

      if in_seventh_rank {
        if one_up_square == EMPTY {
          piece_moves.push(Move::promotion(index, one_up, QUEEN));
          piece_moves.push(Move::promotion(index, one_up, KNIGHT));
          piece_moves.push(Move::promotion(index, one_up, ROOK));
          piece_moves.push(Move::promotion(index, one_up, BISHOP));
        }

        if let Some(Color::Black) = get_color(up_left_square) {
          piece_moves.push(Move::promotion_with_capture(index, up_left, QUEEN, up_left_square));
          piece_moves.push(Move::promotion_with_capture(index, up_left, KNIGHT, up_left_square));
          piece_moves.push(Move::promotion_with_capture(index, up_left, ROOK, up_left_square));
          piece_moves.push(Move::promotion_with_capture(index, up_left, BISHOP, up_left_square));
        }

        if let Some(Color::Black) = get_color(up_right_square) {
          piece_moves.push(Move::promotion_with_capture(index, up_right, QUEEN, up_right_square));
          piece_moves.push(Move::promotion_with_capture(index, up_right, KNIGHT, up_right_square));
          piece_moves.push(Move::promotion_with_capture(index, up_right, ROOK, up_right_square));
          piece_moves.push(Move::promotion_with_capture(index, up_right, BISHOP, up_right_square));
        }
      } else {
        if one_up_square == EMPTY {
          piece_moves.push(Move::pawn_push(index, one_up));

          let in_first_row = (81..=88).contains(&index);

          if in_first_row {
            let two_up = index - 20;
            let two_up_square = self.pieces[two_up];

            if two_up_square == EMPTY {
              piece_moves.push(Move::double_pawn_push(index, two_up));
            }
          }
        }

        if let Some(Color::Black) = get_color(up_left_square) {
          piece_moves.push(Move::capture(index, up_left, up_left_square));
        }

        if let Some(Color::Black) = get_color(up_right_square) {
          piece_moves.push(Move::capture(index, up_right, up_right_square));
        }
      }
    } else {
      let one_down = index + 10;
      let one_down_square = self.pieces[one_down];

      let down_left = index + 9;
      let down_left_square = self.pieces[down_left];

      let down_right = index + 11;
      let down_right_square = self.pieces[down_right];

      if self.meta.en_passant_index == Some(down_left) {
        piece_moves.push(Move::en_passant(index, down_left, one_left, one_left_square));
      }

      if self.meta.en_passant_index == Some(down_right) {
        piece_moves.push(Move::en_passant(index, down_right, one_right, one_right_square));
      }

      if in_second_rank {
        if one_down_square == EMPTY {
          piece_moves.push(Move::promotion(index, one_down, -QUEEN));
          piece_moves.push(Move::promotion(index, one_down, -KNIGHT));
          piece_moves.push(Move::promotion(index, one_down, -ROOK));
          piece_moves.push(Move::promotion(index, one_down, -BISHOP));
        }

        if let Some(Color::White) = get_color(down_left_square) {
          piece_moves.push(Move::promotion_with_capture(index, down_left, -QUEEN, down_left_square));
          piece_moves.push(Move::promotion_with_capture(index, down_left, -KNIGHT, down_left_square));
          piece_moves.push(Move::promotion_with_capture(index, down_left, -ROOK, down_left_square));
          piece_moves.push(Move::promotion_with_capture(index, down_left, -BISHOP, down_left_square));
        }

        if let Some(Color::White) = get_color(down_left_square) {
          piece_moves.push(Move::promotion_with_capture(index, down_right, -QUEEN, down_left_square));
          piece_moves.push(Move::promotion_with_capture(index, down_right, -KNIGHT, down_left_square));
          piece_moves.push(Move::promotion_with_capture(index, down_right, -ROOK, down_left_square));
          piece_moves.push(Move::promotion_with_capture(index, down_right, -BISHOP, down_left_square));
        }
      } else {
        if one_down_square == EMPTY {
          piece_moves.push(Move::pawn_push(index, one_down));

          if in_seventh_rank {
            let two_down = index + 20;
            let two_down_square = self.pieces[two_down];

            if two_down_square == EMPTY {
              piece_moves.push(Move::double_pawn_push(index, two_down));
            }
          }
        }

        if let Some(Color::White) = get_color(down_left_square) {
          piece_moves.push(Move::capture(index, down_left, down_left_square));
        }

        if let Some(Color::White) = get_color(down_right_square) {
          piece_moves.push(Move::capture(index, down_right, down_right_square));
        }
      }
    }

    piece_moves
  }

  // It is slower than pseudo_legal_moves because it doesn't check if the king is left in check
  fn legal_moves(&mut self) -> Vec<Move> {
    self
      .pseudo_legal_moves()
      .into_iter()
      .filter(|chess_move| {
        let re = self.make_move(chess_move);
        self.undo_move(chess_move);

        re != MoveResult::Illegal
      })
      .collect()
  }

  // Generates pseudo-legal moves. It means that it could leave its own king in check.
  // Includes castling (it also can be pseudo-legal)
  fn pseudo_legal_moves(&self) -> Vec<Move> {
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

    if self.side_to_move == Color::White {
      // TODO: self.square_is_attacked(95, Color::White) is called 2 times

      if self.meta.white_king_castle
        && self.pieces[96] == EMPTY
        && self.pieces[97] == EMPTY
        && !self.square_is_attacked(95, Color::White)
        && !self.square_is_attacked(96, Color::White)
      {
        piece_moves.push(Move::castling(CastlingSide::WhiteKing));
      }
      if self.meta.white_queen_castle
        && self.pieces[94] == EMPTY
        && self.pieces[93] == EMPTY
        && !self.square_is_attacked(95, Color::White)
        && !self.square_is_attacked(94, Color::White)
      {
        piece_moves.push(Move::castling(CastlingSide::WhiteQueen));
      }
    } else {
      if self.meta.black_king_castle
        && self.pieces[26] == EMPTY
        && self.pieces[27] == EMPTY
        && !self.square_is_attacked(25, Color::Black)
        && !self.square_is_attacked(26, Color::Black)
      {
        piece_moves.push(Move::castling(CastlingSide::BlackKing));
      }
      if self.meta.black_queen_castle
        && self.pieces[24] == EMPTY
        && self.pieces[23] == EMPTY
        && !self.square_is_attacked(25, Color::Black)
        && !self.square_is_attacked(24, Color::Black)
      {
        piece_moves.push(Move::castling(CastlingSide::BlackQueen));
      }
    }

    piece_moves
  }

  // Updates meta information: castling rights, en passant square, and the halfmove clock
  fn update_meta(&mut self, chess_move: &Move) {
    // save the current meta information
    self.undo_list.push(self.meta.clone());

    match *chess_move {
      Move::Normal { from: _, to: _ } => {
        self.meta.en_passant_index = None;
        self.meta.halfmove_clock += 1;
      }
      Move::PawnPush { from: _, to: _ }
      | Move::Capture {
        from: _,
        to: _,
        captured_piece: _,
      }
      | Move::EnPassant {
        from: _,
        to: _,
        captured_index: _,
        captured_piece: _,
      }
      | Move::Promotion {
        from: _,
        to: _,
        selected_piece: _,
      }
      | Move::PromotionWithCapture {
        from: _,
        to: _,
        selected_piece: _,
        captured_piece: _,
      } => {
        self.meta.en_passant_index = None;
        self.meta.halfmove_clock = 0;
      }
      Move::DoublePawnPush { from, to } => {
        self.meta.en_passant_index = Some((from + to) / 2);
        self.meta.halfmove_clock = 0;
      }
      Move::Castling(_) => {
        self.meta.en_passant_index = None;
        self.meta.halfmove_clock += 1;
      }
    }

    match *chess_move {
      Move::Normal { from, to: _ } => {
        if self.pieces[from] == KING {
          self.meta.white_king_castle = false;
          self.meta.white_queen_castle = false;
        } else if self.pieces[from] == -KING {
          self.meta.black_king_castle = false;
          self.meta.black_queen_castle = false;
        } else if from == 98 {
          self.meta.white_king_castle = false;
        } else if from == 91 {
          self.meta.white_queen_castle = false;
        } else if from == 28 {
          self.meta.black_king_castle = false;
        } else if from == 21 {
          self.meta.black_queen_castle = false;
        }
      }
      Move::Capture {
        from,
        to,
        captured_piece: _,
      } => {
        if from == 98 || to == 98 {
          self.meta.white_king_castle = false;
        } else if from == 91 || to == 91 {
          self.meta.white_queen_castle = false;
        } else if from == 28 || to == 28 {
          self.meta.black_king_castle = false;
        } else if from == 21 || to == 21 {
          self.meta.black_queen_castle = false;
        }
      }
      Move::PromotionWithCapture {
        from: _,
        to,
        selected_piece: _,
        captured_piece: _,
      } => {
        if to == 98 {
          self.meta.white_king_castle = false;
        } else if to == 91 {
          self.meta.white_queen_castle = false;
        } else if to == 28 {
          self.meta.black_king_castle = false;
        } else if to == 21 {
          self.meta.black_queen_castle = false;
        }
      }
      Move::Castling(CastlingSide::WhiteKing) | Move::Castling(CastlingSide::WhiteQueen) => {
        self.meta.white_king_castle = false;
        self.meta.white_queen_castle = false;
      }
      Move::Castling(CastlingSide::BlackKing) | Move::Castling(CastlingSide::BlackQueen) => {
        self.meta.black_king_castle = false;
        self.meta.black_queen_castle = false;
      }
      _ => {}
    }
  }

  // Makes a move, which updates the board and meta information.
  // Returns whether the move was legal
  fn make_move(&mut self, chess_move: &Move) -> MoveResult {
    match *chess_move {
      Move::Normal { from, to }
      | Move::Capture {
        from,
        to,
        captured_piece: _,
      }
      | Move::PawnPush { from, to }
      | Move::DoublePawnPush { from, to } => {
        self.pieces[to] = self.pieces[from];
        self.pieces[from] = EMPTY;
      }
      Move::EnPassant {
        from,
        to,
        captured_index,
        captured_piece: _,
      } => {
        self.pieces[to] = self.pieces[from];
        self.pieces[from] = EMPTY;
        self.pieces[captured_index] = EMPTY;
      }
      Move::Promotion {
        from,
        to,
        selected_piece,
      }
      | Move::PromotionWithCapture {
        from,
        to,
        selected_piece,
        captured_piece: _,
      } => {
        self.pieces[to] = selected_piece;
        self.pieces[from] = EMPTY;
      }
      Move::Castling(CastlingSide::WhiteKing) => {
        self.pieces[95] = EMPTY;
        self.pieces[98] = EMPTY;

        self.pieces[96] = ROOK;
        self.pieces[97] = KING;
      }
      Move::Castling(CastlingSide::WhiteQueen) => {
        self.pieces[95] = EMPTY;
        self.pieces[91] = EMPTY;

        self.pieces[94] = ROOK;
        self.pieces[93] = KING;
      }
      Move::Castling(CastlingSide::BlackKing) => {
        self.pieces[25] = EMPTY;
        self.pieces[28] = EMPTY;

        self.pieces[26] = -ROOK;
        self.pieces[27] = -KING;
      }
      Move::Castling(CastlingSide::BlackQueen) => {
        self.pieces[25] = EMPTY;
        self.pieces[21] = EMPTY;

        self.pieces[24] = -ROOK;
        self.pieces[23] = -KING;
      }
    }

    let mut king_is_safe = true;

    for from in BOARD_INDICES {
      let square = self.pieces[from as usize];

      if ((self.side_to_move == Color::White && square == KING) || (self.side_to_move == Color::Black && square == -KING))
        && self.square_is_attacked(from as usize, self.side_to_move.clone())
      {
        king_is_safe = false;
        break;
      }
    }

    self.update_meta(chess_move);
    self.fullmove_counter += 1;
    self.side_to_move = {
      if self.side_to_move == Color::White {
        Color::Black
      } else {
        Color::White
      }
    };

    if king_is_safe {
      MoveResult::Legal
    } else {
      MoveResult::Illegal
    }
  }

  fn undo_move(&mut self, chess_move: &Move) {
    self.meta = self
      .undo_list
      .pop()
      .expect("Couldn't undo a move, because a move wasn't made");

    self.fullmove_counter -= 1;
    self.side_to_move = {
      if self.side_to_move == Color::White {
        Color::Black
      } else {
        Color::White
      }
    };

    match *chess_move {
      Move::Normal { from, to } | Move::PawnPush { from, to } | Move::DoublePawnPush { from, to } => {
        self.pieces[from] = self.pieces[to];
        self.pieces[to] = EMPTY;
      }
      Move::Capture {
        from,
        to,
        captured_piece,
      } => {
        self.pieces[from] = self.pieces[to];
        self.pieces[to] = captured_piece;
      }
      Move::EnPassant {
        from,
        to,
        captured_index,
        captured_piece,
      } => {
        self.pieces[from] = self.pieces[to];
        self.pieces[to] = EMPTY;
        self.pieces[captured_index] = captured_piece;
      }
      Move::Promotion {
        from,
        to,
        selected_piece,
      } => {
        if selected_piece < 0 {
          self.pieces[from] = -PAWN;
        } else {
          self.pieces[from] = PAWN;
        }
        self.pieces[to] = EMPTY;
      }
      Move::PromotionWithCapture {
        from,
        to,
        selected_piece,
        captured_piece,
      } => {
        if selected_piece < 0 {
          self.pieces[from] = -PAWN;
        } else {
          self.pieces[from] = PAWN;
        }
        self.pieces[to] = captured_piece;
      }
      Move::Castling(CastlingSide::WhiteKing) => {
        self.pieces[95] = KING;
        self.pieces[98] = ROOK;

        self.pieces[96] = EMPTY;
        self.pieces[97] = EMPTY;
      }
      Move::Castling(CastlingSide::WhiteQueen) => {
        self.pieces[95] = KING;
        self.pieces[91] = ROOK;

        self.pieces[94] = EMPTY;
        self.pieces[93] = EMPTY;
      }
      Move::Castling(CastlingSide::BlackKing) => {
        self.pieces[25] = -KING;
        self.pieces[28] = -ROOK;

        self.pieces[26] = EMPTY;
        self.pieces[27] = EMPTY;
      }
      Move::Castling(CastlingSide::BlackQueen) => {
        self.pieces[25] = -KING;
        self.pieces[21] = -ROOK;

        self.pieces[24] = EMPTY;
        self.pieces[23] = EMPTY;
      }
    };
  }

  fn in_check(&self) -> bool {
    for from in BOARD_INDICES {
      let square = self.pieces[from as usize];

      if square.abs() == KING && get_color(square).unwrap() == self.side_to_move {
        return self.square_is_attacked(from as usize, self.side_to_move.clone());
      }
    }

    panic!("No king on board");
  }

  // Returns whether a square is attacked by any of the other side's pieces.
  // Note: does not take en passant into consideration (although, it shouldn't really matter).
  fn square_is_attacked(&self, index: usize, defending_side: Color) -> bool {
    // TODO: check if reordering increases performance

    let king_positions: [usize; 8] = [
      index - 11,
      index - 10,
      index - 9,
      index - 1,
      index + 1,
      index + 9,
      index + 10,
      index + 11,
    ];

    if king_positions.iter().any(|&position| {
      let square = self.pieces[position];
      (defending_side == Color::White && square == -KING) || (defending_side == Color::Black && square == KING)
    }) {
      return true;
    }

    let knight_positions: [usize; 8] = [
      index - 21,
      index - 19,
      index - 12,
      index - 8,
      index + 8,
      index + 12,
      index + 19,
      index + 21,
    ];

    if knight_positions.iter().any(|&position| {
      let square = self.pieces[position];
      (defending_side == Color::White && square == -KNIGHT) || (defending_side == Color::Black && square == KNIGHT)
    }) {
      return true;
    }

    let bishop_directions: [i8; 4] = [-11, -9, 9, 11];

    if bishop_directions.iter().any(|&direction| {
      let mut position = index as i8;

      loop {
        position += direction;

        let square = self.pieces[position as usize];

        if square == OUTSIDE {
          break false;
        }

        if square == EMPTY {
          continue;
        }

        break ((square == -BISHOP || square == -QUEEN) && defending_side == Color::White)
          || ((square == BISHOP || square == QUEEN) && defending_side == Color::Black);
      }
    }) {
      return true;
    }

    let rook_directions: [i8; 4] = [-10, -1, 1, 10];

    if rook_directions.iter().any(|&direction| {
      let mut position = index as i8;

      loop {
        position += direction;

        let square = self.pieces[position as usize];

        if square == OUTSIDE {
          break false;
        }

        if square == EMPTY {
          continue;
        }

        break ((square == -ROOK || square == -QUEEN) && defending_side == Color::White)
          || ((square == ROOK || square == QUEEN) && defending_side == Color::Black);
      }
    }) {
      return true;
    }

    if defending_side == Color::White {
      let up_left = index - 11;
      let up_left_square = self.pieces[up_left];

      let up_right = index - 9;
      let up_right_square = self.pieces[up_right];

      if up_left_square == -PAWN || up_right_square == -PAWN {
        return true;
      }
    } else {
      let down_left = index + 9;
      let down_left_square = self.pieces[down_left];

      let down_right = index + 11;
      let down_right_square = self.pieces[down_right];

      if down_left_square == PAWN || down_right_square == PAWN {
        return true;
      }
    }

    false
  }

  fn to_fen(&self) -> String {
    // rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1

    let mut piece_placement: Vec<String> = vec![];

    for y in 0..8 {
      let mut rank = "".to_string();
      let mut empty_in_a_row = 0;

      for x in 0..8 {
        let i = (y + 2) * 10 + (x + 1);
        let square = self.pieces[i];

        if square == EMPTY {
          empty_in_a_row += 1;
          continue;
        }

        if empty_in_a_row != 0 {
          rank += &empty_in_a_row.to_string();
          empty_in_a_row = 0;
        }

        let mut piece_char = match square.abs() {
          PAWN => "P",
          KNIGHT => "N",
          BISHOP => "B",
          ROOK => "R",
          QUEEN => "Q",
          KING => "K",
          _ => panic!("This shouldn't panic"),
        }
        .to_string();

        if square < 0 {
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

    let side_to_move = match self.side_to_move {
      Color::White => "w",
      Color::Black => "b",
    };

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
    if let Some(en_passant_index) = self.meta.en_passant_index {
      en_passant_target_square = index_to_square(en_passant_index);
    }

    let halfmove_clock = self.meta.halfmove_clock.to_string();

    let fullmove_counter = self.fullmove_counter.to_string();

    format!("{piece_placement} {side_to_move} {castling_ability} {en_passant_target_square} {halfmove_clock} {fullmove_counter}")
  }
}

#[cfg(test)]
mod tests {
  use crate::board::{index_to_square, square_to_index};

  use super::Board;

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
    assert_eq!("a1", index_to_square(square_to_index("a1")));
    assert_eq!("a3", index_to_square(square_to_index("a3")));
    assert_eq!("a8", index_to_square(square_to_index("a8")));
    assert_eq!("c1", index_to_square(square_to_index("c1")));
    assert_eq!("c3", index_to_square(square_to_index("c3")));
    assert_eq!("c8", index_to_square(square_to_index("c8")));
    assert_eq!("h1", index_to_square(square_to_index("h1")));
    assert_eq!("h3", index_to_square(square_to_index("h3")));
    assert_eq!("h8", index_to_square(square_to_index("h8")));
  }

  #[test]
  fn move_generation_test() {
    let mut board = Board::from_fen("r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1");
    let mut expected_moves = vec![
      "h2g1", "h2g3", "h2f4", "h2e5", "h2d6", "h2c7", "h2b8", "a1b1", "a1c1", "a1d1", "a1a2", "a1a3", "a1a4", "a1a5", "a1a6",
      "a1a7", "a1a8", "h1f1", "h1g1", "e1d1", "e1f1", "e1d2", "e1e2", "e1f2", "e1g1", "e1c1",
    ];
    let mut generated_moves: Vec<String> = board.legal_moves().iter().map(|m| m.to_fen()).collect();

    expected_moves.sort();
    generated_moves.sort();

    assert_eq!(expected_moves, generated_moves);

    // --------------

    let mut board = Board::from_fen("k4n2/6P1/8/2pP4/8/8/8/4K2R w K c6 0 1");
    let mut expected_moves = vec![
      "d5d6", "g7f8q", "g7f8r", "g7f8b", "g7f8n", "g7g8q", "g7g8r", "g7g8b", "g7g8n", "d5c6", "h1f1", "h1g1", "h1h2", "h1h3",
      "h1h4", "h1h5", "h1h6", "h1h7", "h1h8", "e1d1", "e1f1", "e1d2", "e1e2", "e1f2", "e1g1",
    ];
    let mut generated_moves: Vec<String> = board.legal_moves().iter().map(|m| m.to_fen()).collect();

    expected_moves.sort();
    generated_moves.sort();

    assert_eq!(expected_moves, generated_moves);

    // --------------

    let mut board = Board::default();
    let mut expected_moves = vec![
      "a2a3", "b2b3", "c2c3", "d2d3", "e2e3", "f2f3", "g2g3", "h2h3", "a2a4", "b2b4", "c2c4", "d2d4", "e2e4", "f2f4", "g2g4",
      "h2h4", "b1a3", "b1c3", "g1f3", "g1h3",
    ];
    let mut generated_moves: Vec<String> = board.legal_moves().iter().map(|m| m.to_fen()).collect();

    expected_moves.sort();
    generated_moves.sort();

    assert_eq!(expected_moves, generated_moves);

    // --------------

    let mut board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
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

      // TODO: include capture, en passant, castle, promotion, check, discovery check, double check and checkmate counts
      fn perft(depth: usize, board: &mut Board) -> u64 {
        // https://www.chessprogramming.org/Perft

        let mut nodes = 0;

        if depth == 0 {
          return 1;
        }

        for chess_move in board.pseudo_legal_moves() {
          board.make_move(&chess_move);
          // TODO: fix
          if !board.in_check() {
            nodes += perft(depth - 1, board);
          }
          board.undo_move(&chess_move);
        }

        nodes
      }

      for (i, nodes) in depth_nodes.iter().enumerate() {
        assert_eq!(perft(i, &mut board), *nodes);
      }
    }

    // Position 1
    test_fen::<6>(
      "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
      [1, 20, 400, 8_902, 197_281, 4_865_609],
    );

    // Position 1
    // test_fen::<7>(
    //   "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
    //   [1, 20, 400, 8_902, 197_281, 4_865_609, 119_060_324],
    // );

    // // Position 5
    // test_fen::<6>(
    //   "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
    //   [1, 44, 1_486, 62_379, 2_103_487, 89_941_194],
    // );

    // // Position 6
    // test_fen::<6>(
    //   "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
    //   [1, 46, 2_079, 89_890, 3_894_594, 164_075_551],
    // );
  }
}
