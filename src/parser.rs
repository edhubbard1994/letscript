use crate::token::Token;
use crate::token::TokenType;

use std::collections::HashMap;
use std::collections::LinkedList;
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
) -> (Token, Vec<Token>) {
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
    return (current.clone(), acc);
}

pub fn infix_to_postfix(tokens: Vec<Token>) -> Vec<Token> {
    let mut operator_stack = LinkedList::<(Token, u8)>::new();
    let mut operand_queue = Vec::<Token>::new();

    let mut last_prec: u8 = 255;
    for token in tokens {
        let mut prec = 0;
        let mut is_operator = true;
        /*
        *NOTE* OPERATOR PRECENDENCE NUMBERS:
        Paren: 255
        Add, Sub: 128

         */
        match token.tok_type {
            TokenType::OpenParen => {}
            TokenType::CloseParen => {}
            TokenType::Plus | TokenType::Minus => prec = 128,
            TokenType::Literal => is_operator = false,
            _ => todo!("return error condition"),
        }
        if is_operator && prec < last_prec {
            operator_stack.push_front((token, prec))
        } else if !operator_stack.is_empty() && prec > last_prec {
            loop {
                if operator_stack.is_empty() {
                    break;
                }
                if prec > last_prec {
                    break;
                }
                let op = operator_stack.pop_front().unwrap();
                last_prec = op.1;
                println!("pushed token with type: {:?}", op.0.tok_type);
                operand_queue.push(op.0);
            }
        } else {
            operand_queue.push(token);
        }
    }
    return operand_queue;
}

pub fn eval_expression() {}
