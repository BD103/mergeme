use mergeme::Merge;

#[derive(Merge)]
#[partial(PartialConfig1)]
#[partial(PartialConfig2)]
struct Config {
    name: String,
    dependencies: Vec<String>,
}

fn main() {}
