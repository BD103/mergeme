use mergeme::Merge;

#[derive(Merge)]
// Name the partial version of this type `PartialConfig`.
#[partial(PartialConfig)]
struct Config {
    // This field will be overwritten when merged. `#[strategy(overwrite)]` may be omitted.
    #[strategy(overwrite)]
    name: String,

    // This field will also be overwritten when merged.
    version: u32,

    // This field will be combined when merged.
    #[strategy(merge)]
    dependencies: Vec<String>,
}

fn main() {}
