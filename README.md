# Rusty Tetris

A Tetris clone written in Rust.

![screenshot](rustytetris.png?raw=true)


The fall speed increases every 10 tetrominoes.

## Keys:
- E / Q or Up => rotate
- A / D or Left / Right => move
- S or Down => drop
- F1 => restart after losing


## How to build & run

To build the examples, you need the latest Rust nightly and Cargo installed.

1. Install [Cargo](https://github.com/rust-lang/cargo)
2. Run `git clone https://github.com/bachm/rusty-tetris.git`
3. cd into the directory rusty-tetris
4. Type `cargo build`
5. Type `cargo run`

## Dependencies:

The project uses the [piston game engine](https://github.com/PistonDevelopers/piston)
