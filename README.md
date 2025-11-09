# CavaBlocks

Converts Cava output into ASCII block characters (i.e. ▁▂▃▄▅▆▇█) for use in taskbars or other things.

## Usage

Use like so:

- `<path to executable> <output width> <framerate>`
- I've found a width of 15 and a framerate of 30 works well for my taskbar
  - `target/release/cava-blocks 15 30`

## Installation

Clone the repo and run `cargo build --release`

### Dependencies
- Cava, obviously
- Cargo, obviously

