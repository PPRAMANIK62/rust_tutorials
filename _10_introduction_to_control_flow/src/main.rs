// ========================================
// Control Flow in Rust
// 1. Conditions [if ... else]
// 2. Repeating actions [loops]

// ========================================
// 1. Conditions [if ... else]
fn main() {
    let age: u16 = 19;
    if age >= 18 {
        println!("You can drive a car!");
    } else {
        println!("You can't drive a car!");
    }

    // Multiple conditions with else if
    let number: i32 = 6;
    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");        
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3 or 2");
    }

    // using if in a let statement
    let condition: bool = true;
    // the data type in each arm of if and else statements has to be same
    let number: i32 = if condition { 5 } else { 6 };
    println!("The number is: {}", number);
}
