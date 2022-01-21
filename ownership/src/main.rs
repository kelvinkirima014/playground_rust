use std::io;
fn main() {
  let int1;
  let int2;
  let sum;

  io::stdin().read_line(&int1)
  .read_line(&int2)
  .expect("Failed to read user input");

  sum = get_sum(int1, int2);
  println!("the sum is: {}", sum);
}
fn get_sum(s: u32, t: u32) -> u32 {
  s + t
}