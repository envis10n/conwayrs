mod cell;
mod vec2d;

use cell::CellMap;

fn main() {
    let mut cells = CellMap::new(256, 256);
    loop {
        cells.tick();
    }
}
