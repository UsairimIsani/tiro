use crate::prelude::*;

#[macro_export]
macro_rules! create_scheme {
    ($($key: expr => [ $($val:expr),* ] ),*) => {{
        use crate::prelude::*;
        use std::collections::{HashSet,HashMap};
        let mut scheme: Scheme<&str> = Scheme::new();

        $(
            let mut temp = HashMap::new();
            let mut upstream: HashSet<&str> = HashSet::new();
            let mut downstream: HashSet<&str> = HashSet::new();
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
// map {
//  task1 => task2,
//  sdf => task2,
//  task2 => task3
//  task2 => task4,
//  task3 => task4
// }

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
