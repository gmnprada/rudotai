use crate::dot3d::Dot3D;

pub struct RudoCube {
    dots: Vec<Dot3D>,
}

impl RudoCube {

    fn distance(&self, other: &Dot3D) -> f64 {
        let min_distance = self.dots[0].distance_to(other);
        for dot in &self.dots {
            let distance = dot.distance_to(other);
            if distance < min_distance {
                return distance;
            }
        }
        min_distance
    }

    fn get_center(&self) -> Dot3D {
        let sum = self
            .dots
            .iter()
            .fold(Dot3D::new(0.0, 0.0, 0.0), |acc, dot| acc + *dot);
        Dot3D::new(
            sum.x / self.dots.len() as f64,
            sum.y / self.dots.len() as f64,
            sum.z / self.dots.len() as f64,
        )
    }

    fn get_height(&self) -> f64 {
        let min_z = self
            .dots
            .iter()
            .map(|dot| dot.z)
            .fold(f64::INFINITY, f64::min);
        let max_z = self
            .dots
            .iter()
            .map(|dot| dot.z)
            .fold(f64::NEG_INFINITY, f64::max);
        max_z - min_z
    }

    fn get_volume(&self) -> f64 {
        let a = self.dots[0].distance_to(&self.dots[1]);
        let b = self.dots[0].distance_to(&self.dots[2]);
        let c = self.dots[0].distance_to(&self.dots[4]);
        a * b * c
    }

    fn intersect(&self, other: &RudoCube) -> bool {
        for dot in &self.dots {
            if other.is_inner(dot) {
                return true;
            }
        }
        false
    }

    fn is_inner(&self, other: &Dot3D) -> bool {
        for dot in &self.dots {
            if dot == other {
                return true;
            }
        }
        false
    }

    fn is_outer(&self, other: &Dot3D) -> bool {
        for dot in &self.dots {
            if dot == other {
                return false;
            }
        }
        true
    }

    pub fn new(center: Dot3D, size: f64) -> RudoCube {
        let half_size = size / 2.0;
        let dots = vec![
            Dot3D::new(
                center.x - half_size,
                center.y - half_size,
                center.z - half_size,
            ),
            Dot3D::new(
                center.x + half_size,
                center.y - half_size,
                center.z - half_size,
            ),
            Dot3D::new(
                center.x - half_size,
                center.y + half_size,
                center.z - half_size,
            ),
            Dot3D::new(
                center.x + half_size,
                center.y + half_size,
                center.z - half_size,
            ),
            Dot3D::new(
                center.x - half_size,
                center.y - half_size,
                center.z + half_size,
            ),
            Dot3D::new(
                center.x + half_size,
                center.y - half_size,
                center.z + half_size,
            ),
            Dot3D::new(
                center.x - half_size,
                center.y + half_size,
                center.z + half_size,
            ),
            Dot3D::new(
                center.x + half_size,
                center.y + half_size,
                center.z + half_size,
            ),
        ];
        RudoCube { dots }
    }

    fn rotate(&mut self, angle: f64) {
        let center = self.get_center();
        let rotation_matrix = Vector3D::new(0.0, 0.0, 1.0).rotation_matrix(angle);
        for dot in &mut self.dots {
            *dot -= center;
            *dot = rotation_matrix * *dot;
            *dot += center;
        }
    }

    fn translate(&mut self, vector: Vector3D) {
        for dot in &mut self.dots {
            *dot += vector;
        }
    }
}
