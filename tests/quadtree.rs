pub use vecto_rs::prelude::*;

#[test]
fn quadtree_simple()
{
    let mut tree : QuadTree<bool> = QuadTree::new(0.0, 0.0, 500.0, 500.0, 80, 0.0, 10);
    
    for i in 0..100
    {
        tree.add((i % 2) == 0, Vector::new2((i % 500) as Float, (i % 500) as Float));
    }

    let cell = tree.query(Vector::new2(230.0, 230.0));

    println!("{}", cell.len());
    for c in cell
    {
        println!("{}: {}", c.0, c.1);
    }

    for i in 0..100
    {
        tree.remove(Vector::new2((i % 500) as Float, (i % 500) as Float));
    }

    tree.prune();
}