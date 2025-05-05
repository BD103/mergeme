use mergeme::Merge;

#[derive(Merge)]
// Missing `#[partial(...)]`.
struct Config {
    name: String,
    dependencies: Vec<String>,
}

fn main() {}
