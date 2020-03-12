use std::error;
use std::fmt;

/// A point on a game board.
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

/// The side of a player in a game, either black or white.
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum GameSide {
    Black,
    White,
}

impl GameSide {
    /// Return the opposite of the side.
    fn toggle(&self) -> GameSide {
        match self {
            GameSide::Black => GameSide::White,
            GameSide::White => GameSide::Black,
        }
    }
}

/// Represent the state of a single point on the board.
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum GameSpot {
    /// An empty spot on the board.
    Empty,

    /// A spot on the board taken by a piece from a side.
    Taken(GameSide),
}

impl GameSpot {
    /// Return true if the spot is empty, otherwise false.
    pub fn is_empty(&self) -> bool {
        match self {
            GameSpot::Empty => true,
            _ => false,
        }
    }
}

/// An error that is caused by adding an invalid point as game step.
#[derive(Debug)]
pub enum GameStepError {
    /// The point is outside of the valid board coordinates.
    InvalidPoint,

    /// The point is already taken by a piece on the board.
    PointTaken,
}

impl fmt::Display for GameStepError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameStepError::InvalidPoint => write!(f, "invalid point on the board"),
            GameStepError::PointTaken => write!(f, "point is already taken on the board"),
        }
    }
}

impl error::Error for GameStepError {}

/// The state of a game.
pub enum GameState {
    /// The state representing that the game is still ongoing.
    Normal,

    /// The state representing that the game is finished with a winner.
    Finished {
        /// The side of the winner.
        winner_side: GameSide,
        /// The consecutive points from the winner that resulted a victory.
        points: Vec<Point>,
    },

    /// The state representing that the game board is full, resulting in a tie.
    BoardFull,
}

/// The board state of a game.
struct GameBoard {
    /// The size of the board, representing both the width and height.
    size: usize,
    /// A vector of length size, containing vectors of length size that contains spots.
    /// A specific spot at `Point(x, y)` is accessed by `spots[x][y]`
    spots: Vec<Vec<GameSpot>>,
    /// A vector containing all the points on the game board.
    points: Vec<Point>,
    /// Cache of possible lines where consecutive pieces may result in victory.
    lines: Vec<Vec<Point>>,
}

impl GameBoard {
    /// Create a new board with size, filled with empty spots.
    fn new(size: usize) -> GameBoard {
        let spots = vec![vec![GameSpot::Empty; size]; size];
        let lines = GameBoard::init_lines(size);
        let points = (0..size)
            .flat_map(|y| (0..size).map(move |x| Point::new(x, y)))
            .collect();

        GameBoard {
            size,
            spots,
            lines,
            points,
        }
    }

    /// Initialize lines given board size.
    ///
    /// Create a vector of all possible straight lines, consisting of consecutive points,
    /// inside a board with given size.
    fn init_lines(size: usize) -> Vec<Vec<Point>> {
        let mut lines: Vec<Vec<Point>> = vec![];

        for i in 0..size {
            // Horizontal lines
            lines.push((0..size).map(|j| Point::new(i, j)).collect());

            // Vertical lines
            lines.push((0..size).map(|j| Point::new(j, i)).collect());

            // Slope down lines
            lines.push((0..size - i).map(|j| Point::new(i + j, j)).collect());
            if i > 0 {
                lines.push((0..size - i).map(|j| Point::new(j, i + j)).collect());
            }

            // Slope up lines
            lines.push((0..i + 1).map(|j| Point::new(i - j, j)).collect());
            if i > 0 {
                lines.push((0..size - i).map(|j| Point::new(i + j, size - j - 1)).collect());
            }
        }

        lines
    }

    /// Return true if the point is within bounds of the board, otherwise false.
    fn is_valid(&self, point: &Point) -> bool {
        let range = 0..self.size;
        range.contains(&point.x) && range.contains(&point.y)
    }

    /// Get the spot at point on the board.
    fn get_spot(&self, point: &Point) -> GameSpot {
        self.spots[point.x][point.y]
    }

    /// Set the spot at point on the board.
    fn set_spot(&mut self, point: &Point, spot: GameSpot) {
        self.spots[point.x][point.y] = spot;
    }

