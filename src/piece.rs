use tetris::{BOARD_WIDTH,BOARD_HEIGHT};
use rand::{Rand,task_rng};
use shape::{Rotation,Shape,R0};

///Represents the currently active piece on the board.
pub struct ActivePiece {
	x: uint,
	y: uint,
	rotation: Rotation,
	shape: Shape
}

impl ActivePiece {
	///Returns an ActivePiece at the entry point with default rotation and random shape.
	pub fn new() -> ActivePiece {
		ActivePiece { x: BOARD_WIDTH / 2 - 1, y: BOARD_HEIGHT - 4, rotation: R0, shape: Rand::rand(&mut task_rng()) }
	}
	
	///Collects the coordinates of points occupied by the ActivePiece
	pub fn as_points(&self) -> Vec<(uint,uint)> {
		self.shape.as_points(self.rotation).move_iter().map(|(x,y)| (x + self.x, y + self.y)).collect()
	}
	
	pub fn associated_color(&self) -> [f32,..4] {
		self.shape.associated_color()
	}

	pub fn try_rotate_right(&mut self, board: &[[Option<Shape>,..BOARD_WIDTH],..BOARD_HEIGHT]) {
		let r = self.rotation.increase();
		if self.is_transformation_legal(self.x, self.y, r, board) {
			self.rotation = r;
		};
	}
	
	pub fn try_rotate_left(&mut self, board: &[[Option<Shape>,..BOARD_WIDTH],..BOARD_HEIGHT]) {
		let r = self.rotation.decrease();
		if self.is_transformation_legal(self.x, self.y, r, board) {
			self.rotation = r;
		};
	}
	
	pub fn try_move_right(&mut self, board: &[[Option<Shape>,..BOARD_WIDTH],..BOARD_HEIGHT]) {
		if self.is_transformation_legal(self.x + 1, self.y, self.rotation, board) {
			self.x += 1
		};
	}
	
	pub fn try_move_left(&mut self, board: &[[Option<Shape>,..BOARD_WIDTH],..BOARD_HEIGHT]) {
		if self.is_transformation_legal(self.x - 1, self.y, self.rotation, board) {
			self.x -= 1;
		}; 
	}
	
	pub fn try_move_down(&mut self, board: &[[Option<Shape>,..BOARD_WIDTH],..BOARD_HEIGHT]) {
		if self.is_transformation_legal(self.x, self.y - 1, self.rotation, board) {
			self.y -= 1;
		};
	}
	
	pub fn try_move_up(&mut self, board: &[[Option<Shape>,..BOARD_WIDTH],..BOARD_HEIGHT]) {
		if self.is_transformation_legal(self.x, self.y + 1, self.rotation, board) { 
			self.y += 1;
		};
	}
	
	fn is_transformation_legal(&mut self, x2: uint, y2: uint, rotation: Rotation, board: &[[Option<Shape>,..BOARD_WIDTH],..BOARD_HEIGHT]) -> bool {
		!self.shape.as_points(rotation).iter()
			.any(|&(x1,y1)| board.get(y1 + y2)
				.and_then(|e| e.get(x1 + x2))
				.map(|e| e.is_some())
				.unwrap_or(true))
	}
}