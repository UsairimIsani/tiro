// use crate::prelude::*;

// use async_trait::async_trait;
// // // use evmap::{ReadHandle, WriteHandle};
// use std::collections::HashMap;
// use std::hash::Hash;

// // unsafe impl<T, K> Send for TaskGraph<K, T> where K: Hash + PartialEq + Eq {}

// #[derive(Debug)]
// pub struct TaskGraph<K, T>
// where
// //     K: Hash + PartialEq + Eq,
// //     // T: Execute,
// {
//     tasks: HashMap<K, T>,
//     schemes: Vec<Scheme<K>>,
//     //     // queue:
// }

// impl<T, K> TaskGraph<K, T>
// where
//     K: Hash + PartialEq + Eq,
//     // T: Execute,
// {
//     pub fn new() -> Self {
//         // let map = evmap::new();
//         Self {
//             // tasks: map,
//             tasks: HashMap::new(),
//             schemes: Vec::new(),
//         }
//     }
//     pub fn add_scheme(mut self, scheme: Scheme<K>) -> Self {
//         self.schemes.push(scheme);
//         self
//     }
// }

// impl<T, K> Register<T, K> for TaskGraph<K, T>
// where
//     K: Hash + PartialEq + Eq,
//     // T: Execute,
// {
//     type Item = Self;
//     fn register(mut self, name: K, task: T) -> Self {
//         self.tasks.insert(name, task);
//         self
//     }
// }

// #[async_trait]
// impl<T, K> Execute for TaskGraph<K, T>
// where
//     K: Hash + PartialEq + Eq,
//     // T: TaskExt,
// {
//     type Item = Self;
//     async fn execute(self) -> Self::Item {
//         // task.execute();
//         self
//     }
// }

// mod tests {
//     use super::*;
//     #[test]
//     fn test_new_taskgraph() {
//         use super::*;
//         let task_graph: TaskGraph<&str, Task<()>> = TaskGraph::new();
//     }
// }

// // initialize task
// // put in the already declared taskgraph
// // pushing that into
