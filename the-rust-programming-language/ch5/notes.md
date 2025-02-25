# Struct

## Creating Instances from Other Instances

```rust
fn main() {
   let user1 = User {
      email: String::from("someone@example.com"),
      username: String::from("someusername123"),
      active: true,
      sign_in_count: 1,
   };
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

we can no longer use user1 after creating user2 because the String in the username field of user1 was moved into user2.

## Tuple Structs

Rust also supports structs that look similar to tuples, called tuple structs.

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```
