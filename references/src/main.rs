fn main() {
    let a = String::from("hello");
    let str_len = get_str_length(&a);
    println!("The length of the string is {}", str_len);
}

fn get_str_length(s: &str) -> usize {
    s.len()
}