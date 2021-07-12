// if expressions
fn main() {
    let number = 3;
    //the condition MUST be a boolean
    //if number = 5; --> throws error, expected bool, found integer
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    exp();
    
}
// handling multiple conditions with else if
fn example() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
//using if in a let statement
//because if is an expression, we can use it on the right side of a let statement
fn another_function() {
    let condition = true;
    // the values that could be results must be of the same type
    // let number = if condition { 1 } else {"two"}//throws error
    let number = if condition { 1 } else {2};
    println!("The value of number is: {}",number);
}
// REPETION WITH LOOPS
//the *loop* keyword tells Rust to execute a block of
//code overe and over again until you explicitly tell it to stop
fn count_() {
    let mut counter =0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}",result); // prints 20
}
// conditional loops with WHILE
// while the condition is true, the loop runs
//when the condition ceases to be true, the program 
//calls `break`, stopping the loop
fn exp(){
    let mut num = 3;
    while num < 10 {
        println!("{}!", num);
        num += 1;
    }
    println!("LiftOff!!");
}

//looping through a collection over a for loop
// a for loop is a concise way to execute some code for each item in a collection.
//the for loop is the safest and most concise looop construct in rust
fn for_loop() {
    let a = [4,7,10,12,15,20,];
    for element in a.iter() {
        println!("the value is:{}", element);
    }
}
//here is how a countdown would look like using
//a for loop and a method to reverse called, `rev`.
fn count_down (){
    for number in (1..10).rev() {
        println!("{}", number);
    }
    println!("Liftoff!");
}