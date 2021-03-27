extern crate nom;
use nom::bytes::streaming::tag;
use nom::combinator;
use nom::sequence::delimited;
use nom::IResult;

#[derive(PartialEq, Eq, Debug)]
pub enum Query {
    Expr(QueryExpr),
    Filter(QueryFilter),
    Index(QueryIndex),
    Identity,
    Pipe(QueryPipe),
    Braces(QueryBraces),
    Array(QueryArray),
}

#[derive(PartialEq, Eq, Debug)]
pub struct QueryBraces {
    pub query: Box<Query>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct QueryArray {
    pub query: Box<Query>,
}

#[derive(PartialEq, Eq, Debug)]
struct QueryPipe {}

#[derive(PartialEq, Eq, Debug)]
struct QueryExpr {}

#[derive(PartialEq, Eq, Debug)]
struct QueryFilter {}

#[derive(PartialEq, Eq, Debug)]
struct QueryIndex {}

pub fn query(input: &str) -> IResult<&str, Query> {
    nom::branch::alt((query_array, query_braces, query_pipe))(input)
}

macro_rules! query_delimited {
    ($name: ident, $full: ident, $left: literal, $right: literal) => {{
        let parse_delimited = delimited(tag($left), query, tag($right));
        combinator::map(parse_delimited, |query: Query| {
            Query::$name($full {
                query: Box::new(query),
            })
        })
    }};
}

fn query_braces(input: &str) -> IResult<&str, Query> {
    query_delimited!(Braces, QueryBraces, "(", ")")(input)
}

fn query_array(input: &str) -> IResult<&str, Query> {
    query_delimited!(Array, QueryArray, "[", "]")(input)
}

fn query_pipe(input: &str) -> IResult<&str, Query> {
    combinator::map(tag("."), |_| Query::Identity)(input)
}
