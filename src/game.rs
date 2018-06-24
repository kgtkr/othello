#[derive(PartialEq, Debug, Clone, Copy)]
enum Piece {
  Black,
  While,
}

type Cell = Option<Piece>;
type Board = [[Cell; 8]; 8];

#[derive(PartialEq, Debug, Clone)]
enum GameResult {
  Next(Piece),
  Pass(Piece),
  IndexOutOf,
  Already,
  ZeroReverse,
  End(usize, usize),
}

struct Game {
  board: Board,
  next_player: Piece,
}

impl Game {
  const size: usize = 8;

  pub fn new() -> Game {
    let board = [
      [None; 8],
      [None; 8],
      [None; 8],
      [
        None,
        None,
        None,
        Some(Piece::While),
        Some(Piece::Black),
        None,
        None,
        None,
      ],
      [
        None,
        None,
        None,
        Some(Piece::Black),
        Some(Piece::While),
        None,
        None,
        None,
      ],
      [None; 8],
      [None; 8],
      [None; 8],
    ];
    Game {
      board: board,
      next_player: Piece::Black,
    }
  }

  pub fn next() -> GameResult {
    GameResult::Next(Piece::Black)
  }
}
