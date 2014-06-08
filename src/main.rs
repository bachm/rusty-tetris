#![crate_id = "rusty-tetris"]
#![feature(globs)]

extern crate graphics;
extern crate piston;
extern crate rand;
extern crate native;

mod tetris;
mod active;
mod tetromino;

#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}

fn main() {
	use piston::{AssetStore,GameWindow,GameWindowSettings,GameWindowSDL2,Game};
	
	let mut window: GameWindowSDL2 = GameWindow::new(
        GameWindowSettings {
            title: "Rusty Tetris".to_string(),
			size: [400, 800],
            fullscreen: false,
            exit_on_esc: true,
            background_color: [0.2, 0.2, 0.2, 0.2],
        }
    );

    let mut assets = AssetStore::from_folder("assets");
    let mut app = tetris::Tetris::new();
    app.run(&mut window, &mut assets);
}