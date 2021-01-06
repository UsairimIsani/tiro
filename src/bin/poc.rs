use tiro::prelude::*;

#[tokio::main]
async fn main() {
    let task = Task::new(&|| println!("{}", "Yes It Works "));
    let t1 = tokio::task::spawn(async move { task.execute().await });
    let task2 = Task::new(&|| println!("{}", "Yes wet Works "));
    let t2 = tokio::task::spawn(async move { task2.execute().await });
    let task2 = t2.await.unwrap();
    println!("Hi");
    let task = t1.await.unwrap();
}
