# Rusty Tetris

A Tetris clone written in Rust. 

![screenshot](rustytetris.png?raw=true)


The fall speed increases every 10 tetrominoes.

Keys:
- Q / E rotates the active tetromino.
- Left / Right moves the active tetromino.
- Down drops the active tetromino.
- F1 to restart after losing.

Dependencies:
- https://github.com/PistonDevelopers/piston-workspace

Installation:
- Build piston-workspace according to instructions.
- Run `git clone https://github.com/bachm/rusty-tetris.git`
- Copy the contents of `/piston-workspace/piston-symlinks/` into `/rusty-tetris/target/cpu-vendor-os/lib/` (replace "cpu-vendor-os" your own, for example x86_64-unknown-linux-gnu)
- In the rusty-tetris folder, run `make exe`
