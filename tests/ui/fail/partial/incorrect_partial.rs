use mergeme::Merge;

#[derive(Merge)]
struct Config {
    // This should annotate the struct, not a field.
    #[partial(PartialConfig)]
    name: String,
    dependencies: Vec<String>,
}

fn main() {}
