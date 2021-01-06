mod task;
mod traits;
pub mod prelude {
    use super::*;
    pub use task::{Task, TaskGraph};
    pub use traits::{Execute, Register};
}
