# Storing UTF-8 Encoded Text with Strings

## Create

```rust
// 1
let mut s = String::new();

// 2
let data = "initial contents";

let s = data.to_string();

// the method also works on a literal directly:
let s = "initial contents".to_string();

// 3
let s = String::from("initial contents");
```

## Update

A String can grow in size and its contents can change.

```rust
let mut s = String::from("foo");
s.push_str("bar");

// add only one char
s.push('l');
```

Concatenation:

```rust
let s1 = String::from("foo");
let s2 = String::from("bar");
let s3 = s1 + &s2; // s1 has been moved here and can no longer be used

// fn add(self, s: &str) -> String {}
```

the compiler can coerce the &String argument into a &str

so s2 will still be a valid String after this operation.

but it will take the ownership of s1, and it appends a copy of the contents of s2 to s1.

Another way to concatenate strings is to use the format! macro:

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
```

`format!` returns a `String` and does not take ownership of any of its parameters.

## Index

This code will not compile:

```rust
let s1 = String::from("hello");
let h = s1[0];
```

Rust does not support indexing strings.

### Internal Representation

For example, `let hello = String::from("Hola");` makes a string in 4 bytes.

Each character here is 1 byte when encoded in UTF-8.

But `let hello = String::from("Здравствуйте");` makes a string in 24 bytes instead of 12,

because each Unicode scalar value in that string takes 2 bytes of storage.

So you cannot use indexing to access individual characters in a string.

### Use Slicing

```rust
let hello = "Здравствуйте";

let s = &hello[0..4]; // the first four bytes of the string 'Зд'
```

Make sure you want characters or bytes.

```rust
for c in "Зд".chars() {
    println!("{c}");  // З, д
}

for b in "Зд".bytes() {
    println!("{b}"); // 208, 151, 208, 180
}
```
