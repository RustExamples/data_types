fn main() {
    let tup: (i32, f64, u8) = (10, 3.14, 8);

    // This is called "destructuring"
    let (x, y, z) = tup;

    // Tuples annotation is optional
    let tup = (10, 3.14, 8);

    // Tuple values can be accessed by their index
    let x = tup.0;

    let y = tup.1;

    let z = tup.2;

    println!("x is {}, y is {}, z is {}", x, y, z);
}
