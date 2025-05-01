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
