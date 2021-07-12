// variables in rust are immutable unless otherwise specified
fn  main() {
    //we use keyword `mut` to create a mutable variable
    let mut x = 5;
    println!("The value of x is: {}", x);
     x = 6;
    println!("The value of x is: {}", x);
    //Rust also allows us to reuse a variable name via 'shadowing'
    let y = 10;
    println!("The value of y is: {}", y);
    let y = "hey there!";
    println! ("The value of y is: {}", y)
}
// Differences btwn Variables and Constant
/*
Like immutable variables, constant are values that are bound to
a name and not allowed to change. 
There are a few differences between them though:
1. You aren't allowed to use `mut` with constants- they're always immutable by default
2. You declare constants using `const` keyword, instead of `let`
3. The `type` of the value must be annotated
4. Constants are declared in any scope, including global scope
--- this makes them useful for values that many part of code need to know about
5. Constants may be set only to ma constant expression, not the result of a function call
...or any other value that could only be computed at runtime
*/
// example of a constant
const MAX_POINTS: u32 = 500_000;
//Rust's naming convention for constants is to use  all uppercase
//with underscores btwn words,
//underscores may be inserted in numeric literals to improve readability
