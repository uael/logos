use logos::Lexer;
use logos::Logos as _;
use logos_derive::Logos;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Logos)]
enum Strings {
    #[error]
    Error,

    #[token("\"", sublexer = StringLiteral)]
    String,

    #[regex(r"\p{White_Space}+")]
    WhiteSpace,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Logos)]
#[logos(flags(early_exit))]
enum StringLiteral {
    #[error]
    Error,

    #[regex(r#"[^\\"]+"#)]
    Text,

    #[token("\\n")]
    EscapedNewline,

    #[regex(r"\\u\{[^}]*\}")]
    EscapedCodepoint,

    #[token(r#"\""#)]
    EscapedQuote,

    #[token("\"", logos::inclusive_end)]
    EndString,
}

#[test]
fn main() {
    let s = r#""Hello W\u{00f4}rld\n"  "yolo" " a \" b ""#;
    let mut outer = Strings::lexer(s);

    while let Some(token) = outer.next() {
        println!("{:?} => [{:?}] '{}'", token, outer.span(), outer.slice())
    }
}
