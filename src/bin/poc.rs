use std::collections::HashMap;
pub trait Execute {
    type Item;
    fn execute(self) -> Self::Item;
}
pub trait Register<T>
where
    T: Execute,
{
    fn register(&self, name: &str, task: T);
}
pub struct TaskGraph<T: Execute> {
    tasks: HashMap<String, T>,
}

impl<T: Execute> Register<T> for TaskGraph<T> {
    fn register(&self, name: &str, task: T) {
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

pub struct Task
// where
// T: FnOnce(),
{
    func: Box<dyn Fn()>,
}
impl Task {
    pub fn new(func: Box<dyn Fn()>) -> Self {
        Task { func }
    }
}

impl Execute for Task {
    type Item = Self;
    fn execute(self) -> Self::Item {
        (self.func)();
        self
    }
}

#[tokio::main]
async fn main() {
    let task = Task::new(Box::new(&|| println!("{}", "Hell Yeah")));
    let task = task.execute();
    let task = task.execute();
}
