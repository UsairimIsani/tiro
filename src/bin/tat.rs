use futures::channel::oneshot;
use futures::lock::BiLock;
use futures::prelude::*;
use futures::stream::futures_unordered::FuturesUnordered;
use indexmap::IndexMap;
use std::future::Future;
use std::marker::Unpin;
use std::pin::Pin;
use std::task::{Context, Poll};

use tokio::sync::{
    broadcast,
    broadcast::{channel, Receiver},
};
#[tokio::main]
async fn main() {
    let task1 = |x: i32| x + 1;

    let task2 = |x: u8| {
        let i = x + 2;
        i
    };
    let task3 = |x: (u8, i32)| {
        let i = x.0 as i32 + x.1;
        i
    };
    let task4 = |x: String| println!("{}", x);
}

fn create_task<T, I, O>(deps: Vec<Receiver<I>>, func: T) -> Task<O>
where
    I: Clone + Send + 'static,
    O: Clone + Send + 'static,
    T: Clone + Send + Sync + 'static + Fn(I) -> O,
{
    let func = func.clone();
    let (tx, rx) = channel::<O>(32);
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
    channel: (broadcast::Sender<T>, Receiver<T>),
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
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};
use std::mem;
use std::vec::IntoIter;

pub struct Graph<N, I, S = RandomState> {
    map: IndexMap<I, (N, Vec<I>), S>,
}

impl<N, I, S> Default for Graph<N, I, S>
where
    I: Default + Hash + PartialEq + Eq + Clone,
    S: Default + BuildHasher,
{
    fn default() -> Graph<N, I, S> {
        Graph {
            map: IndexMap::default(),
        }
    }
}

impl<N, I, S> Graph<N, I, S>
where
    I: Hash + PartialEq + Eq + Clone,
    S: BuildHasher,
{
    pub fn add_node(&mut self, key: I, node: N) -> anyhow::Result<I> {
        self.map.insert(key.clone(), (node, Vec::new()));
        Ok(key)
    }

    pub fn contains(&self, index: &I) -> bool {
        self.map.contains_key(index)
    }

    pub fn add_edge(&mut self, parent: &I, next: I) -> Option<()> {
        self.map.get_mut(parent).map(|(_, sum)| sum.push(next))
    }

    pub fn remove_node(&mut self, index: &I) -> Option<N> {
        self.map.remove(index).map(|(n, _)| n)
    }

    pub fn get_node_mut(&mut self, index: &I) -> Option<&mut N> {
        self.map.get_mut(index).map(|(n, _)| n)
    }

    pub fn walk(&mut self, index: &I) -> IntoIter<I> {
        self.map
            .get_mut(index)
            .map(|(_, arr)| mem::replace(arr, Vec::new()))
            .unwrap_or_else(Vec::new)
            .into_iter()
    }
}

pub struct TaskGraph<T, I, S = RandomState> {
    dag: Graph<State<T>, I, S>,
    pending: Vec<IndexFuture<T, I>>,
}
enum State<T> {
    Pending { count: usize, task: T },
    Running,
}
impl<T, I, S> TaskGraph<T, I, S>
where
    T: Future + Unpin,
    I: Hash + PartialEq + Eq + PartialOrd + Clone + Unpin,
    S: BuildHasher + Unpin,
{
    pub fn add_task(&mut self, deps: &[I], key: I, task: T) -> anyhow::Result<I> {
        let mut count = 0;
        for dep in deps {
            if self.dag.contains(dep) {
                count += 1;
            }
        }

        if count == 0 {
            let i = self.dag.add_node(key, State::Running)?;
            self.pending.push(IndexFuture::new(i.clone(), task));
            Ok(i)
        } else {
            let i = self.dag.add_node(key, State::Pending { count, task })?;
            for parent in deps {
                self.dag.add_edge(parent, i.clone());
            }
            Ok(i)
        }
    }

    pub fn execute(mut self) -> (Sender<T, I, S>, Execute<T, I, S>) {
        let queue = FuturesUnordered::new();
        for fut in self.pending.drain(..) {
            queue.push(fut);
        }
        let (g1, g2) = BiLock::new(self);
        let (tx, rx) = oneshot::channel();
        (
            Sender { inner: g1, tx },
            Execute {
                inner: g2,
                done: Vec::new(),
                is_canceled: false,
                queue,
                rx,
            },
        )
    }

    fn walk(&mut self, index: &I) -> TaskWalker<'_, T, I, S> {
        let walker = self.dag.walk(index);
        TaskWalker {
            dag: &mut self.dag,
            walker,
        }
    }
}
pub struct Sender<T, I, S = RandomState> {
    inner: BiLock<TaskGraph<T, I, S>>,
    tx: oneshot::Sender<()>,
}

