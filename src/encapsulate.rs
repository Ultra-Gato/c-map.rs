use super::*;

use parking_lot::{RwLockReadGuard, RwLockWriteGuard};
use std::collections::hash_map::Entry;

macro_rules! readable {
    () => {
        /// Returns a reference to the value corresponding to the key.
        pub fn get(&self) -> Option<&V> {
            self.map.get(&self.key)
        }
        /// Returns the key-value pair corresponding to the supplied key.
        pub fn get_key_value(&self) -> Option<(&K, &V)> {
            self.map.get_key_value(&self.key)
        }
        /// Returns `true` if the map contains a value for the specified key.
        pub fn contains_key(&self) -> bool {
            self.map.contains_key(&self.key)
        }
    };
}


/// RAII structure used to release the shared read access, when dropped.
#[derive(Debug)]
pub struct Readable<'a, K, V, S> {
    pub(super) key: K,
    pub(super) map: RwLockReadGuard<'a, Map<K, V, S>>,
}

impl<K: Eq + Hash, V, S: BuildHasher> Readable<'_, K, V, S> {
    readable!();
}

/// RAII structure used to release the exclusive write access, when dropped.
#[derive(Debug)]
pub struct Writeable<'a, K, V, S> {
    pub(super) key: K,
    pub(super) map: RwLockWriteGuard<'a, Map<K, V, S>>,
}

impl<K: Eq + Hash, V, S: BuildHasher> Writeable<'_, K, V, S> {
    readable!();

    /// Returns a mutable reference to the value corresponding to the key.
    pub fn get_mut(&mut self) -> Option<&mut V> {
        self.map.get_mut(&self.key)
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, [`None`] is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned. The key is not updated, though; this matters for
    /// types that can be `==` without being identical.
    pub fn insert(&mut self, value: V) -> Option<V>
    where
        K: Clone,
    {
        self.map.insert(self.key.clone(), value)
    }

    /// Removes a key from the map, returning the value at the key if the key
    /// was previously in the map.
    pub fn remove(&mut self) -> Option<V> {
        self.map.remove(&self.key)
    }

    /// Gets the given key's corresponding entry in the map for in-place manipulation.
    pub fn entry(&mut self) -> Entry<'_, K, V>
    where
        K: Clone,
    {
        self.map.entry(self.key.clone())
    }

    /// Removes a key from the map, returning the stored key and value if the
    /// key was previously in the map.
    pub fn remove_entry(&mut self) -> Option<(K, V)> {
        self.map.remove_entry(&self.key)
    }
}
