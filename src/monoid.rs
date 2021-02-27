use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque},
    hash::{BuildHasher, Hash},
};

/// A Monoid is type with an associative binary operator, `combine`, and an
/// identity element, `unit`.
/// Implementations must obey the following *laws*:
/// - **Left identity**:  forall x. `T::unit().combine(x) == T::unit()`
/// - **Right identity**: forall x. `x.combine(T::unit()) == T::unit()`
/// - **Associativity**:  forall x, y, z. `x.combine(y.combine(z)) ==
///   (x.combine(y)).combine(z)`
pub trait Monoid {
    #[doc(alias = "mempty")]
    /// The identity element
    fn unit() -> Self;

    #[doc(alias = "<>")]
    #[doc(alias = "mappend")]
    /// The associative binary operator
    fn combine(self, other: Self) -> Self;

    #[doc(alias = "mconcat")]
    /// Combine all the elements of an iterator by repeated application of
    /// combine: `concat([e₁, e₂, ..., eₙ]) == e₁.combine(e₂) ... .combine(eₙ)`
    fn concat<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Self>,
        Self: Sized,
    {
        iter.into_iter().fold(Self::unit(), Self::combine)
    }
}

/// Combine `String`s by appending their contents.
impl Monoid for String {
    fn unit() -> Self { Self::default() }
    fn combine(mut self, other: Self) -> Self {
        self.push_str(&other);
        self
    }
}

/// Combine monoids wrapped in `Option` by treating each `Option` as a container
/// containing either 0 or 1 elements.
impl<T: Monoid> Monoid for Option<T> {
    fn unit() -> Self { Self::default() }
    fn combine(self, other: Self) -> Self {
        match (self, other) {
            (None, None) => None,
            (None, Some(x)) | (Some(x), None) => Some(x),
            (Some(x), Some(y)) => Some(x.combine(y)),
        }
    }
}

/// Combine `Vec`s by appending thier contents
impl<T> Monoid for Vec<T> {
    fn unit() -> Self { Self::default() }
    fn combine(mut self, other: Self) -> Self {
        self.extend(other);
        self
    }
}

/// Combine `VecDeque`s by appending thier contents
impl<T> Monoid for VecDeque<T> {
    fn unit() -> Self { Self::default() }
    fn combine(mut self, other: Self) -> Self {
        self.extend(other);
        self
    }
}

/// Combine `LinkedList`s by appending thier contents
impl<T> Monoid for LinkedList<T> {
    fn unit() -> Self { Self::default() }
    fn combine(mut self, other: Self) -> Self {
        self.extend(other);
        self
    }
}

/// Combine `HashSet`s by taking the union of the two sets (second set takes
/// priority)
impl<T: Eq + Hash, S: BuildHasher + Default> Monoid for HashSet<T, S> {
    fn unit() -> Self { Self::default() }
    fn combine(mut self, other: Self) -> Self {
        self.extend(other);
        self
    }
}

/// Combine `BTreeSet`s by taking the union of the two sets (second set takes
/// priority)
impl<T: Ord> Monoid for BTreeSet<T> {
    fn unit() -> Self { Self::default() }
    fn combine(mut self, other: Self) -> Self {
        self.extend(other);
        self
    }
}

/// Combine `HashMap`s by taking the union of the two maps (second map takes
/// priority)
impl<K: Eq + Hash, V, S: BuildHasher + Default> Monoid for HashMap<K, V, S> {
    fn unit() -> Self { Self::default() }
    fn combine(mut self, other: Self) -> Self {
        self.extend(other);
        self
    }
}

/// Combine `BTreeMap`s by taking the union of the two maps (second map takes
/// priority)
impl<K: Ord, V> Monoid for BTreeMap<K, V> {
    fn unit() -> Self { Self::default() }
    fn combine(mut self, other: Self) -> Self {
        self.extend(other);
        self
    }
}