impl<T, I, S> Sender<T, I, S>
where
    T: Future + Unpin,
    I: Hash + PartialEq + Eq + PartialOrd + Clone + Unpin,
    S: BuildHasher + Unpin,
{
    #[inline]
    pub fn add_task<'a>(
        &'a self,
        deps: &'a [I],
        key: I,
        task: T,
    ) -> impl Future<Output = anyhow::Result<I>> + 'a {
        async move {
            let mut graph = self.inner.lock().await;
            graph.add_task(deps, key, task)
        }
    }

    pub fn abort(self) {
        let _ = self.tx.send(());
    }
}

pub struct Execute<T, I, S = RandomState> {
    inner: BiLock<TaskGraph<T, I, S>>,
    queue: FuturesUnordered<IndexFuture<T, I>>,
    done: Vec<I>,
    rx: oneshot::Receiver<()>,
    is_canceled: bool,
}

impl<T, I, S> Execute<T, I, S>
where
    T: Future + Unpin,
    I: Hash + PartialEq + Eq + PartialOrd + Clone + Unpin,
    S: BuildHasher + Unpin,
{
    fn enqueue(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        let mut graph = match self.inner.poll_lock(cx) {
            Poll::Ready(graph) => graph,
            Poll::Pending => return Poll::Pending,
        };

        for fut in graph.pending.drain(..) {
            self.queue.push(fut);
        }

        for index in self.done.drain(..) {
            for fut in graph.walk(&index) {
                self.queue.push(fut);
            }
            graph.dag.remove_node(&index);
        }

        Poll::Ready(())
    }
}

impl<F, I, S> Stream for Execute<F, I, S>
where
    F: Future + Unpin,
    I: Hash + PartialEq + Eq + PartialOrd + Clone + Unpin,
    S: BuildHasher + Unpin,
{
    type Item = (I, F::Output);

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.rx).poll(cx) {
            Poll::Pending => (),
            Poll::Ready(Ok(())) => return Poll::Ready(None),
            Poll::Ready(Err(_)) => {
                self.is_canceled = true;
            }
        }

        if let Poll::Pending = self.enqueue(cx) {
            return Poll::Pending;
        }

        match Pin::new(&mut self.queue).poll_next(cx) {
            Poll::Ready(Some((i, item))) => {
                self.done.push(i.clone());
                Poll::Ready(Some((i, item)))
            }
            Poll::Ready(None) if self.is_canceled => Poll::Ready(None),
            Poll::Ready(None) | Poll::Pending => Poll::Pending,
        }
    }
}

struct IndexFuture<F, I> {
    index: I,
    fut: F,
}

impl<F, I> IndexFuture<F, I> {
    pub fn new(index: I, fut: F) -> IndexFuture<F, I> {
        IndexFuture { index, fut }
    }
}

impl<F, I> Future for IndexFuture<F, I>
where
    F: Future + Unpin,
    I: Clone + Unpin,
{
    type Output = (I, F::Output);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let IndexFuture { index, fut } = self.get_mut();

        match Pin::new(fut).poll(cx) {
            Poll::Ready(item) => Poll::Ready((index.clone(), item)),
            Poll::Pending => Poll::Pending,
        }
    }
}

struct TaskWalker<'a, T, I, S> {
    dag: &'a mut Graph<State<T>, I, S>,
    walker: IntoIter<I>,
}

impl<'a, T, I, S> Iterator for TaskWalker<'a, T, I, S>
where
    I: Hash + PartialEq + Eq + Clone,
    S: BuildHasher + Unpin,
{
    type Item = IndexFuture<T, I>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(index) = self.walker.next() {
            let state = match self.dag.get_node_mut(&index) {
                Some(node) => node,
                None => continue,
            };

            if let State::Pending { count, .. } = state {
                *count -= 1;
            }

            match state {
                State::Pending { count: 0, .. } => (),
                _ => continue,
            }

            if let State::Pending { task, .. } = mem::replace(state, State::Running) {
                return Some(IndexFuture::new(index, task));
            }
        }

        None
    }
}
