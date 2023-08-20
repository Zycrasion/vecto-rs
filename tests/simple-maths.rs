use vecto_rs::Vec2;

#[test]
fn addition()
{
    let a = Vec2(1.0, 3.0);
    let b = Vec2(4.0, 5.0);

    let c = a + b;

    assert_eq!(c, Vec2(5.0, 8.0));
}

#[test]
fn addition_3_vectors()
{
    let a = Vec2(1.0, 3.0);
    let b = Vec2(4.0, 5.0);
    let c = Vec2(5.0, 8.0);

    let d = a + b + c;

    assert_eq!(d, Vec2(10.0, 16.0));
}

#[test]
fn subtraction()
{
    let a = Vec2(1.0, 3.0);
    let b = Vec2(4.0, 5.0);

    let c = a - b;

    assert_eq!(c, Vec2(-3.0, -2.0));
}

#[test]
fn subtraction_3_vectors()
{
    let a = Vec2(1.0, 3.0);
    let b = Vec2(4.0, 5.0);
    let c = Vec2(5.0, 8.0);

    let d = a - b - c;

    assert_eq!(d, Vec2(-8.0, -10.0));
}