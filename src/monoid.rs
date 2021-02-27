use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque},
    hash::{BuildHasher, Hash},
};

pub trait Monoid {
    fn mempty() -> Self;
    fn mappend(self, other: Self) -> Self;

    fn mconcat<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Self>,
        Self: Sized,
    {
        iter.into_iter().fold(Self::mempty(), Self::mappend)
    }
}

impl Monoid for String {
    fn mempty() -> Self { Self::default() }
    fn mappend(mut self, other: Self) -> Self {
        self.push_str(&other);
        self
    }
}

impl<T: Monoid> Monoid for Option<T> {
    fn mempty() -> Self { Self::default() }
    fn mappend(self, other: Self) -> Self {
        match (self, other) {
            (None, None) => None,
            (None, Some(x)) | (Some(x), None) => Some(x),
            (Some(x), Some(y)) => Some(x.mappend(y)),
        }
    }
}

impl<T> Monoid for Vec<T> {
    fn mempty() -> Self { Self::default() }
    fn mappend(mut self, other: Self) -> Self {
        self.extend(other);
        self
    }
}

impl<T> Monoid for VecDeque<T> {
    fn mempty() -> Self { Self::default() }
    fn mappend(mut self, other: Self) -> Self {
        self.extend(other);
        self
    }
}

impl<T> Monoid for LinkedList<T> {
    fn mempty() -> Self { Self::default() }
    fn mappend(mut self, other: Self) -> Self {
        self.extend(other);
        self
    }
}

impl<T: Eq + Hash, S: BuildHasher + Default> Monoid for HashSet<T, S> {
    fn mempty() -> Self { Self::default() }
    fn mappend(mut self, other: Self) -> Self {
        self.extend(other);
        self
    }
}

impl<T: Ord> Monoid for BTreeSet<T> {
    fn mempty() -> Self { Self::default() }
    fn mappend(mut self, other: Self) -> Self {
        self.extend(other);
        self
    }
}

impl<K: Eq + Hash, V, S: BuildHasher + Default> Monoid for HashMap<K, V, S> {
    fn mempty() -> Self { Self::default() }
    fn mappend(mut self, other: Self) -> Self {
        self.extend(other);
        self
    }
}

impl<K: Ord, V> Monoid for BTreeMap<K, V> {
    fn mempty() -> Self { Self::default() }
    fn mappend(mut self, other: Self) -> Self {
        self.extend(other);
        self
    }
}
