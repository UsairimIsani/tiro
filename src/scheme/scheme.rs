use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use super::Dependencies;

#[derive(Debug)]
pub struct Scheme<K>
where
    K: Hash + Eq,
{
    inner: HashMap<K, Dependencies<K>>,
}

impl<K> Scheme<K>
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn insert_dep(&mut self, key: K, deps: Dependencies<K>) {
        self.inner.insert(key, deps);
    }

    pub fn get_dep(&self, key: &K) -> Option<&Dependencies<K>> {
        self.inner.get(key)
    }

    // pub fn set_first(&mut self, key: K) {
    //     self.first = Some(key);
    // }
    // pub fn set_last(&mut self, key: K) {
    //     self.last = Some(key);
    // }
    // pub fn get_first(&self) -> &Option<K> {
    //     &self.first
    // }
    // pub fn get_last(&self) -> &Option<K> {
    //     &self.last
    // }
}
