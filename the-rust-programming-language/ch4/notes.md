# Ownershop

## What is ownership

Ownership is a discipline for ensuring the safety of Rust programs.

Rust does not check at runtime whether a variable is defined before being used. Instead, Rust checks at compile-time.

A foundational goal of Rust is to ensure that your programs never have undefined behavior.

Undefined behavior is especially dangerous for low-level programs with direct access to memory.

A secondary goal of Rust is to prevent undefined behavior at compile-time instead of run-time.

## Rust Does Not Permit Manual Memory Management

Instead, Rust automatically frees a box’s heap memory.

```
Box deallocation principle (almost correct):

If a variable owns a box, when Rust deallocates the variable’s frame, then Rust deallocates the box’s heap memory.
```

However, only using ownership to manage memory would be not convenient.

So Rust provides `references` to allow you to refer to some value without taking ownership of it.

```rust
fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2); // note the ampersands
    let s = format!("{} {}", m1, m2);
}

fn greet(g1: &String, g2: &String) { // note the ampersands
    println!("{} {}!", g1, g2);
}
```

`g1` and `g2` are references to `m1` and `m2`, and the ownership does not change.

(You can condider g1 and g2 as a pointer)

## Rust Avoids Simultaneous Aliasing and Mutation

Pointer Safety Principle: data should never be aliased and mutated at the same time.

Data can be aliased. Data can be mutated.

But data cannot be both aliased and mutated.

## References Change Permissions on Places

variables have three kinds of permissions on their data: read, write, and own(RWO).

These permissions don’t exist at runtime, only within the compiler.

They describe how the compiler “thinks” about your program before the program is executed.

## Fix Ownership Errors
