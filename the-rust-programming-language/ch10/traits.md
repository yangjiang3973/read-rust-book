# Traits: Defining Shared Behavior

A trait defines the functionality a particular type has and can share with other types.

`Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.`

## Define a trait

```rust
pub trait Summary {
    fn summarize(&self) -> String;  // behaviors of the types that implement this trait
}

// or default implementation
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

A trait can have multiple methods in its body: the method signatures are listed one per line, and each line ends in a semicolon.

## Implementing a Trait

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

## Lifetime

Every reference in Rust has a lifetime, which is the scope for which that reference is valid.

The main aim of lifetimes is to prevent dangling references, which cause a program to reference data other than the data it’s intended to reference.

```rust
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}

// x doesn't live long enough
```

### Lifetime Annotation

```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
