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
        let mut game_state = [CellState::Empty; 9];
        game_state[3] = CellState::X;
        game_state[1] = CellState::O;
        game_state[7] = CellState::X;
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
                if self.player_position > 2 {
                    new_position = new_position.saturating_sub(3)
                }

                while self.game_state[new_position as usize] != CellState::Empty {
                    if new_position > 2 {
                        new_position = new_position.saturating_sub(3)
                    } else {
                        break;
                    }
                }
            }
            Direction::Down => {
                if self.player_position < 6 {
                    new_position += 3;
                }
                while self.game_state[new_position as usize] != CellState::Empty {
                    if new_position < 6 {
                        new_position += 3;
                    } else {
                        break;
                    }
                }
            }
            Direction::Left => {
                new_position = new_position.saturating_sub(1);
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
