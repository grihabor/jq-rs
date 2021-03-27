mod parser;

fn main() {
    println!("Hello, world!");
}

mod tests {
    use super::parser::{query, Query};
    use crate::parser::{QueryArray, QueryBraces};

    #[test]
    fn parse_identity() {
        assert_eq!(query("."), Ok(("", Query::Identity)))
    }

    #[test]
    fn parse_array() {
        let expected = Query::Array(QueryArray {
            query: Box::new(Query::Identity),
        });
        assert_eq!(query("[.]"), Ok(("", expected)))
    }

    #[test]
    fn parse_braces() {
        let expected = Query::Braces(QueryBraces {
            query: Box::new(Query::Identity),
        });
        assert_eq!(query("(.)"), Ok(("", expected)))
    }
}
