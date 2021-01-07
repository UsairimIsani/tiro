use async_trait::async_trait;
use evmap::{ReadHandle, WriteHandle};

use crate::prelude::*;

unsafe impl<T, K> Send for TaskGraph<T, K>
where
    K: TaskName,
    T: TaskExt,
{
}
pub struct TaskGraph<T, K>
where
    K: TaskName,
    T: TaskExt,
{
    tasks: (ReadHandle<K, T>, WriteHandle<K, T>), // For the time being
    schemes: Vec<Scheme<K>>,
}

impl<T, K> TaskGraph<T, K>
where
    K: TaskName,
    T: TaskExt,
{
    pub fn new() -> Self {
        let map = evmap::new();
        Self {
            tasks: map,
            schemes: Vec::new(),
        }
    }
    pub fn add_scheme(mut self, scheme: Scheme<K>) -> Self {
        self.schemes.push(scheme);
        self
    }
}

impl<T, K> Register<T, K> for TaskGraph<T, K>
where
    K: TaskName,
    T: TaskExt,
{
    type Item = Self;
    fn register(mut self, name: K, task: T) -> Self {
        self.tasks.1.insert(name, task);
        self
    }
}

#[async_trait]
impl<T, K> Execute for TaskGraph<T, K>
where
    K: TaskName,
    T: TaskExt,
{
    type Item = Self;
    async fn execute(self) -> Self::Item {
        // task.execute();
        self
    }
}
