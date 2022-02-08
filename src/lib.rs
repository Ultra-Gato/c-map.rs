#![doc = include_str!("../README.md")]

mod encapsulate;

use parking_lot::RwLock;
use std::{
    collections::{hash_map::RandomState, HashMap as Map},
    hash::{BuildHasher, Hash, Hasher},
};

pub use encapsulate::*;

pub struct HashMap<K, V, S = RandomState> {
    hasher: S,
    shift: usize,
    shards: Box<[RwLock<Map<K, V, S>>]>,
}

impl<K, V> HashMap<K, V> {
    /// Creates an empty `HashMap`.
    ///
    /// The hash map is initially created with a capacity of 0, so it will not allocate until it
    /// is first inserted into.
    #[inline]
    pub fn new() -> Self {
        Self::with_hasher(RandomState::default())
    }

    /// Creates an empty `HashMap` with the specified capacity.
    ///
    /// The hash map will be able to hold at least `capacity` elements without
    /// reallocating. If `capacity` is 0, the hash map will not allocate.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, RandomState::default())
    }
}

impl<K: Hash, V, S: BuildHasher> HashMap<K, V, S> {
    #[inline]
    pub fn read(&self, key: K) -> Readable<K, V, S> {
        Readable {
            map: self.shard(&key).read(),
            key,
        }
    }

    #[inline]
    pub fn write(&self, key: K) -> Writeable<K, V, S> {
        Writeable {
            map: self.shard(&key).write(),
            key,
        }
    }

    #[inline]
    pub fn shard(&self, key: &K) -> &RwLock<Map<K, V, S>> {
        let idx = self.shard_idx(key);
        unsafe { self.shards.get_unchecked(idx) }
    }

    pub fn shard_idx(&self, key: &K) -> usize {
        let mut hasher = self.hasher.build_hasher();
        key.hash(&mut hasher);
        let hash = hasher.finish() as usize;
        // Leave the high 7 bits for the HashBrown SIMD tag.
        (hash << 7) >> self.shift
    }
}

impl<K, V, S: Clone> HashMap<K, V, S> {
    #[inline]
    /// Get a reference to the map's shards.
    pub fn shards(&self) -> &[RwLock<Map<K, V, S>>] {
        self.shards.as_ref()
    }

    /// Creates an empty `HashMap` which will use the given hash builder to hash
    /// keys.
    ///
    /// The created map has the default initial capacity.
    ///
    /// Warning: `hash_builder` is normally randomly generated, and
    /// is designed to allow HashMaps to be resistant to attacks that
    /// cause many collisions and very poor performance. Setting it
    /// manually using this function can expose a DoS attack vector.
    ///
    /// The `hash_builder` passed should implement the [`BuildHasher`] trait for
    /// the HashMap to be useful.
    #[inline]
    pub fn with_hasher(hasher: S) -> Self {
        Self::with_capacity_and_hasher(0, hasher)
    }

    /// Creates an empty `HashMap` with the specified capacity, using `hash_builder`
    /// to hash the keys.
    ///
    /// The hash map will be able to hold at least `capacity` elements without
    /// reallocating. If `capacity` is 0, the hash map will not allocate.
    ///
    /// Warning: `hash_builder` is normally randomly generated, and
    /// is designed to allow HashMaps to be resistant to attacks that
    /// cause many collisions and very poor performance. Setting it
    /// manually using this function can expose a DoS attack vector.
    ///
    /// The `hash_builder` passed should implement the [`BuildHasher`] trait for
    /// the HashMap to be useful.
    pub fn with_capacity_and_hasher(mut capacity: usize, hasher: S) -> Self {
        let shard_amount = (num_cpus::get() * 4).next_power_of_two();
        if capacity != 0 {
            capacity = (capacity + (shard_amount - 1)) & !(shard_amount - 1);
        }
        Self {
            shift: std::mem::size_of::<usize>() * 8 - shard_amount.trailing_zeros() as usize,
            shards: (0..shard_amount)
                .map(|_| {
                    RwLock::new(Map::with_capacity_and_hasher(
                        capacity / shard_amount,
                        hasher.clone(),
                    ))
                })
                .collect(),
            hasher,
        }
    }
}

impl<K, V> Default for HashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
