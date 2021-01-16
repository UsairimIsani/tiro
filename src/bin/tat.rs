use futures::join;
use tokio::sync::broadcast::{channel, Receiver, Sender};

#[tokio::main]
async fn main() {
    let mut task1 = create_task(Vec::new(), |x: i32| x + 1);

    let task2 = create_task(Vec::new(), |x: u8| {
        let i = x + 2;
        i
    });
    // let a = join!(task1, task2);
    task1.chain::<i32, u8>(task2).await;
}

fn create_task<T, I, O>(deps: Vec<Receiver<I>>, func: T) -> Task<O>
where
    I: Clone + Send + 'static,
    O: Clone + Send + 'static,
    T: Clone + Send + Sync + 'static + Fn(I) -> O,
{
    let func = func.clone();
    let (tx, mut rx) = channel::<O>(32);
    let a = async {
        tokio::spawn(async move {
            for mut r in deps {
                match r.recv().await {
                    Ok(v) => {
                        func(v);
                    }
                    Err(e) => println!("{}", e.to_string()),
                }
            }
        })
        .await
        .unwrap();
    };
    Task {
        channel: (tx.clone(), tx.subscribe()),
        deps: Vec::new(),
    }
}
pub struct Task<T> {
    channel: (Sender<T>, Receiver<T>),
    deps: Vec<Receiver<T>>,
}

impl<T> Task<T> {
    pub async fn chain<I, O>(&mut self, task: Task<O>) -> Task<O>
    where
        // F: Fn(I) -> O + Send + 'static,
        O: Clone + Send + 'static,
        I: Clone + Send + 'static,
    {
        let channel = channel::<O>(32);
        Task {
            channel,
            deps: Vec::new(),
        }
    }
}
