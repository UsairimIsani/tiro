use crate::prelude::Execute;
use async_trait::async_trait;
pub struct Task<T>
where
    T: Fn(),
{
    func: Box<T>,
}
unsafe impl<T: Fn()> Send for Task<T> {}

impl<T: Fn()> Task<T> {
    pub fn new(func: T) -> Self {
        let func = Box::new(func);
        Task { func }
    }
}

#[async_trait]
impl<T: Fn()> Execute for Task<T> {
    type Item = Self;
    async fn execute(self) -> Self::Item {
        // let random = rand::random::<u8>() as u64;
        // println!("Random Sleep : {}", random);
        // std::thread::sleep(std::time::Duration::from_millis(random));
        (self.func)();
        self
    }
}
