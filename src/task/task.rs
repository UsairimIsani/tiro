// use std::marker::PhantomData;

// use crate::prelude::*;
// use async_trait::async_trait;
// use evmap::ShallowCopy;
// use futures::Future;
// // #[derive(Debug, PartialEq, Eq, Hash, Clone)]
// // pub struct Task<F, I, O> {
// //     func: F,
// //     res: Option<O>,
// //     inp: Option<I>,
// //     // _phantom: PhantomData<I>,
// // }
// // unsafe impl<F, I, O> Send for Task<F, I, O> {} // Still Unsure if this is right

// // impl<F, I, O> Task<F, I, O>
// // where
// //     F: Future + Send + 'static,
// //     O: Send + 'static,
// //     I: Send + 'static,
// // {
// //     pub fn new(func: F) -> Self {
// //         Task {
// //             func,
// //             res: None,
// //             inp: None,
// //         }
// //     }
// //     pub fn get_response(&mut self) -> Option<O> {
// //         self.res.take()
// //     }
// //     fn chain<CR, T>(self, func: F) -> Task<CR>
// //     where
// //         T: Future<Output = CR> + Send + 'static,
// //         // F: FnOnce(R) -> CR + Send + 'static,
// //         CR: Send + 'static,
// //         R: Send + 'static,
// //     {
// //         // let val = tokio::spawn(async { func(self.val) }).await.unwrap();
// //         Chainable { val }
// //     }
// // }

// // #[async_trait]
// // impl<F, I, O> Execute for Task<F, I, O>
// // where
// //     F: Fn(Option<I>) -> O,
// // {
// //     type Item = Self;
// //     async fn execute(mut self) -> Self::Item {
// //         self.res = Some((self.func)(self.inp.take()));
// //         self
// //     }
// // }

// // pub struct Chainable<R> {
// //     val: R,
// // }

// // impl<R> Chainable<R> {
// //     pub async fn chain<CR, F>(self, func: F) -> Chainable<CR>
// //     where
// //         // F: Future<Output = CR> + Send + 'static,
// //         F: FnOnce(R) -> CR + Send + 'static,
// //         CR: Send + 'static,
// //         R: Send + 'static,
// //     {
// //         let val = tokio::spawn(async { func(self.val) }).await.unwrap();
// //         Chainable { val }
// //     }

// //     pub fn end(self) -> R {
// //         self.val
// //     }
// // }

// // impl<R> std::ops::Deref for Chainable<R> {
// //     type Target = R;

// //     fn deref(&self) -> &R {
// //         &self.val
// //     }
// // }
// // mod tests {
// //     use super::Chainable;

// //     #[tokio::test]
// //     async fn test_chain() {
// //         let a: Chainable<i32> = Chainable { val: 1 };
// //         let b: Chainable<i32> = a.chain(|s| s + 12).await;
// //         let b = b.chain(|x| x + 2).await;

// //         assert_eq!(15, b.val);
// //     }
// // }

// pub struct Task<F, I, O> {
//     f: F,
//     inp: I,
//     out: O,
// }

// impl<F, I, O> Task<F, I, O> {
//     pub fn chain(self,t: F) {

//         Task{

//         }
//     }
//     pub fn parallel() {}
// }

mod tests {

    #[tokio::test]
    async fn test_task_chain() {
        let task1 = Task::new(async { 2 });
        let task2 = Task::new(async {
            let r = task1.get_result().await;
            r + 3
        });

        let task3 = task1.chain(task2);
    }
}
