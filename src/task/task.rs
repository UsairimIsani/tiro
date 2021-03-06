use futures::{channel::mpsc::unbounded, executor::block_on, future::join_all, prelude::*};
use std::future::Future;

use tokio::task::JoinHandle;
pub struct Task {
    fut: JoinHandle<()>,
}
impl Task
// F::Output: Send + 'static,
{
    pub fn new<F>(fut: F) -> Self
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let handle = tokio::spawn(fut);
        Self { fut: handle }
    }
    pub async fn execute(self) -> anyhow::Result<()> {
        self.fut.await?;
        Ok(())
    }

    pub async fn chain(self, other: Task) -> anyhow::Result<Self> {
        self.fut
            .await
            .and_then(|_x| Ok(Task { fut: other.fut }))
            .map_err(|e| anyhow::Error::from(e))
    }

    pub fn and(self, other: Task) -> And {
        let queue = vec![self.fut, other.fut];
        And { queue }
    }
}

pub struct And {
    queue: Vec<JoinHandle<()>>,
}

impl And {
    pub async fn and(mut self, other: Task) -> And {
        self.queue.push(other.fut);
        self
    }
    pub async fn chain(self, other: Task) -> anyhow::Result<Task> {
        join_all(self.queue).await;

        Ok(Task { fut: other.fut })
    }

    pub async fn execute(self) -> anyhow::Result<()> {
        join_all(self.queue).await;
        Ok(())
    }
}
mod test {

    #[tokio::test]
    async fn test_new_tasks() {
        use super::*;
        use futures::future::join_all;
        use tokio::time::{sleep, Duration};

        let a = Task::new(async {
            let _ = sleep(Duration::from_secs(2)).await;
            println!("I am Task 1");
        });

        let b = Task::new(async {
            let _ = sleep(Duration::from_secs(1)).await;
            println!("I am Task 2");
        });
        let v = vec![a.execute(), b.execute()];
        // tokio::join!(a.execute(), b.execute());
        join_all(v).await;

        println!("Should Print Task 2 first and Task1 Later");

        assert!(true)
    }

    #[tokio::test]
    async fn test_single_task() {
        use super::*;
        let _ = Task::new(async {
            println!("I am Single Task");
        })
        .execute()
        .await;
    }

    #[tokio::test]
    async fn test_chained_tasks() -> anyhow::Result<()> {
        use super::*;
        let a = Task::new(async {
            println!("I am Task 1");
        });

        let b = Task::new(async {
            println!("I am Task 2");
        });

        let c = Task::new(async {
            println!("I am Task 3");
        });

        println!("Should Print Task 1 , 3, 2 Later");

        a.chain(c).await?.chain(b).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_parallel_task() -> anyhow::Result<()> {
        use super::*;
        use tokio::time::{sleep, Duration};
        let a = Task::new(async {
            let _ = sleep(Duration::from_secs(1)).await;
            println!("I am Task 1");
        });

        let b = Task::new(async {
            let _ = sleep(Duration::from_secs(2)).await;
            println!("I am Task 2");
        });

        let c = Task::new(async {
            let _ = sleep(Duration::from_secs(1)).await;
            println!("I am Task 3");
        });

        b.and(a).and(c).await.execute().await?;

        println!("Should Print 1,3 and after 2 seconds 3");

        Ok(())
    }

    // Not what i hoped to achieve
    #[tokio::test]
    async fn test_parallel_and_chained() -> anyhow::Result<()> {
        use super::*;
        use tokio::time::{sleep, Duration};
        let a = Task::new(async {
            // let _ = sleep(Duration::from_secs(1)).await;
            println!("I am Task 1");
        });

        let b = Task::new(async {
            // let _ = sleep(Duration::from_secs(3)).await;
            println!("I am Task 2");
        });

        let c = Task::new(async {
            // let _ = sleep(Duration::from_secs(2)).await;
            println!("I am Task 3");
        });
        let d = Task::new(async {
            // let _ = sleep(Duration::from_secs(5)).await;
            println!("I am Task 4");
        });

        let e = Task::new(async {
            // let _ = sleep(Duration::from_secs(4)).await;
            println!("I am Task 5");
        });

        let f = Task::new(async {
            // let _ = sleep(Duration::from_secs(6)).await;
            // block_on(a.fut).unwrap();
            println!("I am Task 6");
        });

        a.chain(c)
            .await?
            .chain(b)
            .await?
            .and(e)
            .and(f)
            .await
            .chain(d)
            .await?
            .execute()
            .await?;
        println!(
            r#"
             task1 -> task3 -> task2 -> task5 -> task4
                                      \ task6 /

                                      "#
        );
        Ok(())
    }
    // I would say no?
    // Would it be possible to add in data into a set of hashmaps? //
    // Like, one saying what's required to move to the next stage // Scheme Dependencies part downstream,
    // Another explaining a count for the number of prior tasks for a given stage//(task) Scheme Dependencies part upstream
    // The 3rd would be a string and generic and would hold data to send onto the next Fn// Problem Here is if The val is used then all the HashMap would be of the same type
    // Then you'd send the data in together
    // Likely in a local hashmap explaining where the data just came from// task scheme Dependency Struct
    // The channel itself would only recieve signal that a pathway is finished and will commission another parallel path to run if requirements are met
    // Which could allow for that?
    // No idea
    // I seriously haven't been doing Rust all too much
    // I know the details are rougher than imagination
}
