use crate::dot3d::Dot3D;
use crate::vector3d::Vector3D;

#[derive(Debug, Copy, Clone)]
pub struct RudoParallelogram {
    pub bottom_left: Dot3D,
    pub bottom_right: Dot3D,
    pub top_left: Dot3D,
}

impl RudoParallelogram {
    pub fn area(&self) -> f64 {
        let v1 = Vector3D::from_two_dots(self.bottom_left, self.bottom_right);
        let v2 = Vector3D::from_two_dots(self.bottom_left, self.top_left);
        v1.cross_product(&v2).length()
    }

    pub fn height(&self) -> f64 {
        let v1 = Vector3D::from_two_dots(self.bottom_left, self.bottom_right);
        let v2 = Vector3D::from_two_dots(self.bottom_left, self.top_left);
        let normal = v1.cross_product(&v2);
        let area = normal.length();
        area / v1.length()
    }

    pub fn is_inner(&self, point: &Dot3D) -> bool {
        let v1 = Vector3D::from_two_dots(self.bottom_left, self.bottom_right);
        let v2 = Vector3D::from_two_dots(self.bottom_left, self.top_left);
        let normal = v1.cross_product(&v2);
        let p1 = Vector3D::from_two_dots(self.bottom_left, *point);
        let p2 = Vector3D::from_two_dots(self.bottom_right, *point);
        let p3 = Vector3D::from_two_dots(self.top_left, *point);
        let cross1 = normal.cross_product(&p1);
        let cross2 = normal.cross_product(&p2);
        let cross3 = normal.cross_product(&p3);
        let dot1 = cross1.dot_product(&cross2);
        let dot2 = cross2.dot_product(&cross3);
        let dot3 = cross3.dot_product(&cross1);
        dot1 >= 0.0 && dot2 >= 0.0 && dot3 >= 0.0
    }

    pub fn is_outer(&self, point: &Dot3D) -> bool {
        !self.is_inner(point)
    }

    pub fn is_outside(&self, point: &Dot3D) -> bool {
        let normal = self.normal();
        let p1 = Vector3D::from_two_dots(self.bottom_left, *point);
        normal.dot_product(&p1) > 0.0
    }

    pub fn new(bottom_left: Dot3D, bottom_right: Dot3D, top_left: Dot3D) -> RudoParallelogram {
        RudoParallelogram {
            bottom_left,
            bottom_right,
            top_left,
        }
    }

    pub fn normal(&self) -> Vector3D {
        let v1 = Vector3D::from_two_dots(self.bottom_left, self.bottom_right);
        let v2 = Vector3D::from_two_dots(self.bottom_left, self.top_left);
        v1.cross_product(&v2)
    }

    pub fn distance(&self, point: &Dot3D) -> f64 {
        // Calculate the normal vector of the parallelogram
        let v1 = Vector3D::from_two_dots(self.bottom_left, self.bottom_right);
        let v2 = Vector3D::from_two_dots(self.bottom_left, self.top_left);
        let normal = v1.cross_product(&v2).normalize();
        
        // Calculate the vector from the point to the bottom-left corner of the parallelogram
        let v3 = Vector3D::from_two_dots(self.bottom_left, *point);
        
        // Calculate the distance between the point and the parallelogram
        let distance = v3.dot_product(&normal).abs();
        distance
    }
}
