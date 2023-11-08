use vecto_rs::linear::{Mat4, vector3};

#[test]
fn mult()
{
    let mut a = Mat4::identity();
    a.translate(vector3(10., 2., 3.));
    let mut b = Mat4::identity();

    let c = a * b;
    assert!(!c.is_nan())
}