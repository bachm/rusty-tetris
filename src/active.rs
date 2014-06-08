use tetris::{BOARD_WIDTH,BOARD_HEIGHT};
use std::rand::{Rng,task_rng};
use tetromino::{Tetromino,SHAPES,Color,Rotation,R0};

static HIDDEN_ROWS: [uint,..3] = [-3, -2,-1];

pub struct ActiveTetromino {
	x: uint,
	y: uint,
	rotation: Rotation,
	shape: &'static Tetromino
}

impl ActiveTetromino {
	pub fn new() -> ActiveTetromino {
		ActiveTetromino { 
			x: BOARD_WIDTH / 2 - 2, 
			y: HIDDEN_ROWS[0], 
			rotation: R0, 
			shape: task_rng().choose(SHAPES).unwrap()
		}
	}
	
	pub fn as_points(&self) -> Vec<(uint,uint)> {
		self.shape.points(self.rotation).iter().map(|&(x,y)| (x + self.x, y + self.y)).collect()
	}
	
	pub fn get_color(&self) -> Color {
		self.shape.get_color()
	}

	pub fn try_rotate_right(&mut self, board: &[[Option<Color>,..BOARD_WIDTH],..BOARD_HEIGHT]) {
		let r = self.rotation.increase();
		if self.is_move_allowed(self.x, self.y, r, board) {
			self.rotation = r;
		};
	}
	
	pub fn try_rotate_left(&mut self, board: &[[Option<Color>,..BOARD_WIDTH],..BOARD_HEIGHT]) {
		let r = self.rotation.decrease();
		if self.is_move_allowed(self.x, self.y, r, board) {
			self.rotation = r;
		};
	}
	
	pub fn try_move_right(&mut self, board: &[[Option<Color>,..BOARD_WIDTH],..BOARD_HEIGHT]) {
		if self.is_move_allowed(self.x + 1, self.y, self.rotation, board) {
			self.x += 1
		};
	}
	
	pub fn try_move_left(&mut self, board: &[[Option<Color>,..BOARD_WIDTH],..BOARD_HEIGHT]) {
		if self.is_move_allowed(self.x - 1, self.y, self.rotation, board) {
			self.x -= 1;
		}; 
	}
	
	pub fn try_move_down(&mut self, board: &[[Option<Color>,..BOARD_WIDTH],..BOARD_HEIGHT]) -> bool {
		if self.is_move_allowed(self.x, self.y + 1, self.rotation, board) {
			self.y += 1;
			true
		} else {
			false
		}
	}
	
	fn is_move_allowed(&mut self, x2: uint, y2: uint, rotation: Rotation, board: &[[Option<Color>,..BOARD_WIDTH],..BOARD_HEIGHT]) -> bool {
		!self.shape.points(rotation).iter()
			.map(|&(x1,y1)| (x1+x2,y1+y2))
			.any(|(x,y)| board.get(y)
				.and_then(|e| e.get(x))
				.map(|e| e.is_some())
				.unwrap_or(!HIDDEN_ROWS.contains(&y)))
	}
}