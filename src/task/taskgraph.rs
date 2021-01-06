use crate::prelude::{Execute, Register};
use std::collections::HashMap; // Will Be swapped with evmap or something else ?
pub struct TaskGraph<T: Execute> {
    pub tasks: HashMap<String, T>, // For the time being
}

impl<T: Execute> Register<T> for TaskGraph<T> {
    fn register(self, _name: &str, _task: T) {
        unimplemented!()
    }
}
impl<T: Execute> Execute for TaskGraph<T> {
    type Item = Self;
    fn execute(self) -> Self::Item {
        // task.execute();
        self
    }
}
