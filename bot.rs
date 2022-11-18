// Hello! Need help getting started? Check out the Tic-Tac-Toe tutorial:
// 👉 https://github.com/zilch/zilch/blob/main/docs/games/tic-tac-toe/tutorial.md

#[derive(Debug)]
pub struct Config {
    pub player: String,
    pub color: String,
    pub turn_time_limit: usize,
    pub game_time_limit: usize,
}

pub struct Bot {}

impl Bot {
    pub fn new(_config: Config) -> Bot {
        // 👇 Get started by uncommenting the next line then restarting the game
        // println!("Hello World! {:?}", _config);
        Bot {}
    }

    pub fn next_move(&self, board: Vec<Vec<String>>) -> (usize, usize) {
        println!("{:?}", board); // 3x3 array with values "x" or "o" or "empty"

        // Return the spot you'd like to move here.
        // x should be an integer between 0 and 2
        // y should be an integer between 0 and 2
        let x = 0;
        let y = 0;
        (x, y)
    }
}
