use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token {
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

    #[regex("\"[^\"]+\"")]
    Text,
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
}