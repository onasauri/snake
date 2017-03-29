use std::collections::VecDeque;
use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::video::FullscreenType;
use game;
use game::{Direction, Tile};

pub struct Engine {
    event_pump: sdl2::EventPump,
    renderer: sdl2::render::Renderer<'static>,
    game_state: game::GameState,
}

impl Engine {
    pub fn run(&mut self) -> Result<(), String> {
        let mut framecounter = 0;
        let mut inputs = VecDeque::new();
        loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => return Ok(()),
                    Event::KeyDown { keycode: Some(keycode), repeat: false, .. } => {
                        match keycode {
                            Keycode::Escape => return Ok(()),
                            Keycode::F => {
                                {
                                    let mut window = self.renderer.window_mut().unwrap();
                                    let new_fullscreen_state = match window.fullscreen_state() {
                                        FullscreenType::Off => FullscreenType::Desktop,
                                        _ => FullscreenType::Off,
                                    };
                                    window.set_fullscreen(new_fullscreen_state)?;
                                }
                                self.renderer
                                    .set_logical_size(640, 480)
                                    .or_else(|e| Err(format!("{}", e)))?;
                            }
                            Keycode::Up => inputs.push_back(Direction::Up),
                            Keycode::Down => inputs.push_back(Direction::Down),
                            Keycode::Left => inputs.push_back(Direction::Left),
                            Keycode::Right => inputs.push_back(Direction::Right),
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
    }

    fn render(&mut self) -> Result<(), String> {
        let floor_color = if self.game_state.snake_alive() {
            Color::RGB(0, 0, 255)
        } else {
            Color::RGB(128, 0, 0)
        };
        self.renderer.set_draw_color(floor_color);
        self.renderer.clear();
        let scale: u32 = 8;
        for ((y, x), &tile) in self.game_state.tiles().indexed_iter() {
            let color = match tile {
                Tile::Floor => floor_color,
                Tile::Wall => Color::RGB(255, 0, 0),
                Tile::Food => Color::RGB(255, 255, 0),
                Tile::Snake(..) => Color::RGB(0, 255, 0),
            };
            self.renderer.set_draw_color(color);
            match tile {
                Tile::Floor | Tile::Wall | Tile::Food => {
                    self.renderer
                        .fill_rect(Rect::new(x as i32 * scale as i32,
                                             y as i32 * scale as i32,
                                             scale,
                                             scale))?;
                }
                Tile::Snake(from, to) => {
                    if from == Some(Direction::Up) || to == Some(Direction::Up) {
                        self.renderer
                            .fill_rect(Rect::new(x as i32 * scale as i32 + 1,
                                                 y as i32 * scale as i32,
                                                 scale - 2,
                                                 scale - 1))?;
                    }
                    if from == Some(Direction::Down) || to == Some(Direction::Down) {
                        self.renderer
                            .fill_rect(Rect::new(x as i32 * scale as i32 + 1,
                                                 y as i32 * scale as i32 + 1,
                                                 scale - 2,
                                                 scale - 1))?;
                    }
                    if from == Some(Direction::Left) || to == Some(Direction::Left) {
                        self.renderer
                            .fill_rect(Rect::new(x as i32 * scale as i32,
                                                 y as i32 * scale as i32 + 1,
                                                 scale - 1,
                                                 scale - 2))?;
                    }
                    if from == Some(Direction::Right) || to == Some(Direction::Right) {
                        self.renderer
                            .fill_rect(Rect::new(x as i32 * scale as i32 + 1,
                                                 y as i32 * scale as i32 + 1,
                                                 scale - 1,
                                                 scale - 2))?;
                    }
                }
            }
        }
        self.renderer.present();

        Ok(())
    }
}

pub fn init() -> Result<Engine, String> {
    let sdl = sdl2::init()?;
    let video = sdl.video()?;
    let event_pump = sdl.event_pump()?;
    let window = video.window("Snake", 640, 480)
        .build()
        .or_else(|e| Err(format!("{}", e)))?;
    let renderer = window.renderer()
        .present_vsync()
        .build()
        .or_else(|e| Err(format!("{}", e)))?;
    let game_state = game::GameState::new();

    Ok(Engine {
           event_pump: event_pump,
           renderer: renderer,
           game_state: game_state,
       })
}
