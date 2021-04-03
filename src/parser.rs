extern crate nom;
use self::nom::bytes::complete::{take_while, take_while1};
use self::nom::character::complete::{alphanumeric1, digit0, digit1};
use self::nom::character::is_alphabetic;
use self::nom::combinator::not;
use self::nom::error::ParseError;
use self::nom::sequence::pair;
use self::nom::Parser;
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
    pub index: &'q str,
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
        Query::ObjectIndex(QueryObjectIndex { index: pair.1 })
    });
    let string_literal = delimited(char('"'), alphanumeric1, char('"'));
    let query_object_index = combinator::map(
        pair(char('.'), delimited(char('['), string_literal, char(']'))),
        |(_, lit)| Query::ObjectIndex(QueryObjectIndex { index: lit }),
    );
    let query_array_index = combinator::map(
        pair(char('.'), delimited(char('['), digit1, char(']'))),
        |(_, lit): (_, &str)| {
            let index = lit.parse().unwrap();
            Query::ArrayIndex(QueryArrayIndex { index })
        },
    );
    let query_slice = combinator::map(
        pair(
            char('.'),
            delimited(char('['), triple(digit0, char(':'), digit0), char(']')),
        ),
        |(_, (start, _, end)): (_, (&str, _, &str))| {
            let start = match start {
                "" => None,
                s => Some(s.parse().unwrap()),
            };
            let end = match end {
                "" => None,
                s => Some(s.parse().unwrap()),
            };
            Query::Slice(QuerySlice { start, end })
        },
    );
    let query_braces = query_delimited!(Braces, QueryBraces, query_identity, '(', ')');
    let query_array = query_delimited!(Array, QueryArray, query_identity, '[', ']');
    let query_wo_pipe = alt((
        query_array,
        query_braces,
        query_attr,
        query_object_index,
        query_array_index,
        query_slice,
        query_identity,
    ));
    let mut query_pipe = combinator::map(separated_list0(char('|'), query_wo_pipe), |arr| {
        let len = arr.len();
        match (arr, len) {
            (mut arr, 1) => arr.pop().unwrap(),
            (arr, _) => Query::Pipe(QueryPipe { seq: arr }),
        }
    });
    query_pipe(input)
}

fn triple<F1, F2, F3, O1, O2, O3, I, E>(
    mut first: F1,
    mut second: F2,
    mut third: F3,
) -> impl FnMut(I) -> IResult<I, (O1, O2, O3), E>
where
    F1: Parser<I, O1, E>,
    F2: Parser<I, O2, E>,
    F3: Parser<I, O3, E>,
    E: ParseError<I>,
{
    combinator::map(pair(first, pair(second, third)), |(a, (b, c))| (a, b, c))
}
