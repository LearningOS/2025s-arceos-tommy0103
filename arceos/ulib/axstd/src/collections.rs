use core::borrow::Borrow;
use core::default::Default;
use core::hash::{Hash, Hasher, BuildHasherDefault};
use core::iter::Map;
use alloc::boxed::Box;
use alloc::vec::Vec;
pub struct FnvHasher(u64);

impl Default for FnvHasher {
    #[inline]
    fn default() -> FnvHasher {
        FnvHasher(0xcbf29ce484222325)
    }
}

impl FnvHasher {
    /// Create an FNV hasher starting with a state corresponding
    /// to the hash `key`.
    #[inline]
    pub fn with_key(key: u64) -> FnvHasher {
        FnvHasher(key)
    }
}

impl Hasher for FnvHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        let FnvHasher(mut hash) = *self;

        for byte in bytes.iter() {
            hash = hash ^ (*byte as u64);
            hash = hash.wrapping_mul(0x100000001b3);
        }

        *self = FnvHasher(hash);
    }
}

// const BUCKET_SIZE: usize = 
 
struct MapEntry<K, V> {
    key: K,
    value: V,
    next: Option<Box<MapEntry<K, V>>>,
}

pub struct HashMap<K, V> {
    buckets: Vec<Option<Box<MapEntry<K, V>>>>,
    capacity: usize,
}

impl<K: Hash, V> HashMap<K, V> {
    pub fn new() -> Self {
        Self::with_capacity(10000)
    }
    pub fn with_capacity(capacity: usize) -> Self {
        let mut buckets = Vec::with_capacity(capacity);
        buckets.resize_with(capacity, || None);
        HashMap { 
            buckets, 
            capacity
        } 
    }
    fn hash(&self, key: &K) -> u64 {
        let mut hasher = FnvHasher::default();
        key.hash(&mut hasher);
        hasher.finish()
    }
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let index = self.hash(&key) as usize % self.capacity ;
        let mut bucket = &mut self.buckets[index];
        match bucket {
            None => {
                *bucket = Some(Box::new(
                    MapEntry {
                        key,
                        value,
                        next: None,
                    }
                ));
                None
            },
            Some(_) => {
                let mut entry = bucket;
                loop {
                    let current = entry.as_mut().unwrap();
                    if current.next.is_none() {
                        current.next = Some(Box::new(
                            MapEntry {
                                key,
                                value,
                                next: None,
                            }
                        ));
                        return None;
                    }
                    entry = &mut current.next;
                }
                None
            }
        }
    }
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            buckets: &self.buckets,
            current_bucket: 0,
            current_entry: None,
        }
    }
}

pub struct Iter<'a ,K, V> {
    buckets: &'a [Option<Box<MapEntry<K, V>>>],
    current_bucket: usize,
    current_entry: Option<&'a MapEntry<K, V>>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(entry) = self.current_entry {
                let next_entry = entry.next.as_deref();
                self.current_entry = next_entry;
                return Some((&entry.key, &entry.value))
            }
            while self.current_bucket < self.buckets.len() {
                if let Some(bucket) = &self.buckets[self.current_bucket] {
                    self.current_entry = Some(bucket.as_ref());
                    self.current_bucket += 1;
                    break;
                }
                self.current_bucket += 1;
            }
            if self.current_bucket >= self.buckets.len() {
                break;
            }
        }
        None        
    }
}