use async_trait::async_trait;
use evmap::ShallowCopy;
use std::hash::Hash;
#[async_trait]
pub trait Execute {
    type Item;
    async fn execute(self) -> Self::Item;
}
pub trait Register<T, K>
where
    T: TaskExt,
    K: TaskName,
{
    type Item;
    fn register(self, name: K, task: T) -> Self::Item;
}
pub trait TaskName: Hash + Eq + Clone + ShallowCopy {}
pub trait TaskExt: Execute + Eq + Clone + ShallowCopy + Hash {}
