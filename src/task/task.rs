use crate::prelude::Execute;
pub struct Task<T>
where
    T: Fn(),
{
    func: Box<T>,
}
impl<T: Fn()> Task<T> {
    pub fn new(func: T) -> Self {
        let func = Box::new(func);
        Task { func }
    }
}

impl<T: Fn()> Execute for Task<T> {
    type Item = Self;
    fn execute(self) -> Self::Item {
        (self.func)();
        self
    }
}
