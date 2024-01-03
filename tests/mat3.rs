use vecto_rs::linear::Mat3;
use vecto_rs::Float;

#[test]
fn mat3_clone()
{
    let mat3 = Mat3::from_array(
        [
            1., 2., 3.,
            4., 5., 6.,
            7., 8., 9.
        ]
    );

    assert_eq!(mat3, mat3.clone());
}

#[test]
fn mat3_default()
{
    let mat3 = Mat3::default();

    assert_eq!(mat3, Mat3::new());
}

#[test]
fn mat3_partial_eq()
{
    let mat3 = Mat3::default();

    assert!(mat3 == Mat3::new());
}

#[test]
fn mat3_debug()
{
    let mat3 = Mat3::default();

    println!("{:#?}", mat3);
}

#[test]
fn mat3_new()
{
    let mat3 = Mat3::new();
    for i in mat3.get_contents()
    {
        if i != 0.
        {
            panic!();
        }
    }
}

#[test]
fn mat3_identity()
{
    let mat3 = Mat3::identity();
    assert_eq!(mat3.get(0,0), 1.);
    assert_eq!(mat3.get(1,1), 1.);
    assert_eq!(mat3.get(2,2), 1.);
}

#[test]
fn mat3_is_nan()
{
    let mut mat3 = Mat3::new();
    assert!(!mat3.is_nan());
    mat3.change(0, 0, Float::NAN);
    assert!(mat3.is_nan());
}

#[test]
fn mat3_transpose()
{
    let mat3 = Mat3::from_array(
        [
            1., 2., 3.,
            4., 5., 6.,
            7., 8., 9.
        ]
    );

    let mat3_transposed = Mat3::from_array(
        [
            1., 4., 7.,
            2., 5., 8.,
            3., 6., 9.
        ]
    );

    assert_eq!(mat3.transpose(), mat3_transposed);
}

#[test]
fn mat3_change()
{
    let mut mat3 = Mat3::from_array(
        [
            1., 2., 3.,
            4., 5., 6.,
            7., 8., 9.
        ]
    );

    let mat3_expected = Mat3::from_array(
        [
            1., 2., 3.,
            4., 6., 6.,
            7., 8., 9.
        ]
    );

    mat3.change(1, 1, 6.);

    assert_eq!(mat3, mat3_expected);
}