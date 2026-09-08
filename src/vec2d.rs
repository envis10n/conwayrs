/// Basic 2D Vector implementation.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Vec2D {
    pub x: i32,
    pub y: i32,
}

impl Vec2D {
    /// Create a 2D vector with 0 for both x and y.
    pub fn empty() -> Self {
        Self { x: 0, y: 0 }
    }
    /// Create a 2D vector with the given x and y.
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    /// Get a 1D index with the provided column count.
    pub fn as_index(&self, col: i32) -> usize {
        (self.y * col + self.x) as usize
    }
    /// Get a 2D vector from a 1D index with the provided column count.
    pub fn from_index(idx: usize, col: i32) -> Vec2D {
        let idx = idx as i32;
        Vec2D::new(idx % col, idx / col)
    }
    /// Get the Euclidean distance between two 2D vectors.
    pub fn distance(&self, coord: Vec2D) -> f32 {
        let d = self.sub(coord);
        ((d.x.pow(2) + d.y.pow(2)) as f32).sqrt()
    }
    /// Get the Manhattan distance between two 2D vectors.
    pub fn manhattan(&self, coord: Vec2D) -> i32 {
        let d = self.sub(coord);
        d.x.abs() + d.y.abs()
    }
    /// Get the length of this 2D vector.
    pub fn length(&self) -> f32 {
        ((self.x.pow(2) + self.y.pow(2)) as f32).sqrt()
    }
    /// Get the result of adding two 2D vectors.
    pub fn add(&self, b: Vec2D) -> Vec2D {
        Vec2D::new(self.x + b.x, self.y + b.y)
    }
    /// Get the result of subtracting two 2D vectors.
    pub fn sub(&self, b: Vec2D) -> Vec2D {
        Vec2D::new(self.x - b.x, self.y - b.y)
    }
}

impl Default for Vec2D {
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn basic_eq() {
        let a = Vec2D::empty();
        let b = Vec2D::new(0, 0);
        let c = Vec2D::new(2, 10);

        assert_eq!(a, b);
        assert_eq!(a.distance(b), 0f32);
        assert_eq!(a.length(), 0f32);
        assert_eq!(c.length(), a.distance(c));
        assert_eq!(a.manhattan(c), 12);
    }
}
