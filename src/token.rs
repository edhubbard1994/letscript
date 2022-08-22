#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    Assign,
    Equals,
    Plus,
    Minus,
    Mult,
    Div,
    Not,
    And,
    Or,
    Is,
    GreaterThan,
    LessThan,
    Gte,
    Lte,
    Literal,
    Bool,
    Function,
    Quote,
    If,
    Else,
    While,
    For,
    Loop,
    Each,
    Period,
    Colon,
    Comma,
    Object,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracet,
    OpenParen,
    CloseParen,
    NewLine,
}
#[derive(Clone)]
pub struct TokenValue {
    pub s_val: Option<String>,
}

#[derive(Clone)]
pub struct Token {
    pub tok_type: TokenType,
    pub tok_value: Option<TokenValue>,
}
