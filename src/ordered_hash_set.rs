use std::{collections::HashSet, hash::{DefaultHasher, Hasher, Hash}};

pub struct OrderedHashSet<T: Hash + Eq> {
    values: Vec<T>,
    hash_values: HashSet<u64>,
}

impl<T: Hash + Eq> OrderedHashSet<T> {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            hash_values: HashSet::new()
        }
    }

    pub fn insert(&mut self, value: T) -> bool {
        let hash_value = {
            let mut hasher = DefaultHasher::new();
            value.hash(&mut hasher);
            hasher.finish()
        };

        if self.hash_values.insert(hash_value) {
            self.values.push(value);
            true
        } else {
            false
        }
    }
}

impl<T: Hash + Eq> FromIterator<T> for OrderedHashSet<T> {
    fn from_iter<Iterator: IntoIterator<Item = T>>(iterator: Iterator) -> Self {
        let mut result = Self::new();
        for item in iterator {
            result.insert(item);
        }
        result
    }
}

impl<T: Hash + Eq> Extend<T> for OrderedHashSet<T> {
    fn extend<Iterator: IntoIterator<Item = T>>(&mut self, iterator: Iterator) {
        for item in iterator {
            self.insert(item);
        }
    }
}

impl<T: Hash + Eq> IntoIterator for OrderedHashSet<T> {
    type Item = T;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }

}
impl<'a, T: Hash + Eq> IntoIterator for &'a OrderedHashSet<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.iter()
    }
}