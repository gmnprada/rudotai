use crate::dot::Dot;

#[derive(Debug, Copy, Clone)]
pub struct Triangle {
    pub a: Dot,
    pub b: Dot,
    pub c: Dot,
}

impl Triangle {
    pub fn new(a: Dot, b: Dot, c: Dot) -> Triangle {
        Triangle { a, b, c }
    }

    pub fn area(&self) -> f64 {
        let ab = self.b.distance_to(&self.a);
        let bc = self.c.distance_to(&self.b);
        let ca = self.a.distance_to(&self.c);
        let s = (ab + bc + ca) / 2.0;
        (s * (s - ab) * (s - bc) * (s - ca)).sqrt()
    }

    pub fn is_inner(&self, point: &Dot) -> bool {
        let a = self.area();
        let a1 = Triangle::new(self.a, self.b, *point).area();
        let a2 = Triangle::new(self.b, self.c, *point).area();
        let a3 = Triangle::new(self.c, self.a, *point).area();
        (a - a1 - a2 - a3).abs() < 1e-6
    }

    pub fn is_outer(&self, point: &Dot) -> bool {
        let a = self.area();
        let a1 = Triangle::new(self.a, self.b, *point).area();
        let a2 = Triangle::new(self.b, self.c, *point).area();
        let a3 = Triangle::new(self.c, self.a, *point).area();
        (a - a1 - a2 - a3).abs() > 1e-6
    }
}