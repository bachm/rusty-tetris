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

    let mini = false;
    let (width, height) = (400, 800);
    let (width, height) = if mini {
            (width / 2, height / 2) 
        } else { (width, height) };
	let mut window: GameWindowSDL2 = GameWindow::new(
		GameWindowSettings {
            title: "Rusty Tetris".to_string(),
			size: [width, height],
            fullscreen: false,
            exit_on_esc: true,
            background_color: [0.2, 0.2, 0.2, 0.2],
        }
    );

	let mut assets = AssetStore::from_folder("assets");
	let mut app = tetris::Tetris::new(if mini { 0.5 } else { 1.0 });
	app.run(&mut window, &mut assets);
}
