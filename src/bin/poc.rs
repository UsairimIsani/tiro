use tiro::prelude::*;

#[tokio::main]
async fn main() {
    // let task_graph = TaskGraph::new();

    // let task = Task::new(|| println!("{}", "Yes It Works "));
    // let task2 = Task::new(move || {
    //     let x = rand::random::<u16>();
    //     println!("{} {}", "Yes wet Works ", x);
    //     x
    // });

    // let task_graph = task_graph.register("task1".to_string(), task);

    // let t1 = tokio::task::spawn(async move { task.execute().await });
    // let t2 = tokio::task::spawn(async move { task2.execute().await });
    // let mut task2 = t2.await.unwrap();
    // println!("Hi {}", task2.get_response().unwrap());
    // let task = t1.await.unwrap();
}
