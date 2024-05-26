// ========================================
// Error handling techniques [2 approaches]
// 1. Option<T> 
// 2. Result<T,E>
fn main() {
    // Approach 1. Option<T> 
    // it is basically an enum, this is used when a value might not be present
    // it is going to avoid the pitfalls of null references
    enum Option<T> {
        Some(T), // represents a value
        None, // represents no value
        // Option is going to check something and if this something is positive then it will return some with that type, otherwise none will be returned
    }

    // Approach 2. Result<T,E>
    enum Result<T,E> {
        Ok(T), // represents a value
        Err(E), // represents an error
    }

    // Example on Option<T>
    let result = divide_option(10.0, 2.0);
    match result {
        Some(x) => println!("Result: {}", x),
        None => println!("Cannot divide by zero"),
    }

    // Example on Result<T,E>
    let result = divide_result(10.0, 0.0);
    match result {
        Ok(res) => println!("Result: {}", res),
        Err(err) => println!("Error: {}", err),
    }
}

fn divide_option(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn divide_result(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(numerator / denominator)
    }
}