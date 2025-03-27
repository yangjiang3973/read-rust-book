# Hash Maps

## Create

all of the keys must have the same type, and all of the values must have the same type.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

## Read

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// read
let team_name = String::from("Blue");
// get method returns an Option<&V>
// use copied to get an Option<i32> rather than an Option<&i32>
let score = scores.get(&team_name).copied().unwrap_or(0);
```

Loop:

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{key}: {value}");
}
```

## Update

```rust

```
