use vecto_rs::{linear::*, trig::Line};

#[test]
fn edge()
{
    let a = Line(Vector::new2(0.0, 0.0), Vector::new2(0.0, 5.0));
    let b = Line(Vector::new2(0.0, 0.0), Vector::new2(0.0, 5.0));
    let c = Line(Vector::new2(0.0, 0.0), Vector::new2(0.0, 5.0));

    assert!(a.edge_function(Vector::new2(5.0, 0.0)) > 0.0);
    assert!(b.edge_function(Vector::new2(-5.0, 0.0)) < 0.0);
    assert!(c.edge_function(Vector::new2(0.0, 0.0)) == 0.0);
}