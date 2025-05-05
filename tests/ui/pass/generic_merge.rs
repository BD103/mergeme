use mergeme::Merge;

#[derive(Merge)]
#[partial(PartialNamedData)]
// `T: Merge<T>` means any type that can be merged with itself.
struct NamedData<T: Merge<T>>
{
    name: String,
    #[strategy(merge)]
    data: T,
}

fn main() {}
