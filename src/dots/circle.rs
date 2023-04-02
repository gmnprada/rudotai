use crate::dot::Dot;

#[derive(Debug, Copy, Clone)]
pub struct Circle {
    pub center: Dot,
    pub radius: f64,
}

impl Circle {
    pub fn new(center: Dot, radius: f64) -> Circle {
        Circle { center, radius }
    }

    fn intersect(&self, point: &Dot) -> bool {
        self.center.distance_to(point) <= self.radius
    }

    fn contains(&self, point: &Dot) -> bool {
        self.center.distance_to(point) < self.radius
    }

    fn is_inner(&self, other: &Circle) -> bool {
        self.center.distance_to(&other.center) + other.radius < self.radius
    }

    fn is_outer(&self, other: &Circle) -> bool {
        other.center.distance_to(&self.center) + self.radius < other.radius
    }

    fn is_outside(&self, other: &Circle) -> bool {
        self.center.distance_to(&other.center) > self.radius + other.radius
    }
}
