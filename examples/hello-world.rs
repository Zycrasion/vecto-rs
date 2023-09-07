use vecto_rs::Vector;

fn main()
{
    // Working out distance between 2 points

    let a = Vector::new2(0.0, 3.0);
    let b = Vector::new2(4.0, 5.0);

    // All vector functions are non-mutable
    let c = a.dist(&b);
    println!("{} {} {}", a, b, c)
}