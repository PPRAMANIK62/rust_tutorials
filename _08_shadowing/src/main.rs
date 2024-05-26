// ========================================
// Shadowing
// we can declare a new variable with the same name as a previous variable
// In this case, the second variable is what the compiler will see when we use the name of the variable.
// So, the second variable overshadows the first variable
fn main() {
    let x: i32 = 5;
    // Shadowing is NOT THE SAME as marking a variable as mutable.
    let x: i32 = x + 1;
    println!("The new value of variable x is: {}", x);

    // can change the type of the variable also
    let spaces: &str = "   ";
    let spaces: usize = spaces.len();
    println!("The number of spaces: {}", spaces);
}
