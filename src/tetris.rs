use std::default::Default;
use glfw;
use piston::{Gl,Game,AssetStore};
use graphics::*;
use piece::Polyomino;
use shape::Color;

pub static BOARD_WIDTH: uint = 10;
pub static BOARD_HEIGHT: uint = 20;

static PADDING: f64 = 12.0;
static TILE_SPACING: f64 = 3.0;
static TILE_WIDTH: f64 = (400.0 - PADDING * 2.0 + TILE_SPACING) / BOARD_WIDTH as f64;
static TILE_HEIGHT: f64 = (800.0 - PADDING * 2.0 + TILE_SPACING) / BOARD_HEIGHT as f64;
static RECT_WIDTH: f64 = TILE_WIDTH - TILE_SPACING;
static RECT_HEIGHT: f64 = TILE_HEIGHT - TILE_SPACING;

#[deriving(Eq)]
enum State {
	//Intro,
	Playing,
	Defeat
}

pub struct Tetris {
	gravity_limit: f64,
	gravity_acc: f64,
	piece: Polyomino,
	board: [[Option<Color>,..BOARD_WIDTH],..BOARD_HEIGHT],
	state: State
}

impl Tetris {
	pub fn new() -> Tetris {
		Tetris {
			gravity_limit: 0.3,
			gravity_acc: 0.0,
			piece: Polyomino::new(), 
			board: [[Default::default(),..BOARD_WIDTH],..BOARD_HEIGHT],
			state: Playing
		}
	}
	fn release_piece(&mut self) {
		for &(x,y) in self.piece.as_points().iter() {
			if y < self.board.len() && x < self.board[y].len() {
				self.board[y][x] = Some(self.piece.get_color());
			} else {
				self.state = Defeat;
			}
		}
		if self.state == Playing {
			let mut board: [[Option<Color>,..BOARD_WIDTH],..BOARD_HEIGHT] = [[None,..BOARD_WIDTH],..BOARD_HEIGHT];
			for (new,old) in board.mut_iter().rev().zip(self.board.iter().rev().filter(|row| row.iter().any(|color| color.is_none()))) {
				*new = *old.clone();
			}
			self.board = board;
			self.piece = Polyomino::new();
		} else {
			println!("Defeat");
		}
	}
}

impl Game for Tetris {
	fn render(&self, context: &Context, gl: &mut Gl) {
		fn render_tile(context: &Context, gl: &mut Gl, x: uint, y: uint, color: [f32,..4]) {
			context.view()
				.rect(x as f64 * TILE_WIDTH + PADDING, y as f64 * TILE_HEIGHT + PADDING, RECT_WIDTH, RECT_HEIGHT)
				.color(color)
				.fill(gl)
		};
		for y in range(0u, BOARD_HEIGHT) {
			for x in range(0u, BOARD_WIDTH) {
				self.board[y][x].map(|e| render_tile(context, gl, x, y, e.as_RGBA()));
			}
		}
		for &(x,y) in self.piece.as_points().iter() {
			render_tile(context, gl, x, y, self.piece.get_color().as_RGBA());
		}
		match self.state {
			Playing => {
				
			}
			Defeat => {
				
			}
		}
	}
	fn update(&mut self, dt: f64, _asset_store: &mut AssetStore) {
		if self.state == Playing {
			self.gravity_acc += dt;
			if self.gravity_acc > self.gravity_limit {
				self.gravity_acc -= self.gravity_limit;
				if ! self.piece.try_move_down(&self.board) {
					self.release_piece();
				}
			}
		}
		
	
	}
	fn key_press(&mut self, key: glfw::Key, _: &mut AssetStore) {
		match self.state {
			Playing => match key {
				glfw::KeyRight => self.piece.try_move_right(&self.board),
				glfw::KeyLeft => self.piece.try_move_left(&self.board),
				glfw::KeyUp => self.piece.try_move_up(&self.board),
				glfw::KeyDown => { 
					self.piece.try_move_down(&self.board);
				},
				glfw::KeyE => self.piece.try_rotate_right(&self.board),
				glfw::KeyQ => self.piece.try_rotate_left(&self.board),
				_ => {},
			},
			_ => ()
		}
    }
}

