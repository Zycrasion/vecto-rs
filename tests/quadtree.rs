use vecto_rs::spatial::AABB;
use vecto_rs::{spatial::QuadTree, linear::*};
use vecto_rs::Float;

#[test]
fn quadtree_simple()
{
    let mut tree : QuadTree = QuadTree::new(AABB::new(Vector::new2(0.,0.), Vector::new2(500., 500.)));
    
    for i in 0..100
    {
        tree.insert(i, &Vector::new2(i as f32, 0.)).expect("Error inserting indice");
    }

    let cell = tree.query(&Vector::new2(50.0, 0.0)).unwrap();

    assert_eq!(cell.len(), 100);
    
    tree.insert(100, &Vector::new2(0., 1.)).expect("Error inserting indice");

    let cell = tree.query(&Vector::new2(0.0, 0.0)).unwrap();
    println!("{:#?}", cell);
    assert_eq!(cell.len(), 64);

    // for c in cell
    // {
    //     println!("{}: {}", c.0, c.1);
    // }

    // for i in 0..100
    // {
    //     tree.remove(Vector::new2((i % 500) as Float, (i % 500) as Float));
    // }

    // tree.prune();
}