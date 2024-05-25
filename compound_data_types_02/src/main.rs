// ========================================
// Compound data types
// Arrays, tuples, slices and strings (slice string)

// ========================================
// Arrays
fn main() {
    let numbers: [i32; 5] = [1,2,3,4,5];
    // println!("Number array: {}", numbers);
    // will give an error
    // 2 types of format -> display and debuggable
    // It's a diaplay format

    // converting to debuggable format
    println!("Number array: {:?}", numbers);

    // let mix: [i32; 4] = [1,2,"apple", true];
    // error
    // expecting homogeneous data type

    let fruits: [&str; 3] = ["apple", "banana", "orange"];
    println!("Fruits array: {:?}", fruits);
    println!("Fruits array 1st element: {}", fruits[0]);

    // ========================================
    // Tuples
    let human: (String, i32, bool) = ("Me".to_string(), 20, false);
    println!("Human tuple: {:?}", human);  
    let my_mix_tuple: (&str, i32, bool, [i32; 5]) = ("Kratos", 23, true, [1,2,3,4,5]);
    println!("My mix tuple: {:?}", my_mix_tuple);

    // ========================================
    // Slices
    // dynamically sized view into a contagious sequence of elements
    let number_slice: &[i32; 5] = &[1,2,3,4,5];
    println!("Number slice: {:?}", number_slice);

    let animals_slice: &[&str; 3] = &["lion", "elephant", "crocodile"];
    println!("Animals slice: {:?}", animals_slice);

    let books_slice: &[&String; 3] = &[&"IT".to_string(), &"Harry Potter".to_string(), &"ZEN".to_string()];
    println!("Books slice: {:?}", books_slice);

    // ========================================
    // Strings vs Strings Slices
    // 1. Strings -> variable size, mutable, owned string types (not borrowed), stored in the heap
    // any data type in rust is by default immutable
    let mut stone_cold: String = String::from("Hell, ");
    println!("Stone cold says: {}", stone_cold);
    stone_cold.push_str("Yeah");
    println!("Stone cold says: {}", stone_cold);

    // 2. String slice (&str) -> stored in stack, not owned, a reference to a portion of string that is stored somewhere in our code, immutable
    // good for memory efficiency
    // Used when we want to work with string data without taking ownership of it
    // have specific size and no number of bytes to the stack
    // stack is quicker than heap
    // but stack can't have any mutable data types, but heap can have dynamic mutable data types
    let string: String = String::from("Hello, World");
    let slice: &str = &string[0..5];
    // reffering to previously declared string
    println!("String slice: {}", slice);

    // Rust cleans any memory allocated to any variable
    // you can't use slice varible outside the main function as it is declared inside main 
}

// fn print() {
//     println!("SLICE: {}", slice)
// }
// error: cannot find value `slice` in this scope