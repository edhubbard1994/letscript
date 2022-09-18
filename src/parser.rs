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

pub fn precedence(tok: &Token) -> u8 {
    return match tok.tok_type {
        TokenType::Mult => 255,
        TokenType::Div => 255,
        TokenType::Plus => 254,
        TokenType::Minus => 254,
        _ => 0,
    };
}

pub fn infix_to_postfix(tokens: Vec<Token>) -> Vec<Token> {
    let mut operator_stack = LinkedList::<Token>::new();
    let mut operand_queue = Vec::<Token>::new();

    for token in tokens {
        // operand_queue.iter().for_each(|t| {
        //     if t.tok_type == TokenType::Literal {
        //         print!(" {:?} ", t.tok_value.clone().unwrap().s_val)
        //     } else {
        //         print!(" {:?} ", t.tok_type)
        //     }
        // });
        // print!(" | ");
        // println!("stacksize({})", operator_stack.len());
        let is_operand = token.tok_type == TokenType::Literal;
        if is_operand {
            operand_queue.push(token.clone())
        } else if token.tok_type == TokenType::OpenParen {
            operator_stack.push_front(token.clone());
        } else if token.tok_type == TokenType::CloseParen {
            while operator_stack.is_empty() == false
                && operator_stack.front().unwrap().tok_type != TokenType::OpenParen
            {
                operand_queue.push(operator_stack.pop_front().unwrap().clone());
            }
            operator_stack.pop_front();
        } else {
            let top = operator_stack.front();
            let prec = precedence(&token);
            let mut stack_prec;
            match top {
                Some(x) => {
                    stack_prec = precedence(x);
                    if prec <= stack_prec {
                        let tok = operator_stack.pop_front().unwrap().clone();
                        operand_queue.push(tok.clone());
                        stack_prec = precedence(&tok);
                    }
                }
                None => stack_prec = 0,
            }

            while operator_stack.is_empty() == false && prec < stack_prec {
                let tok = operator_stack.pop_front().unwrap().clone();
                stack_prec = precedence(&tok);
                operand_queue.push(tok.clone());
            }
            operator_stack.push_front(token.clone());
        }
    }
    while operator_stack.is_empty() == false {
        let tok = operator_stack.pop_front().unwrap().clone();
        operand_queue.push(tok);
    }

    // operand_queue
    //     .iter()
    //     .for_each(|t| println!("{:?}", t.tok_type));

    return operand_queue;
}

pub fn eval_expression() {}
