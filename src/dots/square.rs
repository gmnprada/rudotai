use crate::dot::Dot;
use crate::dots::Triangle;

#[derive(Debug, Copy, Clone)]
pub struct Square {
    pub a: Dot,
    pub b: Dot,
    pub c: Dot,
    pub d: Dot,
}

impl Square {
    pub fn new(a: Dot, b: Dot, c: Dot, d: Dot) -> Square {
        Square { a, b, c, d }
    }

    pub fn area(&self) -> f64 {
        let ab = self.b.distance_to(&self.a);
        let bc = self.c.distance_to(&self.b);
        let s = (ab + bc) / 2.0;
        s * s
    }

    pub fn is_inner(&self, point: &Dot) -> bool {
        let t1 = Triangle::new(self.a, self.b, *point);
        let t2 = Triangle::new(self.b, self.c, *point);
        let t3 = Triangle::new(self.c, self.d, *point);
        let t4 = Triangle::new(self.d, self.a, *point);
        t1.area() + t2.area() + t3.area() + t4.area() - self.area() < 1e-6
    }

    pub fn is_outer(&self, point: &Dot) -> bool {
        let t1 = Triangle::new(self.a, self.b, *point);
        let t2 = Triangle::new(self.b, self.c, *point);
        let t3 = Triangle::new(self.c, self.d, *point);
        let t4 = Triangle::new(self.d, self.a, *point);
        t1.area() + t2.area() + t3.area() + t4.area() - self.area() > 1e-6
    }
}
