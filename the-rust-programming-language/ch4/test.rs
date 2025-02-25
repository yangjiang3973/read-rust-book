// fn incr(n: &mut i32) {
//     *n += 1;
// }
// fn main() {
//     let mut n = 1;
//     incr(&mut n);
//     println!("{n}");
// }

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();
    println!("{}", word);
}
