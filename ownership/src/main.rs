use std::io;
fn main() {
  let int1: u32;
  let int2: u32;
  let sum: u32;

  io::stdin().read_u32(&int1)
  .read_u32(&int2)
  .expect("Failed to read user input")
}