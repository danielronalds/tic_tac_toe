use std::io::Write;

use crossterm::{cursor, style::Print, QueueableCommand, Result};

use rand::Rng;

pub struct Game {
    game_over: bool,
    game_state: [CellState; 9],
    player_position: u8,
    previous_player_position: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellState {
    X,
    O,
    Empty,
    Cursor,
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Game {
    pub fn new() -> Self {
        let game_state = [CellState::Empty; 9];
        Self {
            game_over: false,
            game_state,
            previous_player_position: None,
            player_position: 0,
        }
    }

    pub fn place_player_token(&mut self) {
        self.game_state[self.player_position as usize] = CellState::X;

        self.relocate_player();
    }

    pub fn place_computer_token(&mut self) {
        let mut avaliable_cells = vec![];

        for (i, cell) in self.game_state.iter().enumerate() {
            if cell != &CellState::X && cell != &CellState::O {
                avaliable_cells.push(i);
            }
        }

        if avaliable_cells.is_empty() {
            return;
        }

        let random_index = rand::thread_rng().gen_range(0..avaliable_cells.len());

        let random_position = avaliable_cells[random_index];

        if random_position == self.player_position as usize {
            self.game_state[random_position] = CellState::O;
            self.relocate_player();
        }

        self.game_state[random_position] = CellState::O;
    }

    fn relocate_player(&mut self) {
        // Moving the player out of the way
        self.player_position = 0;
        // Just keep moving the player to the next postion until an empty spot is found
        while self.game_state[self.player_position as usize] != CellState::Empty {
            // If there are no empty spots, end the game
            if self.player_position == 8 {
                self.game_over = true;
                self.player_position = 10;
                return;
            }
            self.player_position += 1;
        }
    }

    pub fn game_over(&self) -> bool {
        self.game_over
    }

    pub fn move_player(&mut self, direction: Direction) {
        let mut new_position = self.player_position;
        match direction {
            Direction::Up => {
                if self.player_position < 3 {
                    return;
                }

                // Moving the player up a row, by subtracting the offset of the grid
                new_position = self.player_position - 3;

                while self.game_state[new_position as usize] != CellState::Empty {
                    // If the position above the player is not available, we need to find the next
                    // available place that is

                    // First we determine what row the player is on
                    let row = new_position / 3;

                    // We then loop through each cell in that row
                    for i in 0..=2 {
                        // We try the current cell
                        let auto_placement = i + row * 3;
                        if self.game_state[auto_placement as usize] == CellState::Empty {
                            // and if the cell is empty we move the player there
                            new_position = auto_placement;
                            break;
                        }
                        // Otherwise we loop through all of the possible cells, and if we cannot
                        // find one that is empty, then the row is full
                    }

                    // We check if the player has already found an empty square before moving them
                    // up another row
                    let position_found = self.game_state[new_position as usize] == CellState::Empty;

                    if (new_position / 3) > 0 && !position_found {
                        new_position -= 3
                    } else {
                        break;
                    }
                }
            }
            Direction::Down => {
                if self.player_position > 5 {
                    return;
                }

                // Moving the player down a row, by adding the offset of the grid
                new_position = self.player_position + 3;

                while self.game_state[new_position as usize] != CellState::Empty {
                    // If the position above the player is not available, we need to find the next
                    // available place that is

                    // First we determine what row the player is on
                    let mut row = new_position / 3;
                    // If the player is in the top corner, then % of the new position will be 0,
                    // even though they have been moved down
                    if row == 0 && new_position - 3 == 0 {
                        row = 1;
                    }

                    // We then loop through each cell in that row
                    for i in 0..=2 {
                        // We try the current cell
                        let auto_placement = i + row * 3;
                        if self.game_state[auto_placement as usize] == CellState::Empty {
                            // and if the cell is empty we move the player there
                            new_position = auto_placement;
                            break;
                        }
                        // Otherwise we loop through all of the possible cells, and if we cannot
                        // find one that is empty, then the row is full
                    }

                    // We check if the player has already found an empty square before moving them
                    // down another row
                    let position_found = self.game_state[new_position as usize] == CellState::Empty;

                    if (new_position / 3) < 2 && !position_found {
                        new_position += 3
                    } else {
                        break;
                    }
                }
            }
            Direction::Left => {
                match new_position {
                    0 | 3 | 6 => (),
                    _ => new_position = new_position.saturating_sub(1),
                }
                while self.game_state[new_position as usize] != CellState::Empty {
                    match new_position {
                        0 | 3 | 6 => break,
                        _ => new_position = new_position.saturating_sub(1),
                    }
                }
            }
            Direction::Right => {
                match self.player_position {
                    2 | 5 | 8 => (),
                    _ => new_position += 1,
                };

                while self.game_state[new_position as usize] != CellState::Empty {
                    match new_position {
                        2 | 5 | 8 => break,
                        _ => new_position += 1,
                    };
                }
            }
        };

        if self.game_state[new_position as usize] == CellState::Empty {
            self.player_position = new_position;
        }
    }

    pub fn apply_game_logic(&mut self) {
        if let Some(i) = self.previous_player_position {
            // if the players previous spot is an X, then they placed it and the cell shouldn't be
            // reset
            if self.game_state[i] != CellState::X && self.game_state[i] != CellState::O {
                self.game_state[i] = CellState::Empty;
            }
        }

        // The player can only be out of bounds if placed there, which typically means the game is
        // over
        if self.player_position < 9 {
            self.game_state[self.player_position as usize] = CellState::Cursor;
            self.previous_player_position = Some(self.player_position as usize);
        }
    }

    /// Draws the game board
    pub fn draw(&mut self, stdout: &mut std::io::Stdout) -> Result<()> {
        for y in 0..=2 {
            let mut board = String::new();
            let offset = y * 3;
            for x in 0..=2 {
                let char = match self.game_state[x + offset] {
                    CellState::X => 'X',
                    CellState::O => 'O',
                    CellState::Empty => ' ',
                    CellState::Cursor => '*',
                };

                match x {
                    2 => board.push_str(&format!(" {} ", char)),
                    _ => board.push_str(&format!(" {} │", char)),
                }
            }
            stdout.queue(Print(board))?;
            stdout.queue(cursor::MoveDown(1))?;
            stdout.queue(cursor::MoveToColumn(0))?;
            stdout.flush()?;

            if y != 2 {
                stdout.queue(Print("───┼───┼───"))?;
                stdout.queue(cursor::MoveDown(1))?;
                stdout.queue(cursor::MoveToColumn(0))?;
                stdout.flush()?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_down_movement_works() {
        let mut game = Game::new();
        // Placing a line through the board
        // P
        // XXX
        //
        game.game_state[3] = CellState::X;
        game.game_state[4] = CellState::X;
        game.game_state[5] = CellState::X;
        // Moving the player down
        game.move_player(Direction::Down);
        // Player should be here
        //
        // XXX
        // P
        assert_eq!(game.player_position, 6)
    }

    #[test]
    fn player_complex_down_movement_works() {
        let mut game = Game::new();
        // Setting up the board to look like this
        // P
        // XXX
        // OO
        game.game_state[3] = CellState::X;
        game.game_state[4] = CellState::X;
        game.game_state[5] = CellState::X;
        game.game_state[6] = CellState::O;
        game.game_state[7] = CellState::O;
        // Moving the player down
        game.move_player(Direction::Down);
        // Player should be here
        //
        // XXX
        // OOP
        assert_eq!(game.player_position, 8)
    }

    #[test]
    fn player_up_movement_works() {
        let mut game = Game::new();
        // Setting up the board to look like this
        //
        // XXX
        //   P
        game.game_state[3] = CellState::X;
        game.game_state[4] = CellState::X;
        game.game_state[5] = CellState::X;
        game.player_position = 8;
        // Moving the player up
        game.move_player(Direction::Up);
        // Player should be here
        //   P
        // XXX
        //
        assert_eq!(game.player_position, 2)
    }

    #[test]
    fn player_complex_up_movement_works() {
        let mut game = Game::new();
        // Setting up the board to look like this
        //
        // XXX
        // OOP
        game.game_state[3] = CellState::X;
        game.game_state[4] = CellState::X;
        game.game_state[5] = CellState::X;
        game.game_state[6] = CellState::O;
        game.game_state[7] = CellState::O;
        game.player_position = 8;
        // Moving the player up
        game.move_player(Direction::Up);
        // Player should be here
        //   P
        // XXX
        // OO
        assert_eq!(game.player_position, 2)
    }

    #[test]
    fn computer_move_works() {
        let number_of_times_to_run = 100;

        // We loop this test multiple times, as the glitch only showed up sometimes
        for _ in 0..number_of_times_to_run {
            // We create a new game every iteration
            let mut game = Game::new();

            let mut game_state = game.game_state.clone();

            // We play 3 turns
            for _ in 0..3 {
                // We place a player token
                game.place_player_token();

                // Check that it has overwritten anything else
                for (index, cell) in game_state.iter().enumerate() {
                    if cell == &CellState::X || cell == &CellState::O {
                        assert_eq!(cell, &game.game_state[index]);
                    }
                }

                // We update our game state to compare against
                game_state = game.game_state.clone();

                // We place a computer token
                game.place_computer_token();

                // Check that it has overwritten anything else
                for (index, cell) in game_state.iter().enumerate() {
                    if cell == &CellState::X || cell == &CellState::O {
                        assert_eq!(cell, &game.game_state[index]);
                    }
                }

                // We update our game state to compare against
                game_state = game.game_state.clone();
            }
        }
    }
}
