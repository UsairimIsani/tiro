use futures::{channel::mpsc::unbounded, executor::block_on, future::join_all, prelude::*};
use std::future::Future;

use tokio::task::JoinHandle;
pub struct Task<V> {
    val: Option<V>,
}
impl<V> Task<V> {
    pub fn new<F>(func: F) -> Self
    where
        V: Clone,
        F: Fn() -> V,
    {
        let val = Some(func());
        Task { val }
    }
    pub fn collect(self) {}

    pub fn then<T, R>(self, other: T) -> Task<R>
    where
        T: Fn(V) -> R,
    {
        let val = self.val.and_then(|v| Some(other(v)));
        Task { val }
    }

    // pub fn and<T, F, R>(self, other: T) -> And<V, F>
    // where
    //     T: Fn(V) -> R,
    // {
    //     And {
    //         val: self.val,
    //         funcs: Vec::new(),
    //     }
    // }
}

// pub struct And<V> {
//     val: Option<V>,
// }

// impl<V, F> And<V, F> {
//     pub fn and<T, R>(mut self, other: F) -> And<V, F>
//     where
//         F: Fn(V) -> R,
//     {
//         self.funcs.push(Box::new(other));
//         self
//     }
//     pub fn then<T, R>(self, other: T) -> Task<R>
//     where
//         T: Fn(V) -> R,
//     {
//         let val = self.val.and_then(|v| Some(other(v)));
//         // self.funcs.into_iter().fold((), |a, f| (a, f(val)));

//         Task { val }
//     }

//     pub fn execute(self) {}
// }
mod test {

    #[test]
    fn test_new_tasks() {
        use super::*;

        let a = Task::new(|| {
            println!("I am Task 1");
            2
        });

        a.then(|x: i32| {
            println!("I am Task 2 {}", x);
        });
        println!("Should Print Task 2 first and Task1 Later");

        assert!(true)
    }

    #[tokio::test]
    async fn test_single_task() {
        use super::*;
        let _ = Task::new(|| {
            println!("I am Single Task");
        });
    }

    #[tokio::test]
    async fn test_chained_tasks() -> anyhow::Result<()> {
        use super::*;
        let a = Task::new(|| {
            println!("I am Task 1");
            (1, 3)
        });

        let b = |x: (i32, u8)| {
            println!("I am Task 2 {:?}", x);
            (x, x.0 + 2)
        };

        let c = |y: ((i32, u8), i32)| {
            println!("I am Task 3  {:#?}", y);
            y
        };

        let d = a.then(b).then(c);
        println!("Task Done : {:#?}", d.val);

        Ok(())
    }

    #[tokio::test]
    async fn test_parallel_task() -> anyhow::Result<()> {
        use super::*;
        let a = Task::new(|| {
            println!("I am Task 1");
            2
        });

        let b = |x: i32| {
            println!("I am Task 2");
        };

        let c = |x: i32| {
            println!("I am Task 3");
        };

        a
            // .and(b).and(c)
            .then(|z: i32| println!("{}", z));

        println!("Should Print 1,3 and after 2 seconds 3");

        Ok(())
    }

    // #[tokio::test]
    // async fn test_parallel_and_chained() -> anyhow::Result<()> {
    //     use super::*;
    //     let a = Task::new(|| {
    //         println!("I am Task 1");
    //     });

    //     let b = Task::new(|| {
    //         println!("I am Task 2");
    //     });

    //     let c = Task::new(|| {
    //         println!("I am Task 3");
    //     });
    //     let d = Task::new(|| {
    //         println!("I am Task 4");
    //     });

    //     let e = Task::new(|| {
    //         println!("I am Task 5");
    //     });

    //     let f = Task::new(|| {
    //         println!("I am Task 6");
    //     });

    //     a.then(c).then(b).and(e).and(f).then(d).collect().await?;
    //     println!(
    //         r#"
    //          task1 -> task3 -> task2 -> task5 -> task4
    //                                   \ task6 /

    //                                   "#
    //     );

    //     Ok(())
    // }
}
