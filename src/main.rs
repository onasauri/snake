mod game {
    extern crate ndarray;

    type TileArray = ndarray::Array2<Tile>;

    pub struct GameState {
        tiles: TileArray,
    }

    impl GameState {
        pub fn new() -> Self {
            let width = 80;
            let height = 50;
            // Arrays are indexed in (row (y), column (x)) order
            let mut tiles = ndarray::Array::from_elem((height, width), Tile::Empty);

            // Build level wall
            for x in 0..width {
                tiles[[0, x]] = Tile::Wall;
                tiles[[height - 1, x]] = Tile::Wall;
            }
            for y in 0..height {
                tiles[[y, 0]] = Tile::Wall;
                tiles[[y, width - 1]] = Tile::Wall;
            }

            GameState { tiles: tiles }
        }

        pub fn tiles(&self) -> &TileArray {
            &self.tiles
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub enum Tile {
        Empty,
        Wall,
    }
}

mod engine {
    extern crate sdl2;

    use game;

    pub struct Engine {
        event_pump: sdl2::EventPump,
        renderer: sdl2::render::Renderer<'static>,
        game_state: game::GameState,
    }

    impl Engine {
        pub fn run(&mut self) -> Result<(), String> {
            loop {
                for event in self.event_pump.poll_iter() {
                    match event {
                        sdl2::event::Event::Quit { .. } |
                        sdl2::event::Event::KeyDown { .. } => return Ok(()),
                        _ => (),
                    }
                }
                self.render()?;
            }
        }

        fn render(&mut self) -> Result<(), String> {
            self.renderer.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
            self.renderer.clear();
            let scale: u32 = 8;
            for ((y, x), &tile) in self.game_state.tiles().indexed_iter() {
                let color = match tile {
                    game::Tile::Empty => sdl2::pixels::Color::RGB(255, 0, 0),
                    game::Tile::Wall => sdl2::pixels::Color::RGB(0, 255, 0),
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
}

fn main() {
    match engine::init() {
        Ok(mut engine) => {
            if let Err(s) = engine.run() {
                println!("Runtime error: {}", s)
            }
        }
        Err(s) => println!("Engine initialization failed: {}", s),
    }
}
