use std::collections::VecDeque;
use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::video::FullscreenType;
use preferences::Preferences;

use game::{Direction, GameState, Tile};

pub struct Engine {
    game_state: GameState,
    tile_size: u32,
    event_pump: sdl2::EventPump,
    renderer: sdl2::render::Renderer<'static>,
}

impl Engine {
    pub fn run(&mut self) -> Result<(), String> {
        let mut framecounter = 0;
        let mut inputs = VecDeque::new();
        'mainloop: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'mainloop,
                    Event::KeyDown { keycode: Some(keycode), repeat: false, .. } => {
                        match keycode {
                            Keycode::Escape => break 'mainloop,
                            Keycode::F => {
                                {
                                    let mut window = self.renderer.window_mut().unwrap();
                                    let new_fullscreen_state = match window.fullscreen_state() {
                                        FullscreenType::Off => FullscreenType::Desktop,
                                        _ => FullscreenType::Off,
                                    };
                                    window.set_fullscreen(new_fullscreen_state)?;
                                }
                                let (level_width, level_height) = self.game_state.level_size();
                                self.renderer
                                    .set_logical_size(level_width as u32 * self.tile_size,
                                                      level_height as u32 * self.tile_size)
                                    .or_else(|e| Err(format!("{}", e)))?;
                            }
                            Keycode::Up => inputs.push_back(Direction::Up),
                            Keycode::Down => inputs.push_back(Direction::Down),
                            Keycode::Left => inputs.push_back(Direction::Left),
                            Keycode::Right => inputs.push_back(Direction::Right),
                            Keycode::Return => {
                                if !self.game_state.snake_alive() {
                                    self.game_state.reset();
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            if framecounter % 10 == 0 {
                self.game_state.update(inputs.pop_front())?;
            }
            self.render()?;
            framecounter += 1;
        }

        // Save game state on exit
        self.game_state.save(&::APP_INFO, "game_state").or_else(|e| Err(format!("{}", e)))?;

        Ok(())
    }

    fn render(&mut self) -> Result<(), String> {
        // Clear surface to black
        self.renderer.set_draw_color(Color::RGB(0, 0, 0));
        self.renderer.clear();

        // Draw floor
        let floor_color = if self.game_state.snake_alive() {
            Color::RGB(0, 0, 255)
        } else {
            Color::RGB(128, 0, 0)
        };
        self.renderer.set_draw_color(floor_color);
        let (level_width, level_height) = self.game_state.level_size();
        self.renderer
            .fill_rect(Rect::new(0,
                                 0,
                                 level_width as u32 * self.tile_size,
                                 level_height as u32 * self.tile_size))?;

        // Draw tiles other than floor
        for ((y, x), &tile) in self.game_state.tiles().indexed_iter() {
            match tile {
                Tile::Floor => {}
                Tile::Wall => {
                    self.renderer.set_draw_color(Color::RGB(255, 0, 0));
                    self.renderer
                        .fill_rect(Rect::new(x as i32 * self.tile_size as i32,
                                             y as i32 * self.tile_size as i32,
                                             self.tile_size,
                                             self.tile_size))?;
                }
                Tile::Food => {
                    self.renderer.set_draw_color(Color::RGB(255, 255, 0));
                    self.renderer
                        .fill_rect(Rect::new(x as i32 * self.tile_size as i32 + 1,
                                             y as i32 * self.tile_size as i32 + 1,
                                             self.tile_size - 2,
                                             self.tile_size - 2))?;
                }
                Tile::Snake(prev, next) => {
                    self.renderer.set_draw_color(Color::RGB(0, 255, 0));
                    if prev == Some(Direction::Up) || next == Some(Direction::Up) {
                        self.renderer
                            .fill_rect(Rect::new(x as i32 * self.tile_size as i32 + 1,
                                                 y as i32 * self.tile_size as i32,
                                                 self.tile_size - 2,
                                                 self.tile_size - 1))?;
                    }
                    if prev == Some(Direction::Down) || next == Some(Direction::Down) {
                        self.renderer
                            .fill_rect(Rect::new(x as i32 * self.tile_size as i32 + 1,
                                                 y as i32 * self.tile_size as i32 + 1,
                                                 self.tile_size - 2,
                                                 self.tile_size - 1))?;
                    }
                    if prev == Some(Direction::Left) || next == Some(Direction::Left) {
                        self.renderer
                            .fill_rect(Rect::new(x as i32 * self.tile_size as i32,
                                                 y as i32 * self.tile_size as i32 + 1,
                                                 self.tile_size - 1,
                                                 self.tile_size - 2))?;
                    }
                    if prev == Some(Direction::Right) || next == Some(Direction::Right) {
                        self.renderer
                            .fill_rect(Rect::new(x as i32 * self.tile_size as i32 + 1,
                                                 y as i32 * self.tile_size as i32 + 1,
                                                 self.tile_size - 1,
                                                 self.tile_size - 2))?;
                    }
                }
            }
        }

        // Present surface to screen
        self.renderer.present();

        Ok(())
    }
}

pub fn init() -> Result<Engine, String> {
    let game_state = GameState::load(&::APP_INFO, "game_state").unwrap_or_default();
    let tile_size = 8;
    let sdl = sdl2::init()?;
    let video = sdl.video()?;
    let event_pump = sdl.event_pump()?;
    let (level_width, level_height) = game_state.level_size();
    let window = video.window("Snake",
                              level_width as u32 * tile_size,
                              level_height as u32 * tile_size)
        .build()
        .or_else(|e| Err(format!("{}", e)))?;
    let renderer = window.renderer()
        .present_vsync()
        .build()
        .or_else(|e| Err(format!("{}", e)))?;

    Ok(Engine {
           game_state: game_state,
           tile_size: tile_size,
           event_pump: event_pump,
           renderer: renderer,
       })
}
