use std::io;

fn main() {
  //ask user to enter first number
  println!("Enter first int bitch!");
  let mut int1 = String::new();
    io::stdin().read_line(& mut int1).unwrap();
  //enter second number bitch
  println!("Enter second int bitch!");
  let mut int2 = String::new();
    io::stdin().read_line(& mut int2).unwrap();
  //convert str to u32
  let a: u32 = int1.trim().parse::<u32>().unwrap();
  let b: u32 = int2.trim().parse::<u32>().unwrap();

  let sum = a + b;

  println!("the sum is: {}", sum);


}
