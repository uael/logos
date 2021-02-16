use logos::Lexer;
use logos::Logos as _;
use logos_derive::Logos;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Logos)]
enum Strings {
    #[error]
    Error,

    #[regex(r"(foo) (yolo)")]
    Yolo,
}

#[test]
fn main() {
    let s = r#"foo yolo"#;
    let mut outer = Strings::lexer(s);

    while let Some(token) = outer.next() {
        println!("{:?} => [{:?}] '{}'", token, outer.span(), outer.slice())
    }
}
