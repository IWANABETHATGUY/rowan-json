use std::{fmt, iter::Peekable};

use crate::lexer::{Lexer, SyntaxKind};

pub enum Value<'a> {
    String(&'a str),
    Boolean(bool),
    Null,
    Number(f64),
    Object(Vec<(&'a str, Value<'a>)>),
    Array(Vec<Value<'a>>),
}

impl<'a> fmt::Display for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(float) => write!(f, "{}", float),
            Value::String(string) => write!(f, "{}", string),
            Value::Object(obj) => {
                write!(f, "{{")?;
                if let Some(((key, value), rest)) = obj.split_first() {
                    write!(f, "{}: {}", key, value)?;
                    for (key, value) in rest.iter() {
                        write!(f, ", {} : {}", key, value)?
                    }
                }
                write!(f, "}}")
            }
            Value::Boolean(flag) => write!(f, "{}", flag),
            Value::Null => write!(f, "null"),
            Value::Array(array) => {
                write!(f, "[")?;
                if let Some((value, rest)) = array.split_first() {
                    write!(f, "{}", value)?;
                    for value in rest.iter() {
                        write!(f, ", {}", value)?
                    }
                }
                write!(f, "]")
            }
        }
    }
}

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input).peekable(),
        }
    }

    pub fn parse(&mut self) -> Value<'a> {
        self.parse_element()
    }

    pub fn parse_element(&mut self) -> Value<'a> {
        self.skip_whitespace();
        let res = match self.peek() {
            Some(t) => match t {
                SyntaxKind::LeftBrace => self.parse_object(),
                SyntaxKind::LeftBracket => self.parse_array(),
                SyntaxKind::True => {
                    self.bump();
                    Value::Boolean(true)
                }
                SyntaxKind::False => {
                    self.bump();
                    Value::Boolean(false)
                }
                SyntaxKind::Null => {
                    self.bump();
                    Value::Null
                }
                SyntaxKind::String => {
                    let (_, inner) = self.bump();
                    Value::String(inner)
                }
                SyntaxKind::Number => {
                    let (_, inner) = self.bump();
                    let res = inner.parse().unwrap();
                    Value::Number(res)
                }
                SyntaxKind::Whitespace
                | SyntaxKind::Error
                | SyntaxKind::RightBrace
                | SyntaxKind::Colon
                | SyntaxKind::Comma
                | SyntaxKind::RightBracket => unreachable!(),
                _ => {
                    unreachable!()
                }
            },
            None => {
                unreachable!("expected have a element")
            }
        };
        self.skip_whitespace();
        res
    }

    pub fn parse_member(&mut self) -> (&'a str, Value<'a>) {
        self.skip_whitespace();

        let key = match self.peek() {
            Some(SyntaxKind::String) => self.bump().1,
            None => todo!(),
            _ => {
                let res = self.lexer.next().unwrap();
                panic!("{:?}", res);
            }
        };
        self.skip_whitespace();
        assert!(matches!(self.peek(), Some(SyntaxKind::Colon)));
        self.bump();
        (key, self.parse_element())
    }
    pub(crate) fn parse_array(&mut self) -> Value<'a> {
        self.bump();
        self.skip_whitespace();
        let mut ret = vec![];
        if self.peek() != Some(SyntaxKind::RightBracket) {
            ret.push(self.parse_element());
        }
        self.skip_whitespace();
        while let Some(SyntaxKind::Comma) = self.peek() {
            self.bump();
            ret.push(self.parse_element());
        }
        if self.peek() == Some(SyntaxKind::RightBracket) {
            self.bump();
        } else {
            unimplemented!("should be ] to finish array") // TODO
        }
        Value::Array(ret)
    }

    pub(crate) fn parse_object(&mut self) -> Value<'a> {
        self.bump();
        self.skip_whitespace();
        let mut ret: Vec<(&'a str, Value<'a>)> = vec![];
        if self.peek() != Some(SyntaxKind::RightBrace) {
            ret.push(self.parse_member());
        }

        while let Some(SyntaxKind::Comma) = self.peek() {
            self.bump();
            ret.push(self.parse_member());
        }
        if self.peek() == Some(SyntaxKind::RightBrace) {
            self.bump();
        } else {
            unimplemented!(r#"should be {} to finish object, {:?}"#, "}", self.peek())
            // TODO
        }
        Value::Object(ret)
    }
    pub fn skip_whitespace(&mut self) {
        while let Some(SyntaxKind::Whitespace) = self.peek() {
            self.bump();
        }
    }

    fn peek(&mut self) -> Option<SyntaxKind> {
        self.lexer.peek().map(|(kind, _)| *kind)
    }

    fn bump(&mut self) -> (SyntaxKind, &'a str) {
        self.lexer.next().unwrap()
    }
}
