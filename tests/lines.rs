use vecto_rs::{trig::edge_function, positional::Vector};

#[test]
fn edge()
{
    assert!(edge_function((Vector::new2(0.0, 0.0), Vector::new2(0.0, 5.0)), Vector::new2(5.0, 0.0)) > 0.0);
    assert!(edge_function((Vector::new2(0.0, 0.0), Vector::new2(0.0, 5.0)), Vector::new2(-5.0, 0.0)) < 0.0);
    assert!(edge_function((Vector::new2(0.0, 0.0), Vector::new2(0.0, 5.0)), Vector::new2(0.0, 0.0)) == 0.0);
}