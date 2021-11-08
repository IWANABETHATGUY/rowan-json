use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Logos, Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub(crate) enum SyntaxKind {
    #[token("{")]
    LeftBrace,

    #[token("}")]
    RightBrace,

    #[token(":")]
    Colon,

    #[token(",")]
    Comma,

    #[token("[")]
    LeftBracket,

    #[token("]")]
    RightBracket,

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("null")]
    Null,

    // (?<string>    " ([^"\\\\]* | \\\\ ["\\\\bfnrt\/] | \\\\ u [0-9a-f]{4} )* " )
    // ||\\u [0-9a-f]{4})*
    #[regex(r#""\\["\\/bfnrt]""#)]
    #[regex(r#""\\u[0-9a-f][0-9a-f][0-9a-f][0-9a-f]""#)]
    #[regex(r#""([^"\\])*""#)]
    String,
    // #[regex(r#"\."#)]
    // Text1,
    #[regex(r#"-?(?:0|[1-9][0-9]*)(?:\.[0-9]+)?(?:[eE][+-]?[0-9]+)?"#)]
    Number,

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\n\f]+")]
    Whitespace,

    #[error]
    Error,


    Root
}



impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}