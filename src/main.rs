fn main() {
    // Default is f64, because in modern CPU's
    // f64 is as fast as f32 and has more precision
    let float64 = 3.0;
    println!("float64 is {}", float64);

    let float32: f32 = 3.0;
    println!("float32 is {}", float32);
}
