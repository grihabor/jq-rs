extern crate nom;
use self::nom::bytes::complete::{take_while, take_while1};
use self::nom::character::complete::alphanumeric1;
use self::nom::character::is_alphabetic;
use self::nom::sequence::pair;
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::IResult;

#[derive(PartialEq, Eq, Debug)]
pub enum Query<'q> {
    Identity,
    ObjectIndex(QueryObjectIndex<'q>),
    ArrayIndex(QueryArrayIndex),
    Slice(QuerySlice),
    Expr(QueryExpr),
    Filter(QueryFilter),
    Pipe(QueryPipe<'q>),
    Braces(QueryBraces<'q>),
    Array(QueryArray<'q>),
}

#[derive(PartialEq, Eq, Debug)]
pub struct QuerySlice {
    pub start: Option<i64>,
    pub end: Option<i64>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct QueryObjectIndex<'q> {
    pub attr: &'q str,
}

#[derive(PartialEq, Eq, Debug)]
pub struct QueryBraces<'q> {
    pub query: Box<Query<'q>>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct QueryArray<'q> {
    pub query: Box<Query<'q>>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct QueryPipe<'q> {
    pub seq: Vec<Query<'q>>,
}

#[derive(PartialEq, Eq, Debug)]
struct QueryExpr {}

#[derive(PartialEq, Eq, Debug)]
struct QueryFilter {}

#[derive(PartialEq, Eq, Debug)]
pub struct QueryArrayIndex {
    pub index: i64,
}

macro_rules! query_delimited {
    ($name: ident, $full: ident, $query: expr, $left: literal, $right: literal) => {{
        let parse_delimited = delimited(char($left), $query, char($right));
        combinator::map(parse_delimited, |query: Query| {
            Query::$name($full {
                query: Box::new(query),
            })
        })
    }};
}

fn query_identity(input: &str) -> IResult<&str, Query> {
    combinator::map(char('.'), |_| Query::Identity)(input)
}

pub fn query(input: &str) -> IResult<&str, Query> {
    let query_attr = combinator::map(pair(char('.'), alphanumeric1), |pair| {
        Query::ObjectIndex(QueryObjectIndex { attr: pair.1 })
    });
    let query_braces = query_delimited!(Braces, QueryBraces, query_identity, '(', ')');
    let query_array = query_delimited!(Array, QueryArray, query_identity, '[', ']');
    let query_wo_pipe = alt((query_array, query_braces, query_attr, query_identity));
    let mut query_pipe = combinator::map(separated_list0(char('|'), query_wo_pipe), |arr| {
        let len = arr.len();
        match (arr, len) {
            (mut arr, 1) => arr.pop().unwrap(),
            (arr, _) => Query::Pipe(QueryPipe { seq: arr }),
        }
    });
    query_pipe(input)
}
