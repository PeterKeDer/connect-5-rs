use std::io;
use connect_5_rs::{Game, GameState, GameStepError, GameSpot, GameSide, Point};

fn main() {
    let mut game = Game::new(15);
    print_board(&game);

    while let GameState::Normal = game.state() {
        let point = get_point(&game);
        game.add_step(point).expect("This should never happen.");
        print_board(&game);
    }

    match game.state() {
        GameState::BoardFull => {
            println!("Tie: board is full.");
        },
        GameState::Finished { winner_side, points: _ } => {
            println!("Winner: {}", match winner_side {
                GameSide::Black => "Black",
                GameSide::White => "White",
            });
        },
        GameState::Normal => (),
    }
}

/// Get a point from user input to place on the board.
/// It is guaranteed that the point is valid for step.
fn get_point(game: &Game) -> Point {
    loop {
        println!("Enter coordinate, in format: x y");

        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let nums: Vec<Option<usize>> = input
            .split_whitespace()
            .map(|n_str| n_str.parse().ok())
            .collect();

        // Match exactly two parsed number elements to create point
        let point = match &nums[..] {
            &[Some(x), Some(y)] => Point::new(x, y),
            _ => {
                // Otherwise, invalid input
                println!("Please enter two valid numbers separated by whitespace.");
                continue;
            },
        };

        // Validate whether point is valid step
        match game.validate_step(&point) {
            Ok(_) => return point,
            Err(GameStepError::PointTaken) => {
                println!("The point is already taken by another piece, try again.");
                continue;
            },
            Err(GameStepError::InvalidPoint) => {
                println!("Please choose a point that is on the board.");
                continue;
            },
        };
    }
}

/// Print the formatted board.
fn print_board(game: &Game) {
    let x_coords: String = (0..game.size())
        .map(|x| format!("{:2} ", x))
        .collect();
    println!("     {}", x_coords);

    let bars: String = (0..game.size())
        .map(|_| "---")
        .collect();
    println!("     {}", bars);

    for y in 0..game.size() {
        print!("{:2} | ", y);
        for x in 0..game.size() {
            let chr = match game.spot(&Point::new(x, y)) {
                GameSpot::Empty => '.',
                GameSpot::Taken(GameSide::Black) => 'B',
                GameSpot::Taken(GameSide::White) => 'W',
            };
            print!(" {} ", chr);
        }
        println!();
    }
}
