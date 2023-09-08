use vecto_rs::positional::Vector;

#[test]
fn addition()
{
    let a = Vector::new2(1.0, 3.0);
    let b = Vector::new2(4.0, 5.0);

    let c = a + b;

    assert_eq!(c, Vector::new2(5.0, 8.0));
}

#[test]
fn addition_3_vectors()
{
    let a = Vector::new2(1.0, 3.0);
    let b = Vector::new2(4.0, 5.0);
    let c = Vector::new2(5.0, 8.0);

    let d = a + b + c;

    assert_eq!(d, Vector::new2(10.0, 16.0));
}

#[test]
fn subtraction()
{
    let a = Vector::new2(1.0, 3.0);
    let b = Vector::new2(4.0, 5.0);

    let c = a - b;

    assert_eq!(c, Vector::new2(-3.0, -2.0));
}

#[test]
fn subtraction_3_vectors()
{
    let a = Vector::new2(1.0, 3.0);
    let b = Vector::new2(4.0, 5.0);
    let c = Vector::new2(5.0, 8.0);

    let d = a - b - c;

    assert_eq!(d, Vector::new2(-8.0, -10.0));
}