use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    println!("Hello, world!");
    let plant = Asparagus {};
    println!("plant: {:?}", plant);
}
