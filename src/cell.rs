use crate::vec2d::Vec2D;

pub struct CellMap {
    width: u32,
    height: u32,
    cells: Vec<bool>,
}

impl CellMap {
    pub fn new(width: u32, height: u32) -> Self {
        CellMap {
            width,
            height,
            cells: vec![false; (width * height) as usize],
        }
    }
    pub fn count(&self) -> usize {
        (self.width * self.height) as usize
    }
    pub fn get_state(&self, pos: Vec2D) -> bool {
        let idx = pos.as_index(self.width as i32);
        self.cells[idx]
    }
    pub fn set_state(&mut self, pos: Vec2D, state: bool) {
        let idx = pos.as_index(self.width as i32);
        self.cells[idx] = state;
    }
    pub fn get_neighbors(&self, pos: Vec2D) -> Vec<(Vec2D, bool)> {
        let mut res: Vec<(Vec2D, bool)> = vec![];
        for y in (pos.y - 1)..=(pos.y + 1) {
            if y < 0 || y == pos.y || y > (self.height - 1) as i32 {
                continue;
            }
            for x in (pos.x - 1)..=(pos.x + 1) {
                if x < 0 || x == pos.x || x > (self.width - 1) as i32 {
                    continue;
                }
                let p = Vec2D::new(x, y);
                res.push((p, self.get_state(p)));
            }
        }
        res
    }
    pub fn update_map(&mut self, cells: Vec<bool>) {
        self.cells.copy_from_slice(&cells[..]);
    }
}
