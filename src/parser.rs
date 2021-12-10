use std::iter::Peekable;

use crate::lexer::{Lexer, SyntaxKind};
use crate::syntax::Json;
use rowan::{GreenNode, GreenNodeBuilder, Language};

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    builder: GreenNodeBuilder<'static>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input).peekable(),
            builder: GreenNodeBuilder::new(),
        }
    }

    pub fn parse(mut self) -> Parse {
        self.builder.start_node(SyntaxKind::Root.into());
        self.parse_element();
        self.builder.finish_node();
        Parse {
            green_node: self.builder.finish(),
        }
    }

    pub fn parse_element(&mut self) {
        self.skip_whitespace();
        if let Some(t) = self.peek() {
            match t {
                SyntaxKind::LeftBrace => self.parse_object(),
                SyntaxKind::LeftBracket => self.parse_array(),
                SyntaxKind::True => self.bump(),
                SyntaxKind::False => self.bump(),
                SyntaxKind::Null => self.bump(),
                SyntaxKind::String => self.bump(),
                SyntaxKind::Number => self.bump(),
                // SyntaxKind::Whitespace => unreachable!(),
                // SyntaxKind::Error => todo!(),
                // SyntaxKind::RightBrace => unreachable!(),
                // SyntaxKind::Colon => unreachable!(),
                // SyntaxKind::Comma => unreachable!(),
                // SyntaxKind::RightBracket => unreachable!(),
                _ => {
                    unreachable!()
                }
            }
        }
        self.skip_whitespace();
    }
    pub fn parse_member(&mut self) {
        self.skip_whitespace();
        match self.peek() {
            Some(SyntaxKind::String) => {
                self.bump();
            }
            None => todo!(),
            _ => {
                let res = self.lexer.next().unwrap();
                panic!("{:?}", res);
            }
        }
        self.skip_whitespace();
        assert!(matches!(self.peek(), Some(SyntaxKind::Colon)));
        self.bump();
        self.parse_element();
    }
    pub(crate) fn parse_array(&mut self) {
        self.start_node(SyntaxKind::Array);
        self.bump();
        self.skip_whitespace();
        if self.peek() != Some(SyntaxKind::RightBracket) {
            self.parse_element();
        }
        self.skip_whitespace();
        while let Some(SyntaxKind::Comma) = self.peek() {
            self.bump();
            self.parse_element();
        }
        if self.peek() == Some(SyntaxKind::RightBracket) {
            self.bump();
            self.finish_node();
        } else {
            unimplemented!("should be ] to finish array") // TODO
        }
    }

    pub(crate) fn parse_object(&mut self) {
        self.start_node(SyntaxKind::Object);
        self.bump();
        self.skip_whitespace();
        if self.peek() != Some(SyntaxKind::RightBrace) {
            self.parse_member();
        }

        while let Some(SyntaxKind::Comma) = self.peek() {
            self.bump();
            self.parse_member();
        }
        if self.peek() == Some(SyntaxKind::RightBrace) {
            self.bump();
            self.finish_node();
        } else {
            unimplemented!("should be ] to finish array") // TODO
        }
    }
    pub fn skip_whitespace(&mut self) {
        while let Some(SyntaxKind::Whitespace) = self.peek() {
            self.bump();
        }
    }
    fn peek(&mut self) -> Option<SyntaxKind> {
        self.lexer.peek().map(|(kind, _)| *kind)
    }

    fn bump(&mut self) {
        let (kind, text) = self.lexer.next().unwrap();

        self.builder.token(Json::kind_to_raw(kind), text);
    }
    fn start_node(&mut self, kind: SyntaxKind) {
        self.builder.start_node(Json::kind_to_raw(kind));
    }

    fn finish_node(&mut self) {
        self.builder.finish_node();
    }
}

pub struct Parse {
    pub green_node: GreenNode,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::SyntaxNode;
    use expect_test::{expect, Expect};

    fn check(input: &str, expected_tree: Expect) {
        let parse = Parser::new(input).parse();
        let syntax_node = SyntaxNode::new_root(parse.green_node);

        let actual_tree = format!("{:#?}", syntax_node);

        // We cut off the last byte because formatting the SyntaxNode adds on a newline at the end.
        expected_tree.assert_eq(&actual_tree[0..actual_tree.len() - 1]);
    }

    #[test]
    fn parse_nothing() {
        check(r#""#, expect![[r#"Root@0..0"#]]);
    }

    #[test]
    fn parse_number() {
        check(
            r#"123"#,
            expect![[r#"
Root@0..3
  Number@0..3 "123""#]],
        );
    }
}
