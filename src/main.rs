extern crate ndarray;
extern crate preferences;
extern crate rand;
extern crate sdl2;
#[macro_use]
extern crate serde_derive;

use preferences::AppInfo;

pub mod engine;
pub mod game;

const APP_INFO: AppInfo = AppInfo { name: "snake", author: "onasauri" };

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
