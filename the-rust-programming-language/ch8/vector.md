# Common Collections

Each kind of collection has different capabilities and costs,

and choosing an appropriate one for your current situation is a skill you’ll develop over time.

A vector allows you to store a variable number of values next to each other.

A string is a collection of characters.

A hash map allows you to associate a value with a specific key.

## Vector

Vectors can only store values of the same type.

```rust
let v: Vec<i32> = Vec::new();
```

Or you can use the `vec!` macro to create a vector with initial values:

```rust
let v = vec![1, 2, 3];  // The integer type is i32 because that’s the default integer type.
```

Updeta:

```rust
let mut v = Vec::new();
v.push(5);
```

Read elements:

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {third}");

let third: Option<&i32> = v.get(2);
match third {
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element."),
}
```

This code does not work:

```rust
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);

println!("The first element is: {first}");
```

Because `first` borrows the reference to the first element of the vector,

and then we push a new element to the vector, which may cause the vector to reallocate and change the reference.

Iterate:

```rust
let v = vec![100, 32, 57];
for n_ref in &v {
    // n_ref has type &i32
    let n_plus_one: i32 = *n_ref + 1;
    println!("{n_plus_one}");
}
```

Enum to Store Multiple Types:

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```
