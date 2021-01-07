use async_trait::async_trait;
use evmap::ShallowCopy;
use std::hash::Hash;
#[async_trait]
pub trait Execute {
    type Item;
    async fn execute(self) -> Self::Item;
}
pub trait Register<T, K> // T: TaskExt,
// K: TaskName,
{
    type Item;
    fn register(self, name: K, task: Box<T>) -> Self::Item;
}
pub trait TaskName: Hash + PartialEq + Eq {}
pub trait TaskExt: Execute {}
