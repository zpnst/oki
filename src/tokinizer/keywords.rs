use crate::token::Token;

pub fn filter(ik: String) -> Token {
    match ik.as_str() {
        "define" => Token::DEFINE,
        "oki"     => Token::OKI,
        "true"   => Token::TRUE,
        "false"  => Token::FALSE,
        "if"     => Token::IF,
        "else"   => Token::ELSE,
        "ret"    => Token::RET,
        _        => Token::IDENT(ik)
    }
}