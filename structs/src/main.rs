
#[derive (Debug)]
struct Person {
    name: String,
    height: u32,
}
fn main() {
    let kelvin = Person {
        name: String::from("Kelvin"),
        height: 182,
    };
    println!("The name is {}", kelvin.name);
}