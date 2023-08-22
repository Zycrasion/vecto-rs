use vecto_rs::{QuadTree, Vec2};

#[test]
fn quadtree_simple()
{
    let mut tree : QuadTree<bool> = QuadTree::new(0.0, 0.0, 500.0, 500.0, 80, 0.0);
    
    for i in 0..10_000
    {
        tree.add((i % 2) == 0, Vec2((i % 500) as f32, (i % 500) as f32));
    }

    let cell = tree.query(Vec2(230.0, 230.0));

    println!("{}", cell.len());
    for c in cell
    {
        println!("{}: {}", c.0, c.1);
    }

    for i in 0..1000
    {
        tree.remove(Vec2((i % 500) as f32, (i % 500) as f32));
    }

    tree.prune();
}