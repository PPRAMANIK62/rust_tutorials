// ========================================
// Repetition with loops
// Doing thisngs over and over
fn main() {
    // loop keyword

    // infinite loop
    // loop {
    //     println!("Hello, world!");
    // }

    // non-infinite loop
    let mut counter: i32 = 0;
    let result: i32 = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {}", result);

    // LOOP LABELS to disambiguate between multiple loops
    // when dealing with nested loops, the default behaviour is that the break and continue statements are going to apply to the innermost loop
    let mut count: i32 = 0;
    'counting_up: loop {
        println!("count: {}", count);
        let mut remaining: i32 = 10;

        loop {
            println!("remaining: {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    // WHILE loop
    let mut number: i32 = 3;
    while number!= 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // FOR loop
    let a: [i32; 5] = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // Ranges
    for number in (1..4).rev() {
        println!("{}", number);
    }
    println!("LIFTOFF!!!");
}
