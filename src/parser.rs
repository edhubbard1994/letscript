use crate::token::Token;
use crate::token::TokenType;

use std::slice::Iter;
use std::vec;

pub fn parse(tokens: &mut Vec<Token>) {
    let mut itr = tokens.iter();
    let mut token = itr.next();
    while token.is_some() {
        match token.unwrap().tok_type {
            TokenType::Literal => {}
            TokenType::Assign => todo!(),
            TokenType::Equals => todo!(),
            TokenType::Plus => todo!(),
            TokenType::Minus => todo!(),
            TokenType::Mult => todo!(),
            TokenType::Div => todo!(),
            TokenType::Not => todo!(),
            TokenType::And => todo!(),
            TokenType::Or => todo!(),
            TokenType::Is => todo!(),
            TokenType::GreaterThan => todo!(),
            TokenType::LessThan => todo!(),
            TokenType::Gte => todo!(),
            TokenType::Lte => todo!(),
            TokenType::Bool => todo!(),
            TokenType::Function => todo!(),
            TokenType::Quote => todo!(),
            TokenType::If => todo!(),
            TokenType::Else => todo!(),
            TokenType::While => todo!(),
            TokenType::For => todo!(),
            TokenType::Loop => todo!(),
            TokenType::Each => todo!(),
            TokenType::Period => todo!(),
            TokenType::Colon => todo!(),
            TokenType::Comma => todo!(),
            TokenType::Object => todo!(),
            TokenType::OpenBrace => todo!(),
            TokenType::CloseBrace => todo!(),
            TokenType::OpenBracket => todo!(),
            TokenType::CloseBracet => todo!(),
            TokenType::OpenParen => todo!(),
            TokenType::CloseParen => todo!(),
            TokenType::NewLine => todo!(),
        }
        token = itr.next();
    }
}

pub fn collect_expression_tokens<'a>(
    current_token: &'a Token,
    tokens: &'a mut Iter<'a, Token>,
) -> Vec<Token> {
    let possible_tokens = vec![
        TokenType::OpenParen,
        TokenType::CloseParen,
        TokenType::Literal,
        TokenType::Plus,
        TokenType::Minus,
    ];
    let t = tokens;
    let mut current = t.next().unwrap();
    let mut acc: Vec<Token> = Vec::new();
    while possible_tokens.contains(&current.tok_type) {
        let val = current.tok_value.clone();
        if val.is_some() {
            println!("{:?}", val.unwrap().s_val.unwrap());
        }

        acc.push(current.clone());
        current = t.next().unwrap();
    }
    return acc;
}
