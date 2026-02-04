fn main() {
    let isAdmin: bool = true;
    let name: &str = "John";

    if isAdmin {
        println!("Welcome, {name}");
    } else {
        println!("Access denied");
    }

    let x = 10;
    let i1: i8 = 100;
    let i2: i16 = 1000;
    let i3: i32 = 10000;
    let i4: i64 = 100000;
    let i5: i128 = 1000000;
    let i6: isize = 10000000;

    println!("x: {}", x);
    println!("i1: {}", i1);
    println!("i2: {}", i2);
    println!("i3: {}", i3);
    println!("i4: {}", i4);
    println!("i5: {}", i5);
    println!("i6: {}", i6);

    // Character
    let c1: char = 'a';
    let emoji: char = '🚀';
    println!("c1: {}", c1);
    println!("emoji: {}", emoji);

    // Min, Max values
    let min_i8: i8 = i8::MIN;
    let max_i8: i8 = i8::MAX;
    println!("min_i8: {}", min_i8);
    println!("max_i8: {}", max_i8);

    let min_u8: u8 = u8::MIN;
    let max_u8: u8 = u8::MAX;
    println!("min_u8: {}", min_u8);
    println!("max_u8: {}", max_u8);

    let min_i16: i16 = i16::MIN;
    let max_i16: i16 = i16::MAX;
    println!("min_i16: {}", min_i16);
    println!("max_i16: {}", max_i16);
}