    /// Return true if the all spots on the board are taken, otherwise false.
    fn is_full(&self) -> bool {
        // Return false if there is an empty spot
        !self.spots.iter().any(|vec| vec.iter().any(GameSpot::is_empty))
    }
}

/// A Connect 5 game.
pub struct Game {
    /// The board of the game.
    board: GameBoard,
    /// A vector containing the points of all the steps taken in the game,
    /// with order from earliest to latest.
    steps: Vec<Point>,
    /// The current side of the game.
    side: GameSide,
    /// The state of the game.
    state: GameState,
}

// Initializers
impl Game {
    /// Create a new game from size.
    pub fn new(size: usize) -> Game {
        let board = GameBoard::new(size);
        let steps = vec![];
        let side = GameSide::Black;

        Game {
            board,
            steps,
            side,
            state: GameState::Normal,
        }
    }

    /// Create a game from given size and steps.
    pub fn from_steps(size: usize, steps: &[Point]) -> Result<Game, GameStepError> {
        let mut game = Game::new(size);

        for point in steps {
            // Try to add each step. Return early if there is an error
            game.add_step(point.clone())?;
        }

        Ok(game)
    }
}

// Public methods
impl Game {
    /// Get the size of the board.
    pub fn size(&self) -> usize {
        self.board.size
    }

    /// Get the current state of the game, which was computed when added last step.
    pub fn state(&self) -> &GameState {
        &self.state
    }

    /// Get the spot at a point on the game board.
    pub fn spot(&self, point: &Point) -> GameSpot {
        self.board.get_spot(point)
    }

    /// Return an iterator that iterates over all steps of this game and their side from the start.
    pub fn iter_steps(&self) -> impl Iterator<Item=(&GameSide, &Point)> {
        [GameSide::Black, GameSide::White].iter()
            .cycle()
            .zip(self.steps.iter())
    }

    /// Return an iterator that iterates over all the points on the board, row by row.
    pub fn iter_points(&self) -> impl Iterator<Item=&Point> {
        self.board.points.iter()
    }

    /// Validate whether can add step at point.
    /// Return a result with error of type `GameStepError` if the step is invalid.
    pub fn validate_step(&self, point: &Point) -> Result<(), GameStepError> {
        if !self.board.is_valid(point) {
            Err(GameStepError::InvalidPoint)
        } else if !self.board.get_spot(point).is_empty() {
            Err(GameStepError::PointTaken)
        } else {
            Ok(())
        }
    }

    /// Add a step to the game.
    /// If the step is invalid, do not add the step and
    /// return a result with error of type `GameStepError`.
    pub fn add_step(&mut self, point: Point) -> Result<(), GameStepError> {
        self.validate_step(&point)?;

        self.board.set_spot(&point, GameSpot::Taken(self.side));
        self.side = self.side.toggle();
        self.steps.push(point);

        self.update_state();

        Ok(())
    }
}

// Private methods
impl Game {
    /// Update the state of the game. Should be called when a step is added/removed.
    fn update_state(&mut self) {
        self.state =
        if let Some((winner_side, points)) = self.compute_winner() {
            GameState::Finished {
                winner_side,
                points,
            }
        } else if self.board.is_full() {
            GameState::BoardFull
        } else  {
            GameState::Normal
        };
    }

    /// Compute and return the winner side and points of the game, if any.
    fn compute_winner(&self) -> Option<(GameSide, Vec<Point>)> {
        for line in &self.board.lines {
            // For each line, check whether there are repeats 5 in a row
            let mut prev_spot = GameSpot::Empty;
            let mut consec = Vec::with_capacity(5);

            for point in line {
                let spot = self.board.get_spot(&point);

                if let GameSpot::Taken(side) = spot {
                    if spot == prev_spot {
                        // Consecutive side
                        consec.push(point);

                        if consec.len() >= 5 {
                            let points = consec.iter().map(|pt| **pt).collect();
                            return Some((side, points));
                        }
                    } else {
                        // Different side
                        consec.clear();
                        consec.push(point);
                    }
                } else {
                    // Current spot empty
                    consec.clear();
                }
                prev_spot = spot;
            }
        }

        None
    }
}
