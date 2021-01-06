use crate::prelude::{Execute, Register};
unsafe impl<T: Execute> Send for TaskGraph<T> {}
pub struct TaskGraph<T: Execute> {
    pub tasks: ReadHandle<String, T>, // For the time being
}

impl<T: Execute> Register<T> for TaskGraph<T> {
    fn register(self, _name: &str, _task: T) {
        unimplemented!()
    }
}
use async_trait::async_trait;
use evmap::ReadHandle;
#[async_trait]
impl<T: Execute> Execute for TaskGraph<T> {
    type Item = Self;
    async fn execute(self) -> Self::Item {
        // task.execute();
        self
    }
}
