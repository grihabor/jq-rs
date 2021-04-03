mod parser;

fn main() {
    println!("Hello, world!");
}

mod tests {
    use super::parser::{query, Query};
    use crate::parser::{QueryArray, QueryBraces, QueryObjectIndex, QueryPipe};

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

    #[test]
    fn parse_pipe() {
        let expected = Query::Pipe(QueryPipe {
            seq: vec![Query::Identity, Query::Identity],
        });
        assert_eq!(query(".|."), Ok(("", expected)))
    }

    #[test]
    fn parse_attr() {
        let expected = Query::ObjectIndex(QueryObjectIndex { attr: "key" });
        assert_eq!(query(".key"), Ok(("", expected)))
    }

    #[test]
    fn parse_array_index() {
        let expected = Query::ObjectIndex(QueryObjectIndex { attr: "key" });
        assert_eq!(query(".[0]"), Ok(("", expected)))
    }

    #[test]
    fn parse_slice() {
        let expected = Query::ObjectIndex(QueryObjectIndex { attr: "key" });
        assert_eq!(query(".[0:2]"), Ok(("", expected)))
    }
}
