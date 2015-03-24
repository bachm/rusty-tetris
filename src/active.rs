use rand::{thread_rng, Rng};
use tetromino::{Tetromino,SHAPES,Color,Rotation};
use tetromino::Rotation::*;

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;
static HIDDEN_ROWS: [usize; 3] = [-3, -2, -1];

pub struct ActiveTetromino {
    x: usize,
    y: usize,
    rotation: Rotation,
    shape: &'static Tetromino
}

impl ActiveTetromino {
    pub fn new() -> ActiveTetromino {
        ActiveTetromino {
            x: BOARD_WIDTH / 2 - 2,
            y: HIDDEN_ROWS[0],
            rotation: R0,
            shape: thread_rng().choose(&SHAPES).unwrap()
        }
    }

    pub fn as_points(&self) -> Vec<(usize,usize)> {
        self.shape.points(self.rotation).iter().map(|&(x,y)| (x.wrapping_add(self.x), y.wrapping_add(self.y))).collect()
    }

    pub fn get_color(&self) -> Color {
        self.shape.get_color()
    }

    pub fn try_rotate_right(&mut self, board: &[[Option<Color>;  BOARD_WIDTH];  BOARD_HEIGHT]) {
        let r = self.rotation.increase();
        if self.is_move_allowed(self.x, self.y, r, board) {
            self.rotation = r;
        };
    }

    pub fn try_rotate_left(&mut self, board: &[[Option<Color>; BOARD_WIDTH]; BOARD_HEIGHT]) {
        let r = self.rotation.decrease();
        if self.is_move_allowed(self.x, self.y, r, board) {
            self.rotation = r;
        };
    }

    pub fn try_move_right(&mut self, board: &[[Option<Color>; BOARD_WIDTH]; BOARD_HEIGHT]) {
        if self.is_move_allowed(self.x.wrapping_add(1), self.y, self.rotation, board) {
            self.x = self.x.wrapping_add(1);
        };
    }

    pub fn try_move_left(&mut self, board: &[[Option<Color>; BOARD_WIDTH]; BOARD_HEIGHT]) {
        if self.is_move_allowed(self.x.wrapping_sub(1), self.y, self.rotation, board) {
            self.x = self.x.wrapping_sub(1);
        };
    }

    pub fn try_move_down(&mut self, board: &[[Option<Color>; BOARD_WIDTH]; BOARD_HEIGHT]) -> bool {
        if self.is_move_allowed(self.x, self.y.wrapping_add(1), self.rotation, board) {
            self.y = self.y.wrapping_add(1);
            true
        } else {
            false
        }
    }

    fn is_move_allowed(&self, x2: usize, y2: usize, rotation: Rotation, board: &[[Option<Color>; BOARD_WIDTH]; BOARD_HEIGHT]) -> bool {
        !self.shape.points(rotation).iter()
            .map(|&(x1,y1)| { (x1.wrapping_add(x2),y1.wrapping_add(y2)) })
            .any(|(x,y)| board.get(y)
                .and_then(|e| e.get(x))
                .map(|e| e.is_some())
                .unwrap_or(!HIDDEN_ROWS.contains(&y)))
    }
}
