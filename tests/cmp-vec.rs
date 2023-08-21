use vecto_rs::Vec2;

#[test]
fn vec_cmp()
{
    assert!(Vec2(0.0,0.0) < Vec2(1.0,1.0));
    assert!(!(Vec2(0.0,0.0) > Vec2(1.0,1.0)));
    assert!(!(Vec2(0.0,0.0) < Vec2(0.0,1.0)));
    assert!(Vec2(0.0,0.0) < Vec2(1.0,1.0));
    assert!(Vec2(1.2, 0.0) == Vec2(1.2, 0.0));
}