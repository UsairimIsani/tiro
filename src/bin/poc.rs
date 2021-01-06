use tiro::prelude::*;

#[tokio::main]
async fn main() {
    let task = Task::new(&|| println!("{}", "Yes It Works "));
    let task = task.execute();
    let _ = task.execute();
}
