use vecto_rs::{spatial::AABB, linear::*};

#[test]
fn test_aabb()
{
    let a = AABB::new(Vector::new2(0.0, 0.0),Vector::new2(5.0, 5.0));
    let b = AABB::new(Vector::new2(2.5, 0.0),Vector::new2(5.0, 5.0));

    assert!(a.intersection(b));
}

#[test]
fn test_aabb1()
{
    let a = AABB::new(Vector::new2(5.1, 0.0),Vector::new2(5.0, 5.0));
    let b = AABB::new(Vector::new2(0.0, 0.0),Vector::new2(5.0, 5.0));

    assert!(!a.intersection(b));
}

#[test]
fn point_inside()
{
    let start = Vector::new2(0.0, 0.0);
    let size = Vector::new2(5.0, 5.0);

    let aabb = AABB::new(start, size);
    assert!(aabb.point_inside(size / 2.0));
}

#[test]
fn new()
{
    let start = Vector::new2(0.0, 0.0);
    let size = Vector::new2(5.0, 5.0);

    let aabb = AABB::new(start, size);

    assert_eq!(aabb.start, start);
    assert_eq!(aabb.size, size);
}