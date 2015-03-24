use std::old_path::*;
use std::default::Default;
use graphics::{ Context, RelativeTransform, Image, default_draw_state };
use opengl_graphics::{ GlGraphics, Texture };
use piston::event::UpdateArgs;
use piston::input::keyboard::Key;

use active::ActiveTetromino;
use tetromino::Color;
use tetris::State::*;

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;
static TILE_SIZE: f64 = 40.0;

#[derive(PartialEq, Copy)]
enum State {
    Playing,
    Dropping,
    Defeated
}

pub struct Tetris {
    gravity_accumulator: f64,
    gravity_factor: f64,
    tetromino_count: usize,
    active_tetromino: ActiveTetromino,
    board: [[Option<Color>; BOARD_WIDTH]; BOARD_HEIGHT],
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
            board: [[Default::default(); BOARD_WIDTH]; BOARD_HEIGHT],
            state: Playing,
            block: Some(Texture::from_path(&(Path::new("./bin/assets/block.png"))).unwrap()),
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
                    let mut board: [[Option<Color>; BOARD_WIDTH]; BOARD_HEIGHT] = [[None; BOARD_WIDTH]; BOARD_HEIGHT];
                    for (new,old) in board.iter_mut().rev().zip(self.board.iter().rev().filter(|row| row.iter().any(|color| color.is_none()))) {
                        *new = (*old).clone();
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
        self.board = [[Default::default(); BOARD_WIDTH]; BOARD_HEIGHT];
        self.active_tetromino = ActiveTetromino::new();
    }

    pub fn render(&mut self, c: &Context, gl: &mut GlGraphics) {
        let c = c.zoom(self.scale);
        fn pos(n: usize) -> f64 { n as f64 * TILE_SIZE }
        for y in 0usize..BOARD_HEIGHT {
            for x in 0usize..BOARD_WIDTH {
                self.board[y][x].as_ref()
                    .map(|e| Image::colored(e.as_rgba())
                                  .draw(self.block.as_ref().unwrap(), default_draw_state(),
                                        c.trans(pos(x), pos(y)).transform, gl));
            }
        }
        for &(x,y) in self.active_tetromino.as_points().iter() {
            Image::colored(self.active_tetromino.get_color().as_rgba())
                 .draw(self.block.as_ref().unwrap(), default_draw_state(), c.trans(pos(x), pos(y)).transform, gl);
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        if self.paused { return }

        match self.state {
            Playing     => self.gravity(args.dt),
            Dropping    => self.gravity(0.12 + args.dt),
            _ => {}
        }
    }

    pub fn key_press(&mut self, key: &Key) {
        match (self.state, key) {
            (Defeated, &Key::F1)
                => self.play_again(),
            (Playing,  &Key::E) if !self.paused
                => self.active_tetromino.try_rotate_right(&self.board),
            (Playing,  &Key::Up)    | (Playing, &Key::Q) if !self.paused
                => self.active_tetromino.try_rotate_left(&self.board),
            (Playing,  &Key::Left)  | (Playing, &Key::A) if !self.paused
                => self.active_tetromino.try_move_left(&self.board),
            (Playing,  &Key::Right) | (Playing, &Key::D) if !self.paused
                => self.active_tetromino.try_move_right(&self.board),
            (Playing,  &Key::Down)  | (Playing, &Key::S) if !self.paused
                => self.state = Dropping,
            (Playing,  &Key::P)
                => self.paused = !self.paused,
            _ => {}
        }
    }
}
