pub struct Grid {
    game_state: [CellState; 9],
}

#[derive(Clone, Copy)]
enum CellState {
    X,
    O,
    Empty,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            game_state: [CellState::Empty; 9],
        }
    }

    /// Draws the game board
    pub fn draw(&self) {
        for y in 0..=2 {
            let mut row = String::new();
            let offset = y * 3;
            for x in 0..=2 {
                let char = match self.game_state[x + offset] {
                    CellState::X => 'X',
                    CellState::O => 'O',
                    CellState::Empty => ' ',
                };

                match x {
                    2 => row.push_str(&format!(" {} ", char)),
                    _ => row.push_str(&format!(" {} │", char)),
                }
            }
            println!("{row}");

            if y != 2 {
                println!("───┼───┼───");
            }
        }
    }
}
