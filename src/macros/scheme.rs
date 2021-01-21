// pub use macros::scheme::create_scheme;
use crate::prelude::*;

#[macro_export]
macro_rules! create_scheme {
    ($($key: expr => [ $($val:expr),* ] ),*) => {{
        use super::*;
        use std::collections::{HashSet,HashMap};
        let mut scheme: Scheme<&str> = Scheme::new();

        $(
            let mut temp = HashMap::new();
            let mut upstream: HashSet<&str> = HashSet::new();
            let downstream: HashSet<&str> = HashSet::new();
            $(
                upstream.insert($val);
                temp.insert($val, $key);

            )*

            let dep = Dependencies::new(upstream,downstream,$key);
            scheme.insert_dep($key, dep);

        )*
        scheme
    }


    };

}

mod tests {
    #[test]
    fn test_macros() {
        use crate::create_scheme;
        let scheme = create_scheme!(
            "task2" => ["task1","sdf"],
            "task3" => ["task2"],
            "task4" => [ "task2","task3" ]
        );
        println!("{:#?}", scheme);
    }
}
