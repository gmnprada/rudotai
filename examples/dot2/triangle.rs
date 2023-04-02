fn main() {
    let a = Dot::new(0.0, 0.0);
    let b = Dot::new(1.0, 0.0);
    let c = Dot::new(0.0, 1.0);
    let triangle = Triangle::new(a, b, c);

    let point = Dot::new(0.5, 0.5);
    let is_inner = triangle.is_inner(&point);
}
