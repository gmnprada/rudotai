use crate::dot::Dot;

#[derive(Debug, Copy, Clone)]
pub struct Semicircle {
    pub center: Dot,
    pub left_endpoint: Dot,
    pub right_endpoint: Dot,
    pub radius: f64,
}

impl Semicircle {
    pub fn new(center: Dot, radius: f64, is_left: bool) -> Semicircle {
        let x_offset = if is_left { -radius } else { radius };
        let left_endpoint = Dot::new(center.x - x_offset, center.y);
        let right_endpoint = Dot::new(center.x + x_offset, center.y);
        Semicircle {
            center,
            radius,
            left_endpoint,
            right_endpoint,
        }
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius / 2.0
    }

    fn intersect(&self, point: &Dot) -> bool {
        if point.x >= self.left_endpoint.x && point.x <= self.right_endpoint.x {
            self.center.distance_to(point) <= self.radius
        } else {
            false
        }
    }

    fn is_inner(&self, point: &Dot) -> bool {
        let dist = self.center.distance_to(point);
        dist < self.radius && (point.x - self.left_endpoint.x) * (point.x - self.right_endpoint.x) < 0.0
    }

    fn is_outer(&self, point: &Dot) -> bool {
        let dist = self.center.distance_to(point);
        dist > self.radius || (point.x - self.left_endpoint.x) * (point.x - self.right_endpoint.x) > 0.0
    }

    fn is_outside(&self, point: &Dot) -> bool {
        !self.is_inner(point) && !self.intersect(point)
    }
}
