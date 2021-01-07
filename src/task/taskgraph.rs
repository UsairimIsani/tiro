use crate::prelude::*;
use async_trait::async_trait;
use evmap::{ReadHandle, WriteHandle};
use std::collections::HashMap;
use std::hash::Hash;

unsafe impl<T, K> Send for TaskGraph<K, T> where K: Hash + PartialEq + Eq // T: Execute,,,
{
}
pub struct TaskGraph<K, T>
where
    K: Hash + PartialEq + Eq,
    // T: Execute,
{
    // tasks: (ReadHandle<K, T>, WriteHandle<K, T>),
    // For the time being
    tasks: HashMap<K, Box<T>>,
    schemes: Vec<Scheme<K>>,
}

impl<T, K> TaskGraph<K, T>
where
    K: Hash + PartialEq + Eq,
    // T: Execute,
{
    pub fn new() -> Self {
        // let map = evmap::new();
        Self {
            // tasks: map,
            tasks: HashMap::new(),
            schemes: Vec::new(),
        }
    }
    pub fn add_scheme(mut self, scheme: Scheme<K>) -> Self {
        self.schemes.push(scheme);
        self
    }
}

impl<T, K> Register<T, K> for TaskGraph<K, T>
where
    K: Hash + PartialEq + Eq,
    // T: Execute,
{
    type Item = Self;
    fn register(mut self, name: K, task: Box<T>) -> Self {
        self.tasks.insert(name, task);
        self
    }
}

#[async_trait]
impl<T, K> Execute for TaskGraph<K, T>
where
    K: Hash + PartialEq + Eq,
    // T: TaskExt,
{
    type Item = Self;
    async fn execute(self) -> Self::Item {
        // task.execute();
        self
    }
}
