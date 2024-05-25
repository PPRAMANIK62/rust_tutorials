// ========================================
// Constants
// Constants vs Variables
// In constants you are not allowed to use the keyword 'mut'
fn main() {
    println!("Hello, world!");
    let x: i32 = 5;
    // const mut y: i32 = 10;
    // Error: const cannot be mutable
    // Constants should be capitalized

    const Y: i32 = 10;
    println!("The value of variable x: {}", x);
    println!("The value of constant Y: {}", Y);

    println!("The value of pi is: {}", PI);
}

// You can declare a constant here with a type annotation
const PI: f32 = 3.14;
