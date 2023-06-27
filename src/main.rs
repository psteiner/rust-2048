#![crate_name = "game2048"]
mod ui;
mod game;

mod prelude {
    pub use std::env;
    pub use std::fmt;
    pub use std::iter;
    pub use std::path::Path;
    pub use std::str::FromStr;
    
    pub use rand::prelude::*;
    pub use sdl2::render::WindowCanvas;
    
    pub use sdl2::event::Event;
    pub use sdl2::gfx::*;
    pub use sdl2::keyboard::Keycode;
    pub use sdl2::pixels::Color;
    pub use sdl2::rect::Rect;
    pub use sdl2::rwops::*;
    pub use sdl2::ttf::*;
    
    pub use crate::game::GameManager;
    pub use crate::ui::*;
}

use prelude::*;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let size : usize = match args.len() {
        1 => 4,
        3 => i64::from_str(args[2].as_ref()).unwrap_or(4) as usize,
        _ => {
            panic!("usage: ./game2048 --size NUM")
        }
    };

    match ui::run(size) {
        Ok(_) => (),
        Err(e) => panic!("Error while running game: {}", e),
    }

}
