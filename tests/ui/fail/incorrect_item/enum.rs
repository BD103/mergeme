use mergeme::Merge;

#[derive(Merge)]
#[partial(PartialChoice)]
enum Choice {
    A,
    B,
}

fn main() {}
