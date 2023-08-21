use vecto_rs::{Vec2, Vec3};

#[test]
fn vec_cmp()
{
    assert!(Vec2(0.0,0.0) < Vec2(1.0,1.0));
    assert!(!(Vec2(0.0,0.0) > Vec2(1.0,1.0)));
    assert!(!(Vec2(0.0,0.0) < Vec2(0.0,1.0)));
    assert!(Vec2(0.0,0.0) < Vec2(1.0,1.0));
    assert!(Vec2(1.2, 0.0) == Vec2(1.2, 0.0));
}

#[test]
fn vec3_cmp()
{
    assert!(Vec3(0.0,0.0, 0.0) < Vec3(1.0,1.0, 1.0));
    assert!(!(Vec3(0.0,0.0, 0.0) > Vec3(1.0,1.0, 1.0)));
    assert!(!(Vec3(0.0,0.0, 1.0) < Vec3(0.0,1.0, 0.0)));
    assert!(Vec3(0.0,0.0, 0.0) < Vec3(1.0,1.0, 1.0));
    assert!(Vec3(1.2, 0.0, 1.0) == Vec3(1.2, 0.0, 1.0));
}