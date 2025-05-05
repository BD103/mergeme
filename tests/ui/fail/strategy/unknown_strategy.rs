use mergeme::Merge;

#[derive(Merge)]
#[partial(PartialDog)]
struct Dog {
    name: String,
    // `add` is not a valid strategy.
    #[strategy(add)]
    age: u16,
}

fn main() {}
