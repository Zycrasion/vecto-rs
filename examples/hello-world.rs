use vecto_rs::Vec2;

fn main()
{
    // Working out distance between 2 points

    let a = Vec2(0.0, 3.0);
    let b = Vec2(4.0, 5.0);

    // All vector functions are non-mutable
    let c = a.dist(&b);
    println!("{} {} {}", a, b, c)
}