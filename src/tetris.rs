use std::default::Default;
use graphics::*;
use piston::{AssetStore,Game,RenderArgs,Texture,KeyPressArgs,UpdateArgs,};
use piston::keyboard;
use active::ActiveTetromino;
use tetromino::Color;

pub static BOARD_WIDTH: uint = 10;
pub static BOARD_HEIGHT: uint = 20;
static TILE_SIZE: f64 = 40.0;

#[deriving(PartialEq)]
enum State {
	Playing,
	Dropping,
	Defeated
}

pub struct Tetris {
	gravity_accumulator: f64,
	gravity_factor: f64,
	tetromino_count: uint,
	active_tetromino: ActiveTetromino,
	board: [[Option<Color>,..BOARD_WIDTH],..BOARD_HEIGHT],
	state: State,
	block: Option<Texture>,
    paused: bool,
    scale: f64,
}

impl Tetris {
	pub fn new(scale: f64) -> Tetris {
		Tetris {
			gravity_accumulator: 0.0,
			gravity_factor: 1.0,
			tetromino_count: 0,
			active_tetromino: ActiveTetromino::new(),
			board: [[Default::default(),..BOARD_WIDTH],..BOARD_HEIGHT],
			state: Playing,
			block: None,
            paused: false,
            scale: scale,
		}
	}
	fn gravity(&mut self, amount: f64) {
		self.gravity_accumulator += amount * self.gravity_factor;
		if self.gravity_accumulator >= 0.35 {
			self.gravity_accumulator = 0.0;
			if ! self.active_tetromino.try_move_down(&self.board) {
				for &(x,y) in self.active_tetromino.as_points().iter() {
					if y < self.board.len() && x < self.board[y].len() {
						self.board[y][x] = Some(self.active_tetromino.get_color());
					} else {
						self.state = Defeated;
					}
				}
				if self.state == Playing || self.state == Dropping {
					self.state = Playing;
					let mut board: [[Option<Color>,..BOARD_WIDTH],..BOARD_HEIGHT] = [[None,..BOARD_WIDTH],..BOARD_HEIGHT];
					for (new,old) in board.mut_iter().rev().zip(self.board.iter().rev().filter(|row| row.iter().any(|color| color.is_none()))) {
						*new = *old.clone();
					}
					self.board = board;
					self.active_tetromino = ActiveTetromino::new();
					self.tetromino_count += 1;
					if self.tetromino_count >= 10 {
						self.tetromino_count = 0;
						self.gravity_factor *= 1.1;
					}
				}
			}
		}
	}
	fn play_again(&mut self) {
		self.state = Playing;
		self.gravity_accumulator = 0.0;
		self.tetromino_count = 0;
		self.gravity_factor = 1.0;
		self.board = [[Default::default(),..BOARD_WIDTH],..BOARD_HEIGHT];
		self.active_tetromino = ActiveTetromino::new();
	}
}

impl Game for Tetris {
	fn load(&mut self, assets: &mut AssetStore) {
		let image = assets.path("block.png").unwrap();
        self.block = Some(Texture::from_path(&image).unwrap());
	}
	fn render(&self, c: &Context, args: RenderArgs) {
        let c = c.zoom(self.scale);
		fn pos(n: uint) -> f64 { n as f64 * TILE_SIZE }
		for y in range(0u, BOARD_HEIGHT) {
			for x in range(0u, BOARD_WIDTH) {
				self.board[y][x].as_ref().map(|e| c.trans(pos(x), pos(y)).image(self.block.as_ref().unwrap()).color(e.as_rgba()).draw(args.gl));
			}
		}
		for &(x,y) in self.active_tetromino.as_points().iter() {
			c.trans(pos(x), pos(y)).image(self.block.as_ref().unwrap()).color(self.active_tetromino.get_color().as_rgba()).draw(args.gl);
		}
	}
	fn update(&mut self, args: UpdateArgs) {
        if self.paused { return }

		match self.state {
			Playing		=> self.gravity(args.dt),
			Dropping	=> self.gravity(0.12 + args.dt),
			_ => {}
		}
	}
	fn key_press(&mut self, args: KeyPressArgs) {
		match (self.state, args.key) {
			(Defeated, keyboard::F1)	=> self.play_again(),
			(Playing, keyboard::E) if !self.paused		
                => self.active_tetromino.try_rotate_right(&self.board),
			(Playing, keyboard::Q) if !self.paused		
                => self.active_tetromino.try_rotate_left(&self.board),
			(Playing, keyboard::A) | (Playing, keyboard::Left) if !self.paused
				=> self.active_tetromino.try_move_left(&self.board),
			(Playing, keyboard::D) | (Playing, keyboard::Right) if !self.paused
				=> self.active_tetromino.try_move_right(&self.board),
			(Playing, keyboard::Down) | (Playing, keyboard::S) if !self.paused
				=> self.state = Dropping,
            (Playing, keyboard::P)
                => self.paused = !self.paused,
			_ => {}
		}
    }
}

