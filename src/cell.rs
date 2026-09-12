use crate::vec2d::Vec2D;
use fastrand::Rng;

/// A map for cells to exist in.
pub struct CellMap {
    width: u32,
    height: u32,
    cells: Vec<bool>,
    seed: u64,
}

impl CellMap {
    /// Create a new CellMap with the provided width and height.
    pub fn new(width: u32, height: u32, seed: u64) -> Self {
        let mut rng = Rng::with_seed(seed);
        let mut cells = vec![false; (width * height) as usize];
        for i in 0..((width * height) as usize) {
            cells[i] = rng.bool();
        }
        CellMap {
            width,
            height,
            cells,
            seed,
        }
    }
    /// Reset the cells back to the initial state based on the provided seed.
    pub fn reset(&mut self) {
        let mut rng = Rng::with_seed(self.seed);
        for i in 0..self.count() {
            self.cells[i] = rng.bool();
        }
    }
    /// The amount of cells in this map.
    pub fn count(&self) -> usize {
        (self.width * self.height) as usize
    }
    /// Get the state of a cell at a given position.
    pub fn get_state(&self, pos: Vec2D) -> bool {
        let idx = pos.as_index(self.width as i32);
        self.cells[idx]
    }
    /// Set the state of a cell at a given position.
    pub fn set_state(&mut self, pos: Vec2D, state: bool) {
        let idx = pos.as_index(self.width as i32);
        self.cells[idx] = state;
    }
    /// Get all neighbors surrounding the cell at the given position.
    ///
    /// This excludes the calling position and any positions out-of-bounds.
    pub fn get_neighbors(&self, pos: Vec2D) -> Vec<(Vec2D, bool)> {
        let mut res: Vec<(Vec2D, bool)> = vec![];
        for y in (pos.y - 1)..=(pos.y + 1) {
            if y < 0 || y > (self.height - 1) as i32 {
                continue;
            }
            for x in (pos.x - 1)..=(pos.x + 1) {
                if x < 0 || (x == pos.x && y == pos.y) || x > (self.width - 1) as i32 {
                    continue;
                }
                let p = Vec2D::new(x, y);
                res.push((p, self.get_state(p)));
            }
        }
        res
    }
    /// Tick the CellMap to simulate the next generation.
    ///
    /// TODO: Allow for multiple rulesets.
    pub fn tick<T>(&mut self)
    where
        T: CellRule,
    {
        let mut res = self.cells.clone();
        for idx in 0..self.count() as usize {
            let pos = Vec2D::from_index(idx, self.width as i32);
            let state = self.cells[idx];
            res[idx] = T::apply_rule(self.get_neighbors(pos), state);
        }
        self.update_map(res);
    }
    /// Update the internal cell vector with new states.
    pub fn update_map(&mut self, cells: Vec<bool>) {
        self.cells.copy_from_slice(&cells[..]);
    }
    pub fn to_slice(&self) -> &[bool] {
        &self.cells[..]
    }
}

/// Describes a Cell Rule that determines the new state of the current cell.
pub trait CellRule {
    /// Returns the new state of the current cell based on the cell's neighbors and current state.
    fn apply_rule(neighbors: Vec<(Vec2D, bool)>, state: bool) -> bool;
}

/// Conway's Game of Life Ruleset
pub struct CellRuleConway {}

impl CellRule for CellRuleConway {
    fn apply_rule(neighbors: Vec<(Vec2D, bool)>, state: bool) -> bool {
        let mut alive = 0;
        // Count living neighbors.
        for (_, s) in neighbors {
            if s {
                alive += 1;
            }
        }
        if state {
            if alive < 2 {
                // Alive, less than 2 living neighbors.
                false
            } else if alive > 3 {
                // Alive, more than 3 living neighbors.
                false
            } else {
                state
            }
        } else if alive == 3 {
            // Dead, exactly 3 living neighbors.
            true
        } else {
            state
        }
    }
}
