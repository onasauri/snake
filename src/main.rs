extern crate ndarray;
extern crate sdl2;

pub mod engine;
pub mod game;

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
