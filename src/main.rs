use std::io::{stdout, Write};

use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    terminal, QueueableCommand, Result,
};

use tic_tac_toe::{Direction, Game};

fn main() -> Result<()> {
    let mut stdout = stdout();

    stdout.queue(cursor::Hide)?;
    terminal::enable_raw_mode()?;

    let mut game = Game::new();

    loop {
        game.apply_game_logic();
        game.draw(&mut stdout)?;

        //stdout.queue(terminal::Clear(ClearType::FromCursorDown))?;

        if let Event::Key(key) = read()? {
            match key.code {
                KeyCode::Up | KeyCode::Char('k') => game.move_player(Direction::Up),
                KeyCode::Down | KeyCode::Char('j') => game.move_player(Direction::Down),
                KeyCode::Left | KeyCode::Char('h') => game.move_player(Direction::Left),
                KeyCode::Right | KeyCode::Char('l') => game.move_player(Direction::Right),
                KeyCode::Char(' ') => {
                    game.place_player_token();
                    game.place_computer_token();
                }
                KeyCode::Esc | KeyCode::Char('q') => break,
                _ => (),
            };
        };

        if game.game_over() {
            break;
        }

        stdout.queue(cursor::MoveUp(5))?;
        stdout.flush()?;
    }

    stdout.queue(cursor::MoveUp(5))?;
    stdout.flush()?;

    game.draw(&mut stdout)?;
    stdout.flush()?;

    stdout.queue(cursor::Show)?;
    terminal::disable_raw_mode()?;

    Ok(())

    //stdout.queue(cursor::MoveUp(5 + 1))?;
}
