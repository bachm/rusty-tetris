#![crate_id = "rusty-tetris"]
#![feature(globs)]

extern crate graphics;
extern crate piston;
extern crate rand;
extern crate native;
extern crate glfw;
extern crate opengles;

mod tetris;
mod piece;
mod shape;

#[start]
fn start(argc: int, argv: **u8) -> int {
    // Run GLFW on the main thread.
    native::start(argc, argv, main)
}

fn main() {
	use piston::Game;
	
	let game_window = piston::GameWindow::window("Rusty Tetris", 400, 800,
        piston::GameWindowSettings {
            exit_on_esc: true,
            background_color: [0.2, 0.2, 0.2, 1.0],
        }
    );
    
    let mut tetris = tetris::Tetris::new();
    tetris.run(&game_window);
}