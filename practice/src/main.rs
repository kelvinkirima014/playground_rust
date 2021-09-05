struct Point <T> 
{
    x: T,
    y: T,
}
impl<T> Point<T>{
    fn x(&self) -> &T {
        &self.x
    }
}
fn main() {
    let p = Point {x: 9, y: 10};
    println!("p.x = {}", p.x);
    println!("p.y = {}", p.y);
}