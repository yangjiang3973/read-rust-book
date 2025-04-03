fn main() {
    let shoes = vec![1, 2, 3, 4, 5];
    let large_shoes: Vec<&i32> = shoes.iter().filter(|x: &&i32| **x > 3).collect();
    println!("largeShoes: {:?}", large_shoes);
}

/*

*/
