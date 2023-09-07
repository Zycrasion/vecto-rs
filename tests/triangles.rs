use vecto_rs::{maths::Triangle2D, Vector};

fn tri_test_inside(tri : Triangle2D, point : Vector, expected : bool)
{
    assert_eq!(tri.point_inside_triangle(point), expected);
}

#[test]
fn tri_test_inside1()
{
    tri_test_inside(Triangle2D::new(Vector::new2(0.0,0.0), Vector::new2(0.0,10.0), Vector::new2(10.0,0.0)), Vector::new2(5.0, 2.5), true);
}

fn tri_test_barycentric(tri : Triangle2D, point : Vector, expected : (f32, f32, f32))
{
    assert_eq!(tri.barycentric_coordinates(point).unwrap(), expected)
}

#[test]
fn tri_test_barycentric1()
{
    tri_test_barycentric(Triangle2D::new(Vector::new2(0.0,0.0), Vector::new2(0.0,10.0), Vector::new2(10.0,0.0)), Vector::new2(5.0, 2.5), (0.25, 0.25, 0.5));
    let coords = Triangle2D::new(Vector::new2(0.0,0.0), Vector::new2(0.0,10.0), Vector::new2(10.0,0.0)).barycentric_coordinates(Vector::new2(5.0, 2.5)).unwrap();
    assert_eq!(
        coords.0 + coords.1 + coords.2,
        1.0
    )
}