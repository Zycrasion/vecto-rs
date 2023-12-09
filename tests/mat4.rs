use vecto_rs::linear::{Mat4, vector3};

#[test]
fn mult()
{
    let mut a = Mat4::identity();
    a.translate(vector3(10., 2., 3.));
    let mut b = Mat4::identity();

    let c = a * b;
    assert!(!c.is_nan());
    assert_eq!(c, a)
}

#[test]
fn from_array_and_transpose()
{
    // Covers both transpose, from_array, get_row_major and get_row_major
    // Because they all depend on these functions
    let array = [
        5.,	2.,	32., 5.,
        4.,	3.,	34., 6., 
        9.,	3.,	 5., 7.,
        1., 5., 59., 6.,
    ];
    
    let mat = Mat4::from_array(array);

    assert_eq!(mat.to_string(), "5 2 32 5 \n4 3 34 6 \n9 3 5 7 \n1 5 59 6 \n");
    assert_eq!(mat.transpose().to_string(), "5 4 9 1 \n2 3 3 5 \n32 34 5 59 \n5 6 7 6 \n")
}