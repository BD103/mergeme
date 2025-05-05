use mergeme::Merge;

#[derive(Merge)]
#[partial(PartialConfig)]
struct Config(bool, u8, Vec<String>);

fn main() {}
