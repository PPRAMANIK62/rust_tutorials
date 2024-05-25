// ========================================
// Functions
// Entry point function for rust
// any function / variables should be written in snake case
// snake case: eg. hello_world
// kebab case: eg. hello-world

fn main() {
    hello_world();
    tell_height(182);
    human_id("Purbayan", 19, 182.3);

    let x = {
        let price = 5;
        let qty = 10;
        // instead of return price * qty;
        // not ending with ; (Expression)
        price * qty 
    };
    println!("Total price: {}", x);

    let sum: i32 = add(10, 20);
    println!("Sum: {}", sum);
    println!("Sum: {}", add(10, 20));

    // Calling the BMI function
    let weight: f64 = 70.0;
    let height: f64 = 1.82;
    let bmi = calculate_bmi(weight, height);
    // 2 decimal points
    println!("Your BMI is: {:.2}", bmi);
}

// HOISTING - can call function anywhere in your code
fn hello_world() {
    println!("Hello, Rust!");
}

// you can insert input values
fn tell_height(height: u32){
    println!("Your height is: {} cm", height);
}

// you can insert more than one value
fn human_id(name: &str, age: u32, height: f32){
    println!("Name: {}, Age: {}, Height: {} cm", name, age, height);
}

// function returning values
fn add(a: i32, b: i32) -> i32 {
    a + b
    // expression (not ending with ;)
}

// Expressions and statements

// 1. Expression: Anything that returns a value.
// eg. 5, true & false, add(4,5), if condition {value} else {value}, ({code})

// 2. Statement: Anything that does not return a value.
// almost all statements in rust end with ;
// let y = let x = 10;
// 1. variable declarations: let x = 5;
// 2. functional definitions: fn foo() {}
// 3. control flow statements: if condition {code} else {code}, while condition {code}, etc.

// BMI = weight(kg) / height(m)^2
fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}
