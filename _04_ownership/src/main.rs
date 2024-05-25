// ========================================
// Ownership, Borrowing and References

// ========================================
// Ownership
// C, C++ -> Memory Management Control Issue
// Garbage Collector solved this issue, but created a new issue -> Slow Performance [stopping / resuming the program]
// This operation is done at run time in the background
// if it wants to clean up the memory it will stop the program -> Freezing

// What is Ownership?
// Every value has a single owner [every variable has one value, and it is it's sole owner]

// ========================================
// Ownership rules
// 1. each value in rust has a variable that's its owner
// 2. there can be only one owner at a time
// 3. when the owner goes out of scope, the value will be dropped

fn main() {
// Example: 1. each value in rust has a variable that's its owner
    let s1: String = String:: from("RUST");
    let len: usize = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

// Example: 2. there can be only one owner at a time
    let s2: String = s1;
    // ownership shifted to s2

    // Error
    // println!("{}", s1);
    println!("{}", s2);

// Example: 3. when the owner goes out of scope, the value will be dropped
}
// s1 goes out of scope and its value will be dropped

fn calculate_length(s: &String) -> usize {
    s.len()
}
