use vecto_rs::linear::{vector3, Mat4, Vector, VectorTrait};

#[test]
fn mult_identity() {
    let mut a = Mat4::identity();
    a.translate(vector3(10., 2., 3.));
    let b = Mat4::identity();

    let c = a * b;
    assert!(!c.is_nan());
    assert_eq!(c, a)
}

#[test]
fn mult_big() {
    let a = Mat4::from_array([
        1.,	 2.,  3.,    4.,
        5.,	 6.,  7.,    8.,
        9., 10., 11.,	12.,
       13., 14., 15.,	16.,
    ]);

    let b = Mat4::from_array([
        17.,	18.,	19.,	20.,
        21.,	22.,	23.,	24.,
        25.,	26.,	27.,	28.,
        29.,	30.,	31.,	32.,
    ]);

    let c = a * b;

    assert_eq!(c.to_string(), "250 260 270 280 \n618 644 670 696 \n986 1028 1070 1112 \n1354 1412 1470 1528 \n")
}

#[test]
fn from_array_and_transpose() {
    // Covers both transpose, from_array, get_row_major and get_row_major
    // Because they all depend on these functions
    let array = [
        5., 2., 32., 5., 4., 3., 34., 6., 9., 3., 5., 7., 1., 5., 59., 6.,
    ];

    let mat = Mat4::from_array(array);

    assert_eq!(
        mat.to_string(),
        "5 2 32 5 \n4 3 34 6 \n9 3 5 7 \n1 5 59 6 \n"
    );
    assert_eq!(
        mat.transpose().to_string(),
        "5 4 9 1 \n2 3 3 5 \n32 34 5 59 \n5 6 7 6 \n"
    )
}

#[test]
fn look_at_rh() {
    let from = Vector::new3(1., 1., 1.);
    let to = Vector::new3(0., 0., 0.);
    let up = Vector::new3(0., 1., 0.);

    let matrix = Mat4::look_at_rh(&from, &to, &up);

    println!("{}", matrix);
}