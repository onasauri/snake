use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::video::FullscreenType;
use game;
use game::Direction;

pub struct Engine {
    event_pump: sdl2::EventPump,
    renderer: sdl2::render::Renderer<'static>,
    game_state: game::GameState,
}

impl Engine {
    pub fn run(&mut self) -> Result<(), String> {
        let mut framecounter = 0;
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
                            Keycode::Up => self.game_state.set_snake_dir(Direction::Up),
                            Keycode::Down => self.game_state.set_snake_dir(Direction::Down),
                            Keycode::Left => self.game_state.set_snake_dir(Direction::Left),
                            Keycode::Right => self.game_state.set_snake_dir(Direction::Right),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            if framecounter % 10 == 0 {
                self.game_state.update();
            }
            self.render()?;
            framecounter += 1;
        }
    }

    fn render(&mut self) -> Result<(), String> {
        self.renderer.set_draw_color(Color::RGB(0, 0, 0));
        self.renderer.clear();
        let scale: u32 = 8;
        for ((y, x), &tile) in self.game_state.tiles().indexed_iter() {
            let color = match tile {
                game::Tile::Floor => Color::RGB(0, 0, 255),
                game::Tile::Wall => Color::RGB(255, 0, 0),
                game::Tile::Food => Color::RGB(255, 255, 0),
                game::Tile::Snake(_) => Color::RGB(0, 255, 0),
            };
            self.renderer.set_draw_color(color);
            self.renderer
                .fill_rect(sdl2::rect::Rect::new(x as i32 * scale as i32,
                                                 y as i32 * scale as i32,
                                                 scale,
                                                 scale))?;
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
