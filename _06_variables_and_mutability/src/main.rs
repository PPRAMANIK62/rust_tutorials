// ========================================
// Variables and Mutability
// When a variable is immutable, once a value is bound to a name, you can't change that value.
// Every variable in rust is by default immutable.
fn main() {
    println!("Hello, world!");
    let mut a: i32 = 5;
    println!("The value of a is: {}", a);

    a = 10;
    println!("The new value of a is: {}", a);
}
