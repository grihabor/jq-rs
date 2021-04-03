mod parser;

fn main() {
    println!("Hello, world!");
}

mod tests {
    use super::parser::{query, Query};
    use crate::parser::{
        QueryArray, QueryArrayIndex, QueryBraces, QueryObjectIndex, QueryPipe, QuerySlice,
    };

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
        let expected = Query::ObjectIndex(QueryObjectIndex { index: "key" });
        assert_eq!(query(".key"), Ok(("", expected)))
    }

    #[test]
    fn parse_object_index() {
        let expected = Query::ObjectIndex(QueryObjectIndex { index: "key" });
        assert_eq!(query(".[\"key\"]"), Ok(("", expected)))
    }

    #[test]
    fn parse_array_index() {
        let expected = Query::ArrayIndex(QueryArrayIndex { index: 0 });
        assert_eq!(query(".[0]"), Ok(("", expected)))
    }

    #[test]
    fn parse_slice() {
        let expected = Query::Slice(QuerySlice {
            start: Some(0),
            end: Some(2),
        });
        assert_eq!(query(".[0:2]"), Ok(("", expected)));
        let expected = Query::Slice(QuerySlice {
            start: None,
            end: Some(2),
        });
        assert_eq!(query(".[:2]"), Ok(("", expected)));
        let expected = Query::Slice(QuerySlice {
            start: Some(0),
            end: None,
        });
        assert_eq!(query(".[0:]"), Ok(("", expected)));
    }
}
