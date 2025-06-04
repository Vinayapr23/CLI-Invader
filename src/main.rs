mod input;
mod game;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType},
};
use std::{
    io::{stdout, Write},
    time::{Duration, Instant},
};

use crate::input::InputHandler;
use crate::game::{Game, GameState};

fn main() -> crossterm::Result<()> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let mut game = Game::new();
    let mut input = InputHandler::new();
    let mut last_frame = Instant::now();

    loop {
        let now = Instant::now();
        if now.duration_since(last_frame) < Duration::from_millis(60) {
            std::thread::sleep(Duration::from_millis(10));
            continue;
        }
        last_frame = now;

        while event::poll(Duration::from_millis(0))? {
            if let Event::Key(key_event) = event::read()? {
                // Quit on Ctrl+C
                if key_event.code == KeyCode::Char('c') && key_event.modifiers.contains(event::KeyModifiers::CONTROL) {
                    terminal::disable_raw_mode()?;
                    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;
                    return Ok(());
                }
        
                if input.handle_key_event(key_event.code) {
                    terminal::disable_raw_mode()?;
                    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;
                    return Ok(());
                }
            }
        }
        

        game.update(&input);
        input.reset();

        execute!(stdout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))?;
        game.draw(&mut stdout)?;
        stdout.flush()?;

        if let GameState::GameOver | GameState::Won = game.state {
            break;
        }
    }

    terminal::disable_raw_mode()?;
    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;

    match game.state {
        GameState::GameOver => println!("Game Over! Thanks for playing."),
        GameState::Won => {
            println!();
            println!("__     ______  _    _   _      ____   _   _   _ ");
            println!("\\ \\   / / __ \\| |  | | | |    / __ \\ | \\ | | | |");
            println!(" \\ \\_/ / |  | | |  | | | |   | |  | ||  \\| | | |");
            println!("  \\   /| |  | | |  | | | |   | |  | || . ` | | |");
            println!("   | | | |__| | |__| | | |___| |__| || |\\  | |_|");
            println!("   |_|  \\____/ \\____/  |______\\____/ |_| \\_| (_)");
            println!("\nCongratulations, you destroyed all invaders!");
        }
        _ => {}
    }

    Ok(())
}
