#![doc = include_str!("../../README.md")]
#![no_std]

#[cfg(feature = "derive")]
pub use mergeme_derive::Merge;

/// A trait for two types that can be merged into one.
///
/// The `Partial` type is the type being merged into `Self`. It is intended to represent a _subset_
/// of the data in `Self`, though it is not a strict requirement.
///
/// # Examples
///
/// This is an example using the [`Merge`](derive@Merge) derive macro. Please see
/// [its documentation](derive@Merge) for further details.
///
/// ```
/// # use mergeme::Merge;
/// #
/// #[derive(Merge)]
/// // Name the partial version of this type `PartialPerson`. This attribute is required.
/// #[partial(PartialPerson)]
/// struct Person {
///     name: String,
///     age: u16,
///     // Instead of overwriting the list of friends when merged, combine them together.
///     #[strategy(merge)]
///     friends: Vec<String>,
/// }
///
/// let person = Person {
///     name: "Janette".to_string(),
///     age: 19,
///     friends: vec!["Lou".to_string()],
/// };
///
/// // Change Janette's age to be 25 and add a friend, but preserve her original name.
/// let partial = PartialPerson {
///     name: None,
///     age: Some(25),
///     friends: Some(vec!["Kylie".to_string()]),
/// };
///
/// let merged = person.merge(partial);
///
/// assert_eq!(merged.name, "Janette");
/// assert_eq!(merged.age, 25);
/// assert_eq!(merged.friends, ["Lou", "Kylie"]);
/// ```
///
/// This is an example implementing [`Merge`] from scratch. It is equivalent to the prior example.
///
/// ```
/// # use mergeme::Merge;
/// #
/// struct Person {
///     name: String,
///     age: u16,
///     friends: Vec<String>,
/// }
///
/// // This represents a subset of `Person` and can be merged with it. When a field is `Some`, it
/// // will overwrite the `Person`'s value. When a field is `None`, it will preserve the original
/// // value.
/// struct PartialPerson {
///     name: Option<String>,
///     age: Option<u16>,
///     friends: Option<Vec<String>>,
/// }
///
/// impl Merge<PartialPerson> for Person {
///     fn merge_in_place(&mut self, other: PartialPerson) {
///         if let Some(name) = other.name {
///             self.name = name;
///         }
///
///         if let Some(age) = other.age {
///             self.age = age;
///         }
///
///         // Instead of overwriting the list of friends, merge the two together.
///         if let Some(friends) = other.friends {
///             self.friends.merge_in_place(friends);
///         }
///     }
/// }
///
/// let person = Person {
///     name: "Janette".to_string(),
///     age: 19,
///     friends: vec!["Lou".to_string()],
/// };
///
/// // Change Janette's age to be 25 and add a friend, but preserve her original name.
/// let partial = PartialPerson {
///     name: None,
///     age: Some(25),
///     friends: Some(vec!["Kylie".to_string()]),
/// };
///
/// let merged = person.merge(partial);
///
/// assert_eq!(merged.name, "Janette");
/// assert_eq!(merged.age, 25);
/// assert_eq!(merged.friends, ["Lou", "Kylie"]);
/// ```
pub trait Merge<Partial>: Sized {
    /// Merges `Self` and `Partial` together, mutating `Self` in place.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mergeme::Merge;
    /// #
    /// #[derive(Merge)]
    /// #[partial(PartialCat)]
    /// struct Cat {
    ///     name: String,
    ///     age: u16,
    /// }
    ///
    /// let mut whiskers = Cat {
    ///     name: "Whiskers".to_string(),
    ///     age: 4,
    /// };
    ///
    /// let partial = PartialCat {
    ///     name: None,
    ///     age: Some(5),
    /// };
    ///
    /// whiskers.merge_in_place(partial);
    ///
    /// assert_eq!(whiskers.name, "Whiskers");
    /// assert_eq!(whiskers.age, 5);
    /// ```
    fn merge_in_place(&mut self, other: Partial);

    /// Merges `Self` and `Partial` together, returning a new `Self`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mergeme::Merge;
    /// #
    /// #[derive(Merge)]
    /// #[partial(PartialCat)]
    /// struct Cat {
    ///     name: String,
    ///     age: u16,
    /// }
    ///
    /// let mut whiskers = Cat {
    ///     name: "Whiskers".to_string(),
    ///     age: 4,
    /// };
    ///
    /// let partial = PartialCat {
    ///     name: Some("Toast".to_string()),
    ///     age: None,
    /// };
    ///
    /// let toast = whiskers.merge(partial);
    ///
    /// assert_eq!(toast.name, "Toast");
    /// assert_eq!(toast.age, 4);
    /// ```
    fn merge(mut self, other: Partial) -> Self {
        self.merge_in_place(other);
        self
    }
}

/// Implements [`Merge`] for any type that implements [`Extend`].
///
/// This means that most standard library collection types can be merged with anything iterable over
/// the same type. Some highlights include:
///
/// - `Vec<T>` with `IntoIterator<Item = T>`
/// - `String` with `IntoIterator<Item = &str>` or `IntoIterator<Item = char>`
/// - `HashMap<K, V>` with `IntoIterator<Item = (K, V)>`
/// - `HashSet<T>` with `IntoIterator<Item = T>`
/// - `BTreeMap<K, V>` with `IntoIterator<Item = (K, V)>`
/// - `BTreeSet<T>` with `IntoIterator<Item = T>`
/// - `PathBuf` with `IntoIterator<Item = Path>`
///
/// # Examples
///
/// ```
/// # use mergeme::Merge;
/// #
/// let a = vec![1, 2];
/// let b = &[3, 4, 5];
///
/// assert_eq!(a.merge(b), [1, 2, 3, 4, 5]);
/// ```
///
/// ```
/// # use mergeme::Merge;
/// #
/// let c = String::from("Hello, ");
/// let d = ['w', 'o', 'r', 'l', 'd', '!'];
///
/// assert_eq!(c.merge(d), "Hello, world!");
/// ```
///
/// ```
/// # use mergeme::Merge;
/// # use std::collections::HashMap;
/// #
/// let mut e = HashMap::from([
///     ("Hello", "Bonjour"),
///     ("Goodbye", "Au revoir"),
/// ]);
///
/// let f = [
///     ("Apple", "Pomme"),
///     ("Potato", "Pomme de terre"),
/// ];
///
/// assert_eq!(
///     e.merge(f),
///     HashMap::from([
///         ("Hello", "Bonjour"),
///         ("Goodbye", "Au revoir"),
///         ("Apple", "Pomme"),
///         ("Potato", "Pomme de terre"),
///     ]),
/// );
/// ```
///
/// ```
/// # use mergeme::Merge;
/// #
/// let g = vec![2, 4, 8];
/// let h = std::iter::successors(Some(16), |n| Some(n * 2)).take(3);
///
/// assert_eq!(g.merge(h), [2, 4, 8, 16, 32, 64]);
/// ```
impl<Base, Partial, Item> Merge<Partial> for Base
where
    Base: Extend<Item>,
    Partial: IntoIterator<Item = Item>,
{
    fn merge_in_place(&mut self, other: Partial) {
        self.extend(other);
    }
}
