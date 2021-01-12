use crate::prelude::*;
use async_trait::async_trait;
use evmap::ShallowCopy;
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Task<T, R>
where
    T: Fn() -> R,
{
    func: Box<T>,
    res: Option<R>,
}
unsafe impl<T: Fn() -> R, R> Send for Task<T, R> {} // Still Unsure if this is right

impl<T: Fn() -> R, R> Task<T, R> {
    pub fn new(func: Box<T>) -> Self {
        Task { func, res: None }
    }
    pub fn get_response(&mut self) -> Option<R> {
        self.res.take()
    }
}

#[async_trait]
impl<T: Fn() -> R, R> Execute for Task<T, R> {
    type Item = Self;
    async fn execute(mut self) -> Self::Item {
        self.res = Some((self.func)());
        self
    }
}
// impl<T, K> ShallowCopy for Task<T, K> where T: Fn() -> K {}
// impl<T, K> TaskExt for Task<T, K> where T: Fn() -> K {}
// use crate::prelude::Execute;
// use async_trait::async_trait;
// pub struct Task<T, I, R> // Need to Figure Out PhantomData?
// where
//     T: Fn(I) -> R,
// {
//     func: Box<T>,
//     res: Option<R>,
// }
// unsafe impl<T: Fn(I) -> R, I, R> Send for Task<T, I, R> {} // Still Unsure if this is right

// impl<T: Fn(I) -> R, I, R> Task<T, I, R> {
//     pub fn new(func: T) -> Self {
//         let func = Box::new(func);
//         Task { func, res: None }
//     }
//     pub fn get_response(&mut self) -> Option<R> {
//         self.res.take()
//     }
// }

// #[async_trait]
// impl<T: Fn(I) -> R, I, R> Execute for Task<T, I, R> {
//     type Item = Self;
//     async fn execute(mut self, param: I) -> Self::Item {
//         self.res = Some((self.func)(param));
//         self
//     }
// }
