use json_pop::{parse_str, value::Value};
use logos::Logos;
use mimalloc_rust::*;
use rayon::prelude::*;
use rowan::{GreenNode, GreenToken, NodeOrToken, SyntaxElement, TextRange};
use rowan_json::recursive;
use rowan_json::{lexer::SyntaxKind, parser::Parser, syntax::SyntaxNode, syntax::SyntaxToken};
use std::{ops::Deref, time::Instant};

#[global_allocator]
static GLOBAL_MIMALLOC: GlobalMiMalloc = GlobalMiMalloc;
fn main() {
    let string = include_str!("../assets/large.json");
    rowan_traverse(string);
}

fn rowan_traverse(string: &str) {
    let start = Instant::now();
    let parse = Parser::new(string);
    let res = parse.parse();
    let mut _root = rowan_json::syntax::SyntaxNode::new_root(res.green_node);
    println!("rowan {:?}", start.elapsed());
    let mut iter = _root.preorder();
    let now = Instant::now();
    while let Some(event) = iter.next() {
        match event {
            rowan::WalkEvent::Leave(node) => {
                // if node.kind() == SyntaxKind::Whitespace {
                //     // node_list.push(node);
                // }
                // println!("leave {:?}, ", node.kind());
            }
            _ => {}
        }
    }
    traverse(_root.clone());
    println!("traverse rowan {:?}", now.elapsed());
    let start = Instant::now();
    let mut _res = format!("{}", _root);
    // let _res = format!("{}", root);

    println!("stringify rowan {:?}", start.elapsed());

    let start = Instant::now();
    let mut _res = parse_str(string).unwrap();
    // let _res = format!("{}", root);
    println!("parse lr {:?}", start.elapsed());

    let start = Instant::now();
    traverse_lr(&mut _res);
    println!("traverse_lr {:?}", start.elapsed());

    let start = Instant::now();
    let _string_lr = format!("{}", _res);
    println!("stringify {:?}", start.elapsed());

    let start = Instant::now();
    json(string).unwrap();
    println!("nom {:?}", start.elapsed());

    let start = Instant::now();
    let mut parser = recursive::Parser::new(string);
    let mut _res = parser.parse();
    println!("recursive traverser {:?}", start.elapsed());

    let start = Instant::now();
    traverse_recursive(&mut _res);
    println!("recursive traverse {:?}", start.elapsed());

    let start = Instant::now();
    let _string = format!("{}", _res);
    // println!("{}", _string);
    println!("recursive stringify {:?}", start.elapsed());
}

fn traverse_lr(value: &mut Value) {
    match value {
        Value::Number(_) => {}
        Value::String(string) => {}
        Value::Object(v) => {
            v.iter_mut().for_each(|item| {
                traverse_lr(&mut item.1);
            });
            // v.push(("key", Value::Bool(false)));
        }
        Value::Bool(_) => {}
        Value::Null => {}
        Value::Array(value) => {
            value.iter_mut().for_each(|v| {
                traverse_lr(v);
            });
            // value.push(Value::String("array"));
        }
    }
}

fn traverse_recursive(value: &mut recursive::Value) {
    match value {
        recursive::Value::String(_) => (),
        recursive::Value::Boolean(_) => (),
        recursive::Value::Null => (),
        recursive::Value::Number(_) => (),
        recursive::Value::Object(v) => v
            .iter_mut()
            .for_each(|item| traverse_recursive(&mut item.1)),
        recursive::Value::Array(value) => {
            value.iter_mut().for_each(|v| {
                traverse_recursive(v);
            });
        }
    }
}

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{anychar, char, multispace0, none_of},
    combinator::{map, map_opt, map_res, value, verify},
    error::{ErrorKind, ParseError},
    multi::{fold_many0, separated_list0},
    number::complete::{double, recognize_float},
    sequence::{delimited, preceded, separated_pair},
    IResult, Parser as NomParser,
};

use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Str(String),
    Num(f64),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

fn boolean(input: &str) -> IResult<&str, bool> {
    alt((value(false, tag("false")), value(true, tag("true"))))(input)
}

fn u16_hex(input: &str) -> IResult<&str, u16> {
    map_res(take(4usize), |s| u16::from_str_radix(s, 16))(input)
}

fn unicode_escape(input: &str) -> IResult<&str, char> {
    map_opt(
        alt((
            // Not a surrogate
            map(verify(u16_hex, |cp| !(0xD800..0xE000).contains(cp)), |cp| {
                cp as u32
            }),
            // See https://en.wikipedia.org/wiki/UTF-16#Code_points_from_U+010000_to_U+10FFFF for details
            map(
                verify(
                    separated_pair(u16_hex, tag("\\u"), u16_hex),
                    |(high, low)| (0xD800..0xDC00).contains(high) && (0xDC00..0xE000).contains(low),
                ),
                |(high, low)| {
                    let high_ten = (high as u32) - 0xD800;
                    let low_ten = (low as u32) - 0xDC00;
                    (high_ten << 10) + low_ten + 0x10000
                },
            ),
        )),
        // Could probably be replaced with .unwrap() or _unchecked due to the verify checks
        std::char::from_u32,
    )(input)
}

fn character(input: &str) -> IResult<&str, char> {
    let (input, c) = none_of("\"")(input)?;
    if c == '\\' {
        alt((
            map_res(anychar, |c| {
                Ok(match c {
                    '"' | '\\' | '/' => c,
                    'b' => '\x08',
                    'f' => '\x0C',
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    _ => return Err(()),
                })
            }),
            preceded(char('u'), unicode_escape),
        ))(input)
    } else {
        Ok((input, c))
    }
}

fn string(input: &str) -> IResult<&str, String> {
    delimited(
        char('"'),
        fold_many0(character, String::new, |mut string, c| {
            string.push(c);
            string
        }),
        char('"'),
    )(input)
}

fn ws<'a, O, E: ParseError<&'a str>, F: NomParser<&'a str, O, E>>(
    f: F,
) -> impl NomParser<&'a str, O, E> {
    delimited(multispace0, f, multispace0)
}

fn array(input: &str) -> IResult<&str, Vec<JsonValue>> {
    delimited(
        char('['),
        ws(separated_list0(ws(char(',')), json_value)),
        char(']'),
    )(input)
}

fn object(input: &str) -> IResult<&str, HashMap<String, JsonValue>> {
    map(
        delimited(
            char('{'),
            ws(separated_list0(
                ws(char(',')),
                separated_pair(string, ws(char(':')), json_value),
            )),
            char('}'),
        ),
        |key_values| key_values.into_iter().collect(),
    )(input)
}

fn json_value(input: &str) -> IResult<&str, JsonValue> {
    use JsonValue::*;

    alt((
        value(Null, tag("null")),
        map(boolean, Bool),
        map(string, Str),
        map(double, Num),
        map(array, Array),
        map(object, Object),
    ))(input)
}

fn json(input: &str) -> IResult<&str, JsonValue> {
    ws(json_value).parse(input)
}

use nom::Err;
use nom::ParseTo;
fn std_float(input: &[u8]) -> IResult<&[u8], f64, (&[u8], ErrorKind)> {
    match recognize_float(input) {
        Err(e) => Err(e),
        Ok((i, s)) => match s.parse_to() {
            Some(n) => Ok((i, n)),
            None => Err(Err::Error((i, ErrorKind::Float))),
        },
    }
}

fn traverse(root: SyntaxNode) {
    for child in root.children() {
        traverse(child);
    }
}
