// ========================================
// Collection Types
// Vectors, UTF-8, Hashmaps

use std::collections::HashMap;

// ========================================
// VECTORS
// allow us to store more than one value in a single data structure that puts all the values newxt to each other in memory.
// can only store values of the same type
fn main() {
    let _v: Vec<i32> = Vec::new();
    let mut _vec: Vec<i32> = vec![1,2,3];

    _vec.push(4);
    println!("{:?}", _vec);

    let third = &_vec[2];
    println!("The third element is: {}", third);

    let third = _v.get(2);
    match third {
      Some(third) => println!("The third element is: {}", third),
      None => println!("There is no third element."),
    }

// ========================================
// UTF-8 Encoding
// String: 
// rust has only one string type in the core language, which is in the string slice 'str' format that is usually borrowed from '&str'
// The 'String' type, which is provided by rust's standard library rather coded in the core language, is a growable, mutable, owned, UTF-8 encoded string type.
// both 'String', and sutring slices are UTF-8 encoded
    
    // 1.
    let _s = "whatever".to_string();
    // 2.
    let _s = String::from("whatever");
    // Mutate the variable 
    let mut s = String::from("foo");
    // for string double quotes "" and for char single quotes ''
    s.push_str("bar");
    s.push('!');

    println!("The value of s: {}", s);

    // if you want to combine strings, use the + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("s3: {}", s3);

// ========================================
// Hash Maps: HashMap<K,V>
// stores a mapping of keys of type K to values of type V using a hashing function, which determines how it places these keys and values into memory.
// in other languages it is named as hash, map, object, hash table, dectionary, or associative array etc.
// are useful when you want to look up data not by using an index, as you can with vectors, but by using a key that can be of any type.

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Score of Team {team_name}: {score}");

    for (key, value) in &scores {
      println!("{key}: {value}");
    }
}
