// use evmap::{new, ReadHandle, WriteHandle};
// use std::hash;
// use std::{collections::HashMap, future::Future};
// use tokio::task::JoinHandle;
// pub struct TaskGraph<T: Send + hash::Hash + Sync + Eq> {
//     nodes: HashMap<String, T>,
//     edges: Vec<Vec<Edge>>,
// }

// pub struct Edge {
//     weight: usize,
//     node: String,
// }
// // pub struct Task<T: Future + Send + Sync + 'static> {
// //     res: tokio::task::JoinHandle<()>,
// // }
// type BoxFuture<'a, T> = Box<dyn std::future::Future<Output = T> + Send + 'a>;
// pub struct Task<'a, T: Send> {
//     res: JoinHandle<BoxFuture<'a, T>>,
// }
// impl<'a, T: Send> Task<'a, T> {
//     pub fn new<J>(func: J) -> Self
//     where
//         J: Future + Send + 'static,
//         J::Output: Future + Send + 'static,
//     {
//         let future: JoinHandle<BoxFuture<<J as Future>>> = tokio::spawn(func);
//         Task { res: future }
//     }
// // }
// // struct Request<'r>(&'r ());

// // type BoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

// // trait FromRequestAsync<'a, 'r>: Sized {
// //     fn from_request<'fut>(req: &'a Request<'r>) -> BoxFuture<'fut, Option<Self>>
// //     where
// //         'a: 'fut;
// // }

// // impl<'a, 'r, T: FromRequestAsync<'a, 'r>> FromRequestAsync<'a, 'r> for Option<T> {
// //     fn from_request<'fut>(req: &'a Request<'r>) -> BoxFuture<'fut, Option<Self>>
// //     where
// //         'a: 'fut,
// //     {
// //         Box::pin(async move { Some(T::from_request(req).await) })
// //     }
// // }
use std::collections::HashMap;
pub trait Execute {
    fn execute(&self);
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
    fn execute(&self) {
        // task.execute();
    }
}

pub struct Task {}

impl Execute for Task {
    fn execute(&self) {}
}

#[tokio::main]
async fn main() {}
