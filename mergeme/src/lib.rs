#![doc = include_str!("../../README.md")]
#![no_std]

#[cfg(feature = "derive")]
pub use mergeme_derive::Merge;

pub trait Merge<Partial>: Sized {
    fn merge_in_place(&mut self, other: Partial);

    fn merge(mut self, other: Partial) -> Self {
        self.merge_in_place(other);
        self
    }
}

/// Implements `Merge` for any type that implements `Extend`.
/// 
/// This means that most standard library collection types can be merged with anything iterable over
/// the same type. Some highlights include:
/// 
/// - `Vec<T>`
/// - `String`
/// - `HashMap<K, V>`
/// - `HashSet<T>`
/// - `BTreeMap<K, V>`
/// - `BTreeSet<T>`
/// - `PathBuf`
impl<Base, Partial, Item> Merge<Partial> for Base
where
    Base: Extend<Item>,
    Partial: IntoIterator<Item = Item>
{
    fn merge_in_place(&mut self, other: Partial) {
        self.extend(other);
    }
}
