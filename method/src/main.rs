#[derive(Debug)]
struct Node{
    num: u32,
    distr: u32,
}

impl Node {
    fn decentralized(&self) -> u32{
        self.num * self.distr
    }
}
fn main() {
    let network = Node {
        num: 14,
        distr: 9,
    };
    println!("The decentralization of the network is {}", network.decentralized());
}
