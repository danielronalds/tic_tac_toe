use std::io::Write;

use crossterm::{cursor, style::Print, QueueableCommand, Result};

pub struct Game {
    game_state: [CellState; 9],
    player_position: u8,
    previous_player_position: Option<usize>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
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
            game_state,
            previous_player_position: None,
            player_position: 0,
        }
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

    /// Draws the game board
    pub fn draw(&mut self, stdout: &mut std::io::Stdout) -> Result<()> {
        if let Some(index) = self.previous_player_position {
            self.game_state[index] = CellState::Empty;
        }

        self.game_state[self.player_position as usize] = CellState::Cursor;

        self.previous_player_position = Some(self.player_position as usize);

        for y in 0..=2 {
            let mut board = String::new();
            let offset = y * 3;
            for x in 0..=2 {
                let char = match self.game_state[x + offset] {
                    CellState::X => 'X',
                    CellState::O => 'O',
                    CellState::Empty => ' ',
                    CellState::Cursor => 'P',
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
}
