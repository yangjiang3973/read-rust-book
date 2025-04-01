# Erroe handling

Rust has the `panic!` macro.

```rust
fn main() {
    panic!("crash and burn");
}
```

## Recoverable Errors

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Use match to handle the `Result` type:

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
}
```

### unwrap and expect

These two are shortcuts for handling `Result`:

If successful, `unwrap` returns the value inside the `Ok`,

else, it calls `panic!`.

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}
```

But actually most Rustaceans choose `expect`:

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
```

Its functionality is the same as `unwrap`, but you can customize the error message.

## Propagating Errors

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```

This pattern of propagating errors is so common in Rust that Rust provides the question mark operator ? to make this easier.

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```

The ? operator eliminates a lot of boilerplate and makes this function’s implementation simpler.

## When to use panic!
