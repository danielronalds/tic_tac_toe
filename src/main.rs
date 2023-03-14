use std::io::{stdout, Write};

use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    terminal,
    QueueableCommand, Result,
};

use tic_tac_toe::Grid;

fn main() -> Result<()> {
    let mut stdout = stdout();
    let mut player_position = 0;

    stdout.queue(cursor::Hide)?;
    terminal::enable_raw_mode()?;

    let mut game = Grid::new();
    loop {
        game.draw(&mut stdout, player_position)?;

        //stdout.queue(terminal::Clear(ClearType::FromCursorDown))?;

        if let Event::Key(key) = read()? { match key.code {
                KeyCode::Up => if player_position > 2 {
                    player_position = player_position.saturating_sub(3)
                },
                KeyCode::Down => {
                    if player_position < 6 {
                        player_position += 3;
                    }
                },
                KeyCode::Left => player_position = player_position.saturating_sub(1),
                KeyCode::Right => {
                    match player_position {
                        2 | 5 | 8 => (),
                        _ => player_position += 1
                    };
                },
                KeyCode::Esc | KeyCode::Char('q') => break,
                _ => (),
            };
        };

        stdout.queue(cursor::MoveUp(5))?;
        stdout.flush()?;
    }

    stdout.queue(cursor::Show)?;
    terminal::disable_raw_mode()?;

    Ok(())

    //stdout.queue(cursor::MoveUp(5 + 1))?;
}
