use vecto_rs::spatial::AABB;
use vecto_rs::{spatial::QuadTree, linear::*};
use vecto_rs::Float;

#[test]
fn quadtree_simple()
{
    let mut tree : QuadTree = QuadTree::new(AABB::new(Vector::new2(0.,0.), Vector::new2(500., 500.)));
    
    for i in 0..100
    {
        tree.insert(i, &Vector::new2((i % 500) as Float, (i % 500) as Float));
    }

    // let cell = tree.query(Vector::new2(230.0, 230.0));

    // println!("{}", cell.len());
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