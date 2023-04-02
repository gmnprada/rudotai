#[derive(Debug, Copy, Clone)]
pub struct Dot {
    pub x: f64,
    pub y: f64,
}

impl Dot {
    pub fn new(x: f64, y: f64) -> Dot {
        Dot { x, y }
    }

    pub fn set_position(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    pub fn get_position(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    pub fn distance_to(&self, other: &Dot) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    pub fn midpoint(&self, other: &Dot) -> Dot {
        Dot::new((self.x + other.x) / 2.0, (self.y + other.y) / 2.0)
    }
}
