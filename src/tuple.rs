#[allow(unused)]

fn main() {
    let t: (i8, bool, &str) = (100, true, "John");
    let (a, b, c) = t; // destructuring
    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);

    // Skipping values
    let t2: (i8, bool, &str) = (100, true, "John");
    let (a, _, _) = t2;
    println!("a: {}", a);

    // accessing values by index
    let t3: (i8, bool, &str) = (100, true, "John");
    println!("t3.0: {}", t3.0);
    println!("t3.1: {}", t3.1);
    println!("t3.2: {}", t3.2);
}
