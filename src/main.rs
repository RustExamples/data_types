fn main() {
    /*
        Having "-" sign in front of numbers for unsigned int raises 
        "error[E0600]: cannot apply unary operator `-` to type `u16`"
    */
    let int8: i8 = 1;
    let uint8: u8 = 1;
    println!("int8 is {} and uint8 is {}", int8, uint8);

    let int16: i16 = 1;
    let uint16: u16 = 1;
    println!("int16 is {} and uint16 is {}", int16, uint16);

    let int32: i32 = 1;
    let uint32: u32 = 1;
    println!("int32 is {} and uint32 is {}", int32, uint32);

    let int64: i64 = 1;
    let uint64: u64 = 1;
    println!("int64 is {} and uint64 is {}", int64, uint64);

    // Stores signed or unsigned numbers based on system architecture (32 or 64 bit)
    let iarch: isize = 1;
    let uarch: usize = 1;
    println!("iarch is {} and uarch is {}", iarch, uarch);

    //Number literals

    let decimal_literal = 98_000;
    println!("decimal_literal is {}", decimal_literal);
    
    let hexadecimal_literal = 0xFE;
    println!("hexadecimal_literal is {}", hexadecimal_literal);

    let octal_literal = 0o77;
    println!("octal_literal is {}", octal_literal);

    let binary_literal = 0b1110_1111;
    println!("binary_literal is {}", binary_literal);

    // Only for "u8" data type
    let byte_literal = b'A';
    println!("byte_literal is {}", byte_literal);
}
