use mergeme::Merge;

#[derive(Merge)]
#[partial(PartialNamedData)]
struct NamedData<T> {
    name: String,
    data: T,
}

fn main() {}
