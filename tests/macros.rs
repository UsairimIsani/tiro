mod tests {
    #[test]
    fn test_macros() {
        use tiro::prelude::*;

        use tiro::create_scheme;
        create_scheme!("hello" => ["world"]);
    }
}
