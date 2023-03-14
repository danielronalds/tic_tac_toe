use std::io::Write;

use crossterm::{
    cursor,
    style::Print,
    QueueableCommand, Result,
};

pub struct Grid {
    game_state: [CellState; 9],
    previous_player_position: Option<usize>,
}

#[derive(Clone, Copy)]
enum CellState {
    X,
    O,
    Empty,
    Cursor
}

impl Grid {
    pub fn new() -> Self {
        Self {
            game_state: [CellState::Empty; 9],
            previous_player_position: None,
        }
    }

    /// Draws the game board
    pub fn draw(&mut self, stdout: &mut std::io::Stdout, player_position: u8) -> Result<()> {
        if let Some(index) = self. previous_player_position {
            self.game_state[index] = CellState::Empty;
        }

        self.game_state[player_position as usize] = CellState::Cursor;
        
        self.previous_player_position = Some(player_position as usize);

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
