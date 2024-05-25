// ========================================
// Primitive data types
// int, float, bool, char

// Integer
// Rust has signed (+ and -) and unsigned integer (only +) types of different sizes
// i8, i16, i32, i64, i128: Signed integers
// u8, u16, u32, u64, u128: Unsigned integers

fn main(){
    let x: i32 = -42;
    let y: u64 = 100;
    println!("Signed integer: {}", x);
    println!("Unsigned integer: {}", y);

// defference bewtween i32 (32 bits) and i64 (64 bits)
// range: 
// i32 -> 2147483647
// i64 -> 9223372036854775807
    let e: i32 = 2147483647;
    let i: i64 = 9223372036854775807;
    println!("i32 max value: {}", e);
    println!("i64 max value: {}", i);

// ========================================
// Floats [Floating point types]
// f32, f64
    let pi: f32 = 3.14;
    println!("Value of pi: {}", pi);

// ========================================
// Boolean values: true, false
    let is_snowing: bool = true;
    println!("Is it snowing? {}", is_snowing);

// ========================================
// Character type -> char
    let letter: char = 'a';
    println!("Letter: {}", letter);
}