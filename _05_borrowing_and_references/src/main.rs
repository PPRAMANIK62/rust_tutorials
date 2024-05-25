// ========================================
// Safety and Proformance
// Safety refers to the prevention of certain types of bugs and errors that commonly occur in other programming languages like C, C++
// like null pointer D referencing, dangling pointers, buffer overflows and data races

// ========================================
// Understanding References
// Referencing in rust enables you to borrow values without taking the ownership
// can be both mutable (allows borrowing with the ability tomodify the data) and immutable
// create reference by adding '&' at the start

fn main() {
    let x: i32 = 5;
    let r: &i32 =  &x;
    println!("Owner x: {}", x);
    println!("Reference r: {}", r);

    // *r += 1;
    // error: (not mutable) can be fixed like this
    let mut x: i32 = 5;
    let r: &mut i32 = &mut x;
    *r += 1;
    *r -= 3;
    println!("Modified value of reference r: {}", r);

    // you can have one mutable reference or any number of immutable references
    let mut account: BankAccount = BankAccount{
        owner: String::from("John"),
        balance: 1000.0,
    };
    // Immutable borrow to check the balance
    account.check_balance();

    // Mutable borrow to withdraw money
    account.withdraw(50.5);

    account.check_balance()
}

// ========================================
// STRUCT: 
// A data structure that allows you to group multiple fields together under one name

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!("Account owned by {} has a balance of {}", self.owner, self.balance);
    }

    // here one mutable borrow is withdraw and one immutable borrow is check_balance, but they are in their own scopes, not in common scope, so they don't overlap. So succesfully compiled without any errors
}