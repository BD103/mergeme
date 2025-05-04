# Merge Me!

A derivable trait that assists with merging data together.

This crate provides the `Merge` trait and derive macro. The trait is a simple interface for combining data together, while the derive macro lets you easily implement this trait for any struct.

## Installation

You can install `mergeme` with `cargo add`:

```sh
cargo add mergeme
```

If you do not need `#[derive(Merge)]` and wish to avoid depending on `syn`, you may disable the default features:

```sh
cargo add mergeme --no-default-features
```

Once you have installed `mergeme`, be sure to [read the documentation](https://docs.rs/mergeme) for both the trait and the derive macro. Their interfaces are simple and their docs are extensive!

## Merging in Action

```rust
use mergeme::Merge;

#[derive(Merge)]
#[partial(PartialPerson)]
struct Person {
    name: String,
    age: u16,
    #[strategy(merge)]
    friends: Vec<String>,
}

let person = Person {
    name: "Janette".to_string(),
    age: 19,
    friends: vec!["Lou".to_string()],
};

// Change Janette's age to be 25 and add a friend, but preserve her original name.
let partial = PartialPerson {
    name: None,
    age: Some(25),
    friends: Some(vec!["Kylie".to_string()]),
};

let merged = person.merge(partial);

assert_eq!(merged.name, "Janette");
assert_eq!(merged.age, 25);
assert_eq!(merged.friends, ["Lou", "Kylie"]);
```
