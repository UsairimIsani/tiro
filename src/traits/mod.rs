use async_trait::async_trait;
#[async_trait]
pub trait Execute {
    type Item;
    async fn execute(self) -> Self::Item;
}
pub trait Register<T>
where
    T: Execute,
{
    fn register(self, name: &str, task: T);
}
