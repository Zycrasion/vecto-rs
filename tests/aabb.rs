use vecto_rs::{AABB, Vector};

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