use std::iter::Repeat;

///Taken from http://tetris.wikia.com/wiki/Nintendo_Rotation_System
///It represents 7 shapes, each having 4 possible rotations.
///A rotation is a 4x4 array of bools, which each bool representing a filled or empty tile.
static SHAPES: [[[[bool,..4],..4],..4],..7] = [
	[
		[
			[false,false,false,false],
			[false,false,false,false],
			[ true, true, true, true],
			[false,false,false,false]
		],[
			[false,false, true,false],
			[false,false, true,false],
			[false,false, true,false],
			[false,false, true,false]
		],
		[
			[false,false,false,false],
			[false,false,false,false],
			[ true, true, true, true],
			[false,false,false,false]
		],[
			[false,false, true,false],
			[false,false, true,false],
			[false,false, true,false],
			[false,false, true,false]
		],
	],[
		[
			[false,false,false,false],
			[ true, true, true,false],
			[false,false, true,false],
			[false,false,false,false]
		],[
			[false, true,false,false],
			[false, true,false,false],
			[ true, true,false,false],
			[false,false,false,false]
		],[
			[ true,false,false,false],
			[ true, true, true,false],
			[false,false,false,false],
			[false,false,false,false]
		],[
			[false, true, true,false],
			[false, true,false,false],
			[false, true,false,false],
			[false,false,false,false]
		],
	],[
		[
			[false,false,false,false],
			[ true, true, true,false],
			[ true,false,false,false],
			[false,false,false,false]
		],[
			[ true, true,false,false],
			[false, true,false,false],
			[false, true,false,false],
			[false,false,false,false]
		],[
			[false,false, true,false],
			[ true, true, true,false],
			[false,false,false,false],
			[false,false,false,false]
		],[
			[false, true,false,false],
			[false, true,false,false],
			[false, true, true,false],
			[false,false,false,false]
		],
	],[
		[
			[false,false,false,false],
			[false, true, true,false],
			[false, true, true,false],
			[false,false,false,false]
		],
		[
			[false,false,false,false],
			[false, true, true,false],
			[false, true, true,false],
			[false,false,false,false]
		],
		[
			[false,false,false,false],
			[false, true, true,false],
			[false, true, true,false],
			[false,false,false,false]
		],
		[
			[false,false,false,false],
			[false, true, true,false],
			[false, true, true,false],
			[false,false,false,false]
		],
	],[
		[
			[false,false,false,false],
			[false, true, true,false],
			[ true, true,false,false],
			[false,false,false,false]
		],[
			[false, true,false,false],
			[false, true, true,false],
			[false,false, true,false],
			[false,false,false,false]
		],[
			[false,false,false,false],
			[false, true, true,false],
			[ true, true,false,false],
			[false,false,false,false]
		],[
			[false, true,false,false],
			[false, true, true,false],
			[false,false, true,false],
			[false,false,false,false]
		]
	],[
		[
			[false,false,false,false],
			[ true, true, true,false],
			[false, true,false,false],
			[false,false,false,false]
		],[
			[false, true,false,false],
			[ true, true,false,false],
			[false, true,false,false],
			[false,false,false,false]
		],[
			[false, true,false,false],
			[ true, true, true,false],
			[false,false,false,false],
			[false,false,false,false]
		],
		[
			[false, true,false,false],
			[false, true, true,false],
			[false, true,false,false],
			[false,false,false,false]
		],
	],[
		[
			[false,false,false,false],
			[ true, true,false,false],
			[false, true, true,false],
			[false,false,false,false]
		],[
			[false,false, true,false],
			[false, true, true,false],
			[false, true,false,false],
			[false,false,false,false]
		],[
			[false,false,false,false],
			[ true, true,false,false],
			[false, true, true,false],
			[false,false,false,false]
		],[
			[false,false, true,false],
			[false, true, true,false],
			[false, true,false,false],
			[false,false,false,false]
		],
	],];

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

#[deriving(Rand)]
pub enum Shape { I, J, L, O, S, T, Z }

impl Shape {
	///Returns the RGBA color associated with the shape.
	pub fn associated_color(&self) -> [f32, ..4] {
		match *self {
			I => [0.0, 1.0, 1.0, 1.0],
			J => [0.5, 1.0, 0.0, 1.0],
			L => [1.0, 0.6, 0.0, 1.0], 
			O => [1.0, 1.0, 0.0, 1.0],
			S => [0.5, 1.0, 0.0, 1.0],
			T => [0.7, 0.7, 0.7, 1.0],
			Z => [1.0, 0.0, 0.0, 1.0]
		}
	}
	
	///Collects the coordinates of filled points in a rotated shape.
	///Format is (x,y)
	pub fn as_points(&self, rotation: Rotation) -> Vec<(uint,uint)> {
		SHAPES[*self as uint][rotation as uint].iter()
			.enumerate()
			.flat_map(|(y,row)| Repeat::new(y).zip(row.iter().enumerate()))
			.filter_map(|(y,(x,&b))| if b { Some((x,y)) } else { None })
			.collect()
	}
}