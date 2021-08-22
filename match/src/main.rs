fn main() {
    let number: u32 = 13;

    println!("The number is: {}", number);
    match number {
        1 => println!("one"),
        2| 3| 5|7 => println!("Prime"),
        13..=19 => println!("Teen"),
        _ => println!("Not interested") //handle all possible cases
    }
   }