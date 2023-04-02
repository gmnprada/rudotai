use crate::dot::Dot;
use crate::dot3d::Dot3D;
use crate::dots::triangle::Triangle;

pub struct RudoPyramid {
    base: Triangle,
    height: f64,
}

impl RudoPyramid {
    pub fn area(&self) -> f64 {
        self.surface_area()
    }

    pub fn distance(&self, point: &Dot3D) -> f64 {
        let top_center = self
            .base
            .centroid()
            .to_3d()
            .translate(&Dot3D::new(0.0, 0.0, self.height));
        top_center.distance_to(point)
    }

    pub fn is_inner(&self, point: &Dot3D) -> bool {
        self.base.is_inner(point) && self.height > point.z
    }

    pub fn is_outer(&self, point: &Dot3D) -> bool {
        self.base.is_outer(point) || self.height < point.z
    }

    pub fn is_outside(&self, point: &Dot3D) -> bool {
        self.base.is_outside(point) || point.z < 0.0
    }

    pub fn new(base: Triangle, height: f64) -> RudoPyramid {
        RudoPyramid { base, height }
    }

    pub fn new(dot3d: &Dot3D) -> RudoPyramid {
        // Define the base square or rectangle with two sides perpendicular to the z-axis
        let mut base = Triangle::new(Dot::new(dot3d.x - 1.0, dot3d.y - 1.0),
                                      Dot::new(dot3d.x + 1.0, dot3d.y - 1.0),
                                      Dot::new(dot3d.x - 1.0, dot3d.y + 1.0));
        // If the apex is on the z-axis, define the base square with two sides perpendicular to the x-axis
        if dot3d.z == 0.0 {
            base = Triangle::new(Dot::new(dot3d.x - 1.0, dot3d.y),
                                  Dot::new(dot3d.x + 1.0, dot3d.y),
                                  Dot::new(dot3d.x - 1.0, dot3d.y + 2.0));
        }
        let height = dot3d.z;
        RudoPyramid { base, height }
    }

    pub fn surface_area(&self) -> f64 {
        let a = self.base.a.distance_to(&self.base.b);
        let b = self.base.b.distance_to(&self.base.c);
        let c = self.base.c.distance_to(&self.base.a);
        let base_area = self.base.area();
        let slant_height = (self.height.powi(2) + ((a * b * c) / (4.0 * base_area)).powi(2)).sqrt();
        base_area + 0.5 * (a + b + c) * slant_height
    }

    pub fn volume(&self) -> f64 {
        let base_area = self.base.area();
        base_area * self.height / 3.0
    }
}
