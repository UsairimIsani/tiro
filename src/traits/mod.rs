pub trait Execute {
    type Item;
    fn execute(self) -> Self::Item;
}
pub trait Register<T>
where
    T: Execute,
{
    fn register(self, name: &str, task: T);
}
