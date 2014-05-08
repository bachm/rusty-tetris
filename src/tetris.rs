use std::default::Default;
use glfw;
use piston::{Gl,Game};
use graphics::*;
use piece::ActivePiece;
use shape::Shape;

pub static BOARD_WIDTH: uint = 10;
pub static BOARD_HEIGHT: uint = 20;
static XPADDING: f64 = 0.05;
static YPADDING: f64 = XPADDING / 2.0;
static XSPACE: f64 = 2.0 - XPADDING * 2.0;
static YSPACE: f64 = 2.0 - YPADDING * 2.0;
static TILE_WIDTH:  f64 = XSPACE / BOARD_WIDTH  as f64 * 0.95;
static TILE_HEIGHT: f64 = YSPACE / BOARD_HEIGHT as f64 * 0.95;

enum State {
	Intro,	//Display key bindings; ESC to exit, any other key to continue
	Playing,//Keep sending new piece; ESC to exit
	Defeat	//ESC to exit, any other key to play again
}

///'board' represents optionally drawable tiles.
///'Shape' allows reading of its color via the '.associated_color' method.
pub struct Tetris {
	piece: ActivePiece,
	board: [[Option<Shape>,..BOARD_WIDTH],..BOARD_HEIGHT] //
}

impl Tetris {
	pub fn new() -> Tetris {
		Tetris { piece: ActivePiece::new(), board: [[Default::default(),..BOARD_WIDTH],..BOARD_HEIGHT] }
	}
	fn render_tile(context: &Context, gl: &mut Gl, x: uint, y: uint, color: [f32,..4]) {
		let x = XSPACE / BOARD_WIDTH as f64 * x as f64 - 1.0 + XPADDING;
		let y = YSPACE / BOARD_HEIGHT as f64 * y as f64 - 1.0 + YPADDING;
		context.rect(x, y, TILE_WIDTH, TILE_HEIGHT).color(color).fill(gl)
	}
}

impl Game for Tetris {
	fn render(&self, context: &Context, gl: &mut Gl) {
		for y in range(0u, BOARD_HEIGHT) {
			for x in range(0u, BOARD_WIDTH) {
				self.board[y][x].map(|e| Tetris::render_tile(context, gl, x, y, e.associated_color()));
			}
		}
		for &(x,y) in self.piece.as_points().iter() {
			Tetris::render_tile(context, gl, x, y, self.piece.associated_color());
		}
	}
	fn key_press(&mut self, key: glfw::Key) {
		match key {
			glfw::KeyRight => self.piece.try_move_right(&self.board),
			glfw::KeyLeft => self.piece.try_move_left(&self.board),
			glfw::KeyUp => self.piece.try_move_up(&self.board),
			glfw::KeyDown => self.piece.try_move_down(&self.board),
			glfw::KeyE => self.piece.try_rotate_right(&self.board),
			glfw::KeyQ => self.piece.try_rotate_left(&self.board),
			_ => {},
        }
    }
}

