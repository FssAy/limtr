use std::collections::HashMap;
use std::hash::Hash;
use std::slice::SliceIndex;


#[derive(Debug, Default, Clone)]
pub(crate) struct IndexMap<K: Hash + Default + Eq + PartialEq, V: Default> {
    values: Vec<Option<V>>,
    keys: HashMap<K, usize>,
    nuked: Vec<usize>,
}

impl<K: Hash + Default + Eq + PartialEq, V: Default> IndexMap<K, V> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_capacity(capacity: usize) -> Self {
        Self {
            values: Vec::with_capacity(capacity),
            keys: HashMap::with_capacity(capacity),
            nuked: vec![],
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(index) = self.keys.remove(key) {
            let maybe_value = self.values.get_mut(index).unwrap();
            let value = std::mem::take(maybe_value).unwrap();
            self.nuked.push(index);
            Some(value)
        } else {
            None
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if let Some(old_index) = self.keys.get(&key) {
            let maybe_value = self.values.get_mut(*old_index).unwrap();
            let old_value = std::mem::replace(
                maybe_value,
                Some(value)
            ).unwrap();

            self.keys.insert(key, *old_index);

            return Some(old_value);
        }

        if self.nuked.is_empty() {
            let index = self.values.len();
            self.values.push(Some(value));
            self.keys.insert(key, index);
        } else {
            let index = self.nuked.pop().unwrap();
            let nuked_value = self.values.get_mut(index).unwrap();
            let _ = std::mem::replace(nuked_value, Some(value));
            self.keys.insert(key, index);
        }

        None
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let index = *self.keys.get(key)?;
        self.values.get(index).unwrap().as_ref()
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let index = *self.keys.get(key)?;
        self.values.get_mut(index).unwrap().as_mut()
    }
}
