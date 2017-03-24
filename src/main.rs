mod game {
    extern crate ndarray;

    type TileArray = ndarray::Array2<Tile>;
    // Index into a TileArray; arrays are indexed in (row (y), column (x)) order
    type TileIndex = (usize, usize);

    pub struct GameState {
        tiles: TileArray,
        snake_head: TileIndex,
        snake_tail: TileIndex,
        snake_dir: TileIndex,
    }

    impl GameState {
        pub fn new() -> Self {
            let width = 80;
            let height = 50;
            let mut tiles = ndarray::Array::from_elem((height, width), Tile::Floor);

            // Build level wall
            for x in 0..width {
                tiles[(0, x)] = Tile::Wall;
                tiles[(height - 1, x)] = Tile::Wall;
            }
            for y in 0..height {
                tiles[(y, 0)] = Tile::Wall;
                tiles[(y, width - 1)] = Tile::Wall;
            }

            // Place snake
            tiles[(3, 3)] = Tile::Snake(Some((3, 4)));
            tiles[(3, 4)] = Tile::Snake(Some((3, 5)));
            tiles[(3, 5)] = Tile::Snake(None);
            let snake_head = (3, 5);
            let snake_tail = (3, 3);
            let snake_dir = (0, 1);

            GameState {
                tiles: tiles,
                snake_head: snake_head,
                snake_tail: snake_tail,
                snake_dir: snake_dir,
            }
        }

        pub fn tiles(&self) -> &TileArray {
            &self.tiles
        }

        pub fn update(&mut self) {
            // Move snake
            let new_snake_head = (self.snake_head.0 + self.snake_dir.0,
                                  self.snake_head.1 + self.snake_dir.1);
            // Handle out of bounds or collision
            match self.tiles.get(new_snake_head) {
                None => {
                    // New head is out of bounds, so don't move
                    // FIXME Shouldn't happen, so report error or something
                    return;
                }
                Some(&Tile::Wall) |
                Some(&Tile::Snake(_)) => {
                    // New head collides with wall or snake, so game over
                    // FIXME Implement game over, just return for now
                    return;
                }
                Some(&Tile::Food) => {
                    // New head collides with food, so eat the food
                    // FIXME Spawn new food; adjust score
                }
                Some(&Tile::Floor) => (), // No collision
            }
            self.tiles[self.snake_head] = Tile::Snake(Some(new_snake_head));
            self.tiles[new_snake_head] = Tile::Snake(None);
            self.snake_head = new_snake_head;
            if let Tile::Snake(Some(new_snake_tail)) = self.tiles[self.snake_tail] {
                self.tiles[self.snake_tail] = Tile::Floor;
                self.snake_tail = new_snake_tail;
            } else {
                // FIXME Shouldn't happen, so report error?
                return;
            }
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub enum Tile {
        Floor,
        Wall,
        Food,
        // A Snake tile contains the index of the next tile of the snake (pointing towards the
        // head of the snake, which is the only Snake tile without an index)
        Snake(Option<TileIndex>),
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
                self.game_state.update();
                self.render()?;
            }
        }

        fn render(&mut self) -> Result<(), String> {
            self.renderer.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
            self.renderer.clear();
            let scale: u32 = 8;
            for ((y, x), &tile) in self.game_state.tiles().indexed_iter() {
                let color = match tile {
                    game::Tile::Floor => sdl2::pixels::Color::RGB(0, 0, 255),
                    game::Tile::Wall => sdl2::pixels::Color::RGB(255, 0, 0),
                    game::Tile::Food => sdl2::pixels::Color::RGB(255, 255, 0),
                    game::Tile::Snake(_) => sdl2::pixels::Color::RGB(0, 255, 0),
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
