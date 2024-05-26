// ========================================
// STRUCTS
// are used to name and package related values similar to tuples
fn main() {
    // tuple
    let _rect: (i32, i32) = (200, 500);

    // unlike tuples each field in struct is named, this provides clear identification of what each value represents

    // struct
    struct Book {
        title: String,
        author: String,
        pages: u32,
        available: bool,
    }

    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let user1 = User {
        active: true,
        username: String::from("Kratos"),
        email: String::from("kratos.com"),
        sign_in_count: 1,
    };
    // user1.email = String::from("anotheremail@email.com");
    println!("User email is: {}", user1.email);

    // return a struct from a function
    fn build_user(username: String, email: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }
    let user2 = build_user(String::from("Kratos"), String::from("kratos.com"));

    // create instances from other instances
    let user3 = User {
        email: String::from("another@email.com"),
        ..user1
    };

    // tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);
    let origin = Point(0, 0, 0);

    // unit like struct
    struct AlwaysEqual;
    let subject = AlwaysEqual;
}
