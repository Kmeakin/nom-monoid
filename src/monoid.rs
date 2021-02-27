use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque},
    hash::{BuildHasher, Hash},
};

pub trait Monoid {
    fn unit() -> Self;

    fn combine(self, other: Self) -> Self;
    fn concat<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Self>,
        Self: Sized,
    {
        iter.into_iter().fold(Self::unit(), Self::combine)
    }
}

impl Monoid for String {
    fn unit() -> Self { Self::default() }
    fn combine(mut self, other: Self) -> Self {
        self.push_str(&other);
        self
    }
}

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

impl<T> Monoid for Vec<T> {
    fn unit() -> Self { Self::default() }
    fn combine(mut self, other: Self) -> Self {
        self.extend(other);
        self
    }
}

impl<T> Monoid for VecDeque<T> {
    fn unit() -> Self { Self::default() }
    fn combine(mut self, other: Self) -> Self {
        self.extend(other);
        self
    }
}

impl<T> Monoid for LinkedList<T> {
    fn unit() -> Self { Self::default() }
    fn combine(mut self, other: Self) -> Self {
        self.extend(other);
        self
    }
}

impl<T: Eq + Hash, S: BuildHasher + Default> Monoid for HashSet<T, S> {
    fn unit() -> Self { Self::default() }
    fn combine(mut self, other: Self) -> Self {
        self.extend(other);
        self
    }
}

impl<T: Ord> Monoid for BTreeSet<T> {
    fn unit() -> Self { Self::default() }
    fn combine(mut self, other: Self) -> Self {
        self.extend(other);
        self
    }
}

impl<K: Eq + Hash, V, S: BuildHasher + Default> Monoid for HashMap<K, V, S> {
    fn unit() -> Self { Self::default() }
    fn combine(mut self, other: Self) -> Self {
        self.extend(other);
        self
    }
}

impl<K: Ord, V> Monoid for BTreeMap<K, V> {
    fn unit() -> Self { Self::default() }
    fn combine(mut self, other: Self) -> Self {
        self.extend(other);
        self
    }
}
