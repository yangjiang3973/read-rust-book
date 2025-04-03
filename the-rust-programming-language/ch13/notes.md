# Closures: Anonymous Functions

Closures don’t usually require you to annotate the types of the parameters or the return value like fn functions do.

Within these limited contexts, the compiler can infer the types of the parameters and the return type.

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

Closures can capture values from their environment in three ways：

borrowing immutably, borrowing mutably, and taking ownership

The closure will decide which of these to use based on what the body of the function does with the captured values.

# Iterators

```rust
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

for val in v1_iter {
    println!("Got: {val}");
}
```

The iterator is stored in the v1_iter variable.

## The Iterator Trait and the next Method

All iterators implement a trait named Iterator that is defined in the standard library.

Call `next` to get the next value from the iterator.

```rust
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();  // need to be mutable if you use next

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
```

## Methods that Produce Other Iterators

```rust
let v1: Vec<i32> = vec![1, 2, 3];

v1.iter().map(|x| x + 1);  // create a new iterator
```

```rust
let v1: Vec<i32> = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();  // use `collect` to collect the resulting values into a collection data type.

assert_eq!(v2, vec![2, 3, 4]);
```

Use `into_iter` if you need to take the ownership.

for example, if you use `filter`:

```rust
shoes.into_iter().filter(|s| s.size == shoe_size).collect()
```

## Diff between `iter` and `into_iter`

iter produces an iterator that borrows the values of the collection.

(a list of references to the element, <&T>)

into_iter produces an iterator that takes ownership of the values in the collection.

(a list of owned values, <T>)

Use `iter` if you want to keep the original collection intact.

Use `into_iter` if you want to consume the original collection with ownership.

In addition, `iter_mut` produces an iterator over mutable references to the values in the collection.

If you want to modify the original collection, use `iter_mut`.
