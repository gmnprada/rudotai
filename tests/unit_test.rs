use rudotai::dots::triangle::Triangle;
use rudotai::dot::Dot;
use rudotai::dots3d::pyramid::Pyramid;

#[test]
fn test_pyramid_volume() {
    // Define a base triangle and pyramid height
    let base = Triangle::new(Dot::new(0 as f64,0 as f64),Dot::new(1.0,0 as f64),Dot::new(0 as f64,1.0));
    let height = 2.0;

    // Create a pyramid and calculate its volume
    let pyramid = Pyramid::new(base, height);
    let volume = pyramid.volume();

    // Assert that the calculated volume is correct (within some small tolerance)
    let expected_volume = 1.0 / 3.0;
    let tolerance = 1e-6;
    assert!((volume - expected_volume).abs() < tolerance);
}


