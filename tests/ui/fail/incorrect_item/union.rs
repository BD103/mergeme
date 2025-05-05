use mergeme::Merge;

#[derive(Merge)]
#[partial(PartialChoice)]
union Choice {
    a: u32,
    b: f32,
}

fn main() {}
