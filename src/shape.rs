pub static SHAPES: [Shape,..7] = [
	Shape {
		color: Cyan,
		points: [[(0,2),(1,2),(2,2),(3,2)],[(2,0),(2,1),(2,2),(2,3)],[(0,2),(1,2),(2,2),(3,2)],[(2,0),(2,1),(2,2),(2,3)]]
	},
	Shape {
		color: Blue,
		points: [[(0,1),(1,1),(2,1),(2,2)],[(1,0),(1,1),(0,2),(1,2)],[(0,0),(0,1),(1,1),(2,1)],[(1,0),(2,0),(1,1),(1,2)]]
	},
	Shape {
		color: Orange,
		points: [[(0,1),(1,1),(2,1),(0,2)],[(0,0),(1,0),(1,1),(1,2)],[(2,0),(0,1),(1,1),(2,1)],[(1,0),(1,1),(1,2),(2,2)]]
	},
	Shape {
		color: Yellow,
		points: [[(1,1),(2,1),(1,2),(2,2)],[(1,1),(2,1),(1,2),(2,2)],[(1,1),(2,1),(1,2),(2,2)],[(1,1),(2,1),(1,2),(2,2)]]
	},
	Shape {
		color: Lime,
		points: [[(1,1),(2,1),(0,2),(1,2)],[(1,0),(1,1),(2,1),(2,2)],[(1,1),(2,1),(0,2),(1,2)],[(1,0),(1,1),(2,1),(2,2)]]
	},
	Shape {
		color: Purple,
		points: [[(0,1),(1,1),(2,1),(1,2)],[(1,0),(0,1),(1,1),(1,2)],[(1,0),(0,1),(1,1),(2,1)],[(1,0),(1,1),(2,1),(1,2)]]
	},
	Shape {
		color: Red,
		points: [[(0,1),(1,1),(1,2),(2,2)],[(2,0),(1,1),(2,1),(1,2)],[(0,1),(1,1),(1,2),(2,2)],[(2,0),(1,1),(2,1),(1,2)]]
	}
];

pub struct Shape {
	color: Color,  
	points: [[(uint,uint),..4],..4]
}

impl Shape {
	pub fn points(&'static self, rotation: Rotation) -> &'static [(uint,uint),..4] {
		&self.points[rotation as uint]
	}
	pub fn get_color(&self) -> Color {
		self.color
	}
}

pub enum Rotation { R0, R1, R2, R3 }

impl Rotation {
	pub fn increase(&self) -> Rotation {
		match *self {
			R0 => R1,
			R1 => R2,
			R2 => R3,
			R3 => R0
		}
	}
	pub fn decrease(&self) -> Rotation {
		match *self {
			R0 => R3,
			R1 => R0,
			R2 => R1,
			R3 => R2
		}
	}
}

pub enum Color { Cyan, Blue, Orange, Yellow, Lime, Purple, Red }

impl Color {
	pub fn as_RGBA(&self) -> [f32,..4] {
		match *self {
			Cyan	=> [0.0, 1.0, 1.0, 1.0],
			Blue	=> [0.0, 0.2, 1.0, 1.0],
			Orange	=> [1.0, 0.6, 0.0, 1.0],
			Yellow	=> [1.0, 1.0, 0.0, 1.0],
			Lime	=> [0.5, 1.0, 0.0, 1.0],
			Purple	=> [0.8, 0.0, 1.0, 1.0],
			Red		=> [1.0, 0.0, 0.0, 1.0]
		}
	}
}