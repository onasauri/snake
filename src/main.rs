mod engine {
    extern crate sdl2;

    pub struct Engine {
        event_pump: sdl2::EventPump,
        renderer: sdl2::render::Renderer<'static>,
    }

    impl Engine {
        pub fn run(&mut self) {
            loop {
                for event in self.event_pump.poll_iter() {
                    match event {
                        sdl2::event::Event::Quit { .. } |
                        sdl2::event::Event::KeyDown { .. } => return,
                        _ => (),
                    }
                }
                self.renderer.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
                self.renderer.clear();
                self.renderer.present();
            }
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

        Ok(Engine {
               event_pump: event_pump,
               renderer: renderer,
           })
    }
}

fn main() {
    match engine::init() {
        Ok(mut engine) => engine.run(),
        Err(s) => println!("Engine initialization failed: {}", s),
    }
}
