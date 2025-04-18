# Smart Pointers

Smart pointers, on the other hand, are data structures that act like a pointer but also have additional metadata and capabilities.

For example, `String` is a smart pointer because it stores some metadata about the string.

Smart pointers are usually implemented using structs and implement the `Deref` and `Drop` traits.

The most common smart pointers in the standard library:

`Box<T>` for allocating values on the heap

`Rc<T>`, a reference counting type that enables multiple ownership

`Ref<T>` and `RefMut<T>`, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time

## Box<T>

Boxes allow you to store data on the heap rather than the stack.

What remains on the stack is the pointer to the heap data.

When to use Box?

When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size

When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so

When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type

### Using a Box<T> to Store Data on the Heap

```rust
fn main() {
    let b = Box::new(5);  // on the heap, not on the stack
    println!("b = {b}");
}
```

But usually, we do not need to save i32 on the heap.

### Enabling Recursive Types with Boxes

A value of `recursive type` can have another value of the same type as part of itself.

There is a problem with recursive types: the compiler needs to know the size of each type at compile time.

Because boxes have a known size, we can enable recursive types by inserting a box in the recursive type definition.

```rust
enum List {
    Cons(i32, List),
    Nil,
}
//////////////
use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
// cannot compile, need to use Box
// change to:
enum List {
    Cons(i32, Box<List>),  // use Box to store the recursive type
    Nil,
}
use crate::List::{Cons, Nil};
fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```

## Rc<T>

Rc is an abbreviation for `Reference Counting`.

The `Rc<T>` type keeps track of the number of references to a value to determine whether or not the value is still in use.

If there are zero references to a value, the value can be cleaned up without any references becoming invalid.

```rust
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
```

Every time we call Rc::clone, the reference count to the data within the `Rc<List>` will increase.

## RefCell<T>

`Interior mutability` is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data;

The pattern uses `unsafe` code inside a data structure, because this action is disallowed by the borrowing rules.

`Unsafe` code indicates to the compiler that we’re checking the rules manually

### What makes RefCell<T> different from `Box<T>`?

Recall the borrowing rules:

1. You can have either one mutable reference or any number of immutable references to a piece of data at a time.

2. References must always be valid.

`Box<T>` enforces these rules at compile time.

`RefCell<T>` enforces these rules at runtime.

Checking the borrowing rules at compile time is the best choice in the majority of cases, which is why this is Rust’s default.

Rust compiler is conservative.

The `RefCell<T>` type is useful when you’re sure your code follows the borrowing rules but the compiler is unable to understand and guarantee that.

### Interior Mutability: A Mutable Borrow to an Immutable Value

this code won’t compile:

```rust
fn main() {
    let x = 5;
    let y = &mut x;
}
```
