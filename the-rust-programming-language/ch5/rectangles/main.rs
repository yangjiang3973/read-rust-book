// 1
// fn main() {
//     let w = 30;
//     let h = 30;
//     println!("The area of the rectangle is {} square pixels.", area(w, h));
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// 2
// fn main() {
//     let rect1 = (30, 50);
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// 3
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rec1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rec1)
//     );
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// 4
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rec1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//     println!("rec1 is {:?}", rec1);
// }

// 5
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     // The &self is actually short for self: &Self
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );
// }
