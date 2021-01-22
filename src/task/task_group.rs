use rayon::prelude::*;
// use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;

use arrow::datatypes::DataType;

pub struct TestTask<T> {
    data: T,
}

impl<T> TestTask<T>
where
    T: Send + 'static,
{
    pub fn new(data: T) -> Self {
        Self { data }
    }
}
pub enum TestType {
    String(String),
    Number(i32),
}
mod test {
    use super::*;
    #[test]
    fn test_task_group() {
        let c = TestTask::new(Arc::new(Box::new(DataType::Int16::from(1))));
        // let d = TestTask::new(Arc::new(DataType::Dictionary::from(
        //     Box::new(DataType::Int16::from(6)),
        //     Box::new(DataType::List::from(vec![
        //         DataType::Utf8::from(String::new()),
        //         DataType::Utf8::from(String::new()),
        //     ])),
        // )));

        let d = TestTask::new(Arc::new(DataType::Utf8::from(String::from("hkk"))));
        let mut map = HashMap::new();
        map.insert("a", c);
        map.insert("b", d);

        assert_eq!(2, map.len());
    }
}
