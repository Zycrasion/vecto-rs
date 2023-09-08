use vecto_rs::positional::Vector;

#[test]
fn vector_cmp()
{
    assert!(Vector::new3(0.0,0.0, 0.0) < Vector::new3(1.0,1.0, 1.0));
    assert!(!(Vector::new3(0.0,0.0, 0.0) > Vector::new3(1.0,1.0, 1.0)));
    assert!(!(Vector::new3(0.0,0.0, 1.0) < Vector::new3(0.0,1.0, 0.0)));
    assert!(Vector::new3(0.0,0.0, 0.0) < Vector::new3(1.0,1.0, 1.0));
    assert!(Vector::new3(1.2, 0.0, 1.0) == Vector::new3(1.2, 0.0, 1.0));
}