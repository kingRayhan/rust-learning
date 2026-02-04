fn main() {
    let a: [i8; 5] = [1, 2, 3];
    println!("a: {:?}", a);

    let slice: &[i8] = &a[1..3];
    println!("slice: {:?}", slice);
}
