
use std::io::{stdout, Write};
use std::time::Instant;


use crossterm::{
    cursor::MoveTo,
    style::{Color, Print, ResetColor, SetForegroundColor},
    execute,
};

use crate::input::InputHandler;

pub const WIDTH: u16 = 40;
pub const HEIGHT: u16 = 20;

pub enum GameState {
    Playing,
    GameOver,
    Won
}

pub struct Game {
    pub player_x: u16,
    pub player_y: u16,
    pub bullet_x: Option<u16>,
    pub bullet_y: Option<u16>,
    pub invaders: Vec<(u16, u16)>,
    pub invader_direction: i16,
    pub state: GameState,
    pub score: u32,
    pub iconic_mode: bool,
    pub last_invader_move: Instant,
}

impl Game {
    pub fn new() -> Self {
        let iconic_mode = std::env::var("ICONIC").unwrap_or_default() == "1"
            || std::env::var("LANG").unwrap_or_default().to_lowercase().contains("utf");

        let mut invaders = Vec::new();
        for y in 1..4 {
            for x in 5..(WIDTH - 5) {
                if x % 2 == 0 {
                    invaders.push((x, y));
                }
            }
        }

        Self {
            player_x: WIDTH / 2,
            player_y: HEIGHT - 2,
            bullet_x: None,
            bullet_y: None,
            invaders,
            invader_direction: 1,
            state: GameState::Playing,
            score: 0,
            iconic_mode,
            last_invader_move: Instant::now(),

        }
    }

    pub fn update(&mut self, input: &InputHandler) {
        if let GameState::Playing = self.state {
            if input.left && self.player_x > 1 {
                self.player_x -= 1;
            }
            if input.right && self.player_x < WIDTH - 2 {
                self.player_x += 1;
            }
    
            if input.shoot && self.bullet_y.is_none() {
                self.bullet_x = Some(self.player_x);
                self.bullet_y = Some(self.player_y - 1);
            }
    
            if let Some(by) = self.bullet_y {
                if by > 0 {
                    let new_y = by - 1;
                    if let Some((i, _)) = self.invaders.iter().enumerate()
                    .find(|&(_, &(x, y))| {
                        let bx = self.bullet_x.unwrap();
                        let by = new_y;
                        (x as i16 - bx as i16).abs() <= 1 && (y as i16 - by as i16).abs() <= 0
                    })
                    {
                        self.invaders.remove(i);
                        self.bullet_x = None;
                        self.bullet_y = None;
                        self.score += 10;
                    } else {
                        self.bullet_y = Some(new_y);
                    }
                } else {
                    self.bullet_x = None;
                    self.bullet_y = None;
                }
            }
    
            // Invader movement every 0.5 second
            if self.last_invader_move.elapsed().as_millis() >= 500 {
                self.last_invader_move = Instant::now();
    
                let edge_hit = self.invaders.iter().any(|&(x, _)| {
                    (self.invader_direction == 1 && x >= WIDTH - 2)
                        || (self.invader_direction == -1 && x <= 1)
                });
    
                if edge_hit {
                    for inv in &mut self.invaders {
                        inv.1 += 1;
                    }
                    self.invader_direction *= -1;
                } else {
                    for inv in &mut self.invaders {
                        inv.0 = (inv.0 as i16 + self.invader_direction) as u16;
                    }
                }
                  
                if self.invaders.is_empty() {
                     self.state = GameState::Won;
             }
    
                if self.invaders.iter().any(|&(_, y)| y >= self.player_y) {
                    self.state = GameState::GameOver;
                }
            }
        }
    }
    pub fn draw<W: Write>(&self, stdout: &mut W) -> std::io::Result<()> {
        for x in 0..=WIDTH {
            execute!(stdout, MoveTo(x, 0), Print("-"))?;
            execute!(stdout, MoveTo(x, HEIGHT), Print("-"))?;
        }
        for y in 0..=HEIGHT {
            execute!(stdout, MoveTo(0, y), Print("|"))?;
            execute!(stdout, MoveTo(WIDTH, y), Print("|"))?;
        }

        execute!(
            stdout,
            MoveTo(self.player_x, self.player_y),
            SetForegroundColor(Color::Green),
            Print("A"),
            ResetColor
        )?;

        if let (Some(bx), Some(by)) = (self.bullet_x, self.bullet_y) {
            execute!(
                stdout,
                MoveTo(bx, by),
                SetForegroundColor(Color::Yellow),
                Print("|"),
                ResetColor
            )?;
        }

        let invader_symbol = if self.iconic_mode { "👾" } else { "X" };

        for &(x, y) in &self.invaders {
            execute!(
                stdout,
                MoveTo(x, y),
                SetForegroundColor(Color::Red),
                Print(invader_symbol),
                ResetColor
            )?;
        }

        execute!(
            stdout,
            MoveTo(2, HEIGHT + 1),
            SetForegroundColor(Color::White),
            Print(format!("Score: {}", self.score)),
            ResetColor
        )?;

        Ok(())
    }
}
