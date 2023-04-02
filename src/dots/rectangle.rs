use crate::dot::Dot;
use crate::dot3d::Dot3D;
use crate::dots3d::rudo_cube::RudoCube;

#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
    pub bottom_left: Dot,
    pub top_right: Dot,
}

impl Rectangle {
    pub fn new(bottom_left: Dot, top_right: Dot) -> Rectangle {
        Rectangle { bottom_left, top_right }
    }

    pub fn intersect(&self, point: &Dot) -> bool {
        point.x >= self.bottom_left.x && point.x <= self.top_right.x && point.y >= self.bottom_left.y && point.y <= self.top_right.y
    }

    pub fn to_cube(&self, height: f64) -> RudoCube {
        let center = Dot3D::new(
            (self.bottom_left.x + self.top_right.x) / 2.0,
            (self.bottom_left.y + self.top_right.y) / 2.0,
            height / 2.0,
        );
        let size = self.top_right.x - self.bottom_left.x;
        RudoCube::new(center, size)
    }

    pub fn contains(&self, point: &Dot) -> bool {
        point.x >= self.bottom_left.x && point.x <= self.top_right.x && point.y >= self.bottom_left.y && point.y <= self.top_right.y
    }

    pub fn is_outside(&self, point: &Dot) -> bool {
        point.x < self.bottom_left.x || point.x > self.top_right.x || point.y < self.bottom_left.y || point.y > self.top_right.y
    }
}
