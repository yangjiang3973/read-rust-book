`const` can be used in the global scope, and `let` can only be used in a function.

## Shadowing

We can change the type of the value but reuse the same name.

```rust
let spaces = "   ";
let spaces = spaces.len();
```

## Types

two data type subsets: `scalar` and `compound`

Rust has four primary `scalar` types: integers, floating-point numbers, Booleans, and characters(use single quotes).

Rust has two primitive `compound` types: tuples and arrays.

tuple's elements can have different types, but array's elements must be the same type.

And array's length is fixed. If you need a collection that can grow or shrink, you can use a `vector`.

## if

no brackets, but must have a block.

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

You must be explicit and always provide if with a Boolean as its condition.

You cannot use number or any other non-Boolean type as a condition.

## loop

Rust has three kinds of loops: `loop`, `while`, and `for`.

```rust
fn main() {
    loop {
        println!("again!");
    }
}
```
