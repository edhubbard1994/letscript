//use crate::ast::token_to_operable;

use crate::expr::operate;
use crate::expr::operate_unary;

use crate::token::Token;
use crate::token::TokenType;
use crate::token::TokenValue;

use crate::ast;

use std::collections::LinkedList;
use std::slice::Iter;
use std::vec;

pub fn parse(tokens: &mut Vec<Token>) {
    let mut itr = tokens.iter();
    let mut token = itr.next();

    match token.unwrap().tok_type {
        TokenType::Assign => parse_assignment(tokens),
        TokenType::Bool => todo!(),
        TokenType::Function => todo!(),
        TokenType::Quote => todo!(),
        TokenType::If => todo!(),
        TokenType::Else => todo!(),
        TokenType::While => todo!(),
        TokenType::For => todo!(),
        TokenType::Loop => todo!(),
        TokenType::Object => todo!(),
        TokenType::OpenBrace => todo!(),
        TokenType::CloseBrace => todo!(),
        TokenType::OpenBracket => todo!(),
        TokenType::CloseBracket => todo!(),
        TokenType::OpenParen => todo!(),
        TokenType::CloseParen => todo!(),
        TokenType::NewLine => todo!(),
        TokenType::TokenError => todo!(),
        _ => {}
    }
}

pub fn parse_assignment(tokens: &mut Vec<Token>) {
    match (tokens[0].tok_type, tokens[1].tok_type, tokens[2].tok_type) {
        (TokenType::Assign, TokenType::Literal, TokenType::Is) => {
            let name = tokens[1].tok_value.clone().unwrap().s_val.unwrap();
            let value = parse_expression(&mut tokens[3..].to_vec());
            ast::CALL_STACK.add_symbol(name, value);
            println!("{:?}", ast::CALL_STACK)
        }
        (_, _, _) => panic!("keyword cannot be used as a variable name"),
    }
}

fn parse_expression(expr: &mut Vec<Token>) -> ast::SymbolType {
    match expr[0].tok_type {
        //TokenType::OpenBracket => {}
        TokenType::Literal | TokenType::Minus | TokenType::Not => {
            if expr[expr.len() - 1].tok_type == TokenType::NewLine {
                expr.pop();
            }
            let resolved_vars = resolve_symbols(expr.clone().to_vec());
            println!("{:?}", resolved_vars);
            let non_unary = resolve_unary_operators(resolved_vars.clone().to_vec());
            let postfix = infix_to_postfix(non_unary.clone());
            let evaluated = eval_expression(&mut postfix.clone());
            return ast::SymbolType::Number(evaluated.tok_value.unwrap().s_val.unwrap());
        }
        TokenType::Quote => {
            if expr[expr.len() - 1].tok_type == TokenType::NewLine {
                expr.pop();
            }
            return parse_string(expr.clone().to_vec());
        }
        TokenType::OpenBracket => {
            return parse_array(expr.clone().to_vec());
        }
        _ => panic!("invalid expression syntax"),
    }
}

fn parse_string(tokens: Vec<Token>) -> ast::SymbolType {
    if tokens.len() < 3 {
        panic!("invalid string syntax");
    }
    let mut itr = tokens.iter();
    let mut token = itr.next();
    println!("{:?}", tokens);
    if token.unwrap().tok_type != TokenType::Quote {
        panic!("invalid string syntax 2");
    }
    println!("before loop");
    token = itr.next();
    let mut value = "".to_string();
    while token.is_some() {
        println!("parse string: {:?}", token.clone().unwrap().tok_type);
        match token.clone().unwrap().tok_type {
            TokenType::Literal => {
                value = token
                    .unwrap()
                    .clone()
                    .tok_value
                    .unwrap()
                    .s_val
                    .unwrap()
                    .clone();
            }
            TokenType::Quote => break,
            _ => panic!("invalid string syntax 3"),
        }
        token = itr.next();
    }
    println!("after loop");
    ast::SymbolType::String(value)
}

fn parse_array(tokens: Vec<Token>) -> ast::SymbolType {
    let mut symbols = Vec::<ast::SymbolType>::new();
    for mut i in 1..tokens.len() - 1 {
        match tokens[i].tok_type {
            TokenType::OpenBracket => {
                let nested = tokens[i..].to_vec();
                symbols.push(parse_array(nested));
            }
            TokenType::CloseBracket => break,
            TokenType::Literal => {
                let existing_var = ast::CALL_STACK
                    .lookup_symbol(tokens[i].clone().tok_value.unwrap().s_val.unwrap());
                if existing_var.is_some() {
                    symbols.push(match existing_var.unwrap() {
                        _ => ast::SymbolType::Pointer(
                            tokens[i].clone().tok_value.unwrap().s_val.unwrap(),
                        ),
                    });
                    continue;
                }

                symbols.push(ast::SymbolType::Number(
                    tokens[i].clone().tok_value.unwrap().s_val.unwrap(),
                ));
            }
            TokenType::Comma => continue,
            TokenType::Quote => {
                println!("string: {:?}", tokens[i..][0]);
                let str_slice = tokens[i..i + 3].to_vec();
                symbols.push(parse_string(str_slice));
                i += 3;
            }

            _ => panic!("invalid array syntax"),
        }
    }
    return ast::SymbolType::Array(symbols);
}

pub fn resolve_symbols(tokens: Vec<Token>) -> Vec<Token> {
    let mut new_tokens = Vec::<Token>::new();
    for token in tokens {
        match token.clone().tok_type {
            TokenType::Literal => {
                let symbol =
                    ast::CALL_STACK.lookup_symbol(token.clone().tok_value.unwrap().s_val.unwrap());
                match symbol.clone() {
                    Some(sym) => {
                        new_tokens.push(Token {
                            tok_type: TokenType::Literal,
                            tok_value: Some(TokenValue {
                                s_val: match symbol.clone().unwrap() {
                                    ast::SymbolType::Number(s) => Some(s),
                                    ast::SymbolType::String(s) => Some(s),
                                    _ => panic!("invalid symbol type"),
                                },
                            }),
                        });
                    }
                    None => {
                        new_tokens.push(token.clone());
                    }
                }
            }
            _ => {
                new_tokens.push(token);
            }
        }
    }
    return new_tokens;
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

pub fn find_number_sign_helper(mut itr: std::vec::IntoIter<Token>) -> Token {
    let mut token = itr.next();
    while token.is_some()
        && [TokenType::Plus, TokenType::Minus].contains(&token.clone().unwrap().tok_type)
    {}
    return Token {
        tok_type: TokenType::TokenError,
        tok_value: None,
    };
}

pub fn resolve_unary_operators(mut tokens: Vec<Token>) -> Vec<Token> {
    let mut z = 0;
    let mut new_tokens = Vec::<Option<Token>>::new();
    let mut window = (
        None,
        None,
        match tokens.get_mut(z) {
            Some(token) => Some(token.tok_type),
            None => None,
        },
    );
    while z < tokens.len() {
        match window {
            (
                Some(
                    TokenType::Minus
                    | TokenType::Plus
                    | TokenType::Mult
                    | TokenType::Div
                    | TokenType::Mod,
                )
                | None,
                Some(TokenType::Minus),
                Some(TokenType::Literal),
            ) => {
                let mut val = String::from("-");
                println!("{:?}", tokens[z].tok_type);
                val.push_str(tokens[z].tok_value.clone().unwrap().s_val.unwrap().as_str());
                new_tokens.pop();
                new_tokens.push(None);
                new_tokens.push(Some(Token {
                    tok_type: TokenType::Literal,
                    tok_value: Some(TokenValue { s_val: Some(val) }),
                }))
            }
            (_, Some(TokenType::Not), Some(TokenType::Literal)) => {
                let val = tokens[z].clone();
                let operator = tokens[z - 1].clone();
                let negated = operate_unary(val, operator);
                new_tokens.pop();
                new_tokens.push(Some(negated));
            }
            (_, Some(TokenType::Not), Some(TokenType::Equals)) => {
                let negated = Token {
                    tok_type: TokenType::NotEqual,
                    tok_value: None,
                };
                new_tokens.pop();
                new_tokens.push(Some(negated));
            }
            (_, _, _) => {
                new_tokens.push(Some(tokens[z].clone()));
            }
        }
        z += 1;
        window = (
            window.1,
            window.2,
            match tokens.get_mut(z) {
                Some(token) => Some(token.tok_type),
                None => None,
            },
        )
    }
    let final_tokens = new_tokens
        .into_iter()
        .filter(|t| t.is_some())
        .flatten()
        .collect();

    return final_tokens;
}

pub fn precedence(tok: &Token) -> u8 {
    return match tok.tok_type {
        //multiplication
        TokenType::Mult => 255,
        TokenType::Div => 255,
        TokenType::Mod => 255,
        //addition
        TokenType::Plus => 253,
        TokenType::Minus => 253,
        //equality
        TokenType::In => 252,
        TokenType::Equals => 252,
        //logical
        TokenType::And => 251,
        TokenType::Or => 251,

        TokenType::GreaterThan => 250,
        TokenType::LessThan => 250,
        TokenType::Gte => 250,
        TokenType::Lte => 250,
        TokenType::Equals => 250,
        _ => 0,
    };
}

pub fn expr_type_factory(mut itr: vec::IntoIter<Token>) -> Vec<Token> {
    let mut token = itr.next();
    let mut new_tokens = Vec::<Token>::new();
    loop {
        if token.is_none() {
            break;
        }
        match token.clone().unwrap().tok_type {
            TokenType::OpenParen => {
                let sub_expr = expr_type_factory(itr.clone());
                let mut postfix = infix_to_postfix(sub_expr);
                //TODO: resolve all variables here;
                let x = eval_expression(&mut postfix);
                new_tokens.push(x);
            }
            TokenType::CloseParen => {
                break;
            }
            TokenType::OpenBracket => {}
            TokenType::CloseBracket => {}
            TokenType::Literal
            | TokenType::And
            | TokenType::Or
            | TokenType::Plus
            | TokenType::Minus
            | TokenType::Mult
            | TokenType::Div
            | TokenType::Mod
            | TokenType::Not => new_tokens.push(token.clone().unwrap()),
            _ => panic!("Invalid token in expression"),
        }
        token = itr.next();
    }
    return new_tokens;
}

pub fn infix_to_postfix(tokens: Vec<Token>) -> Vec<Token> {
    let mut operator_stack = LinkedList::<Token>::new();
    let mut operand_queue = Vec::<Token>::new();

    for token in tokens {
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

pub fn eval_expression(postfix_expr: &mut Vec<Token>) -> Token {
    if postfix_expr.len() as i32 == 1 {
        return postfix_expr[0].clone();
    }

    let mut calc_stack = LinkedList::<Token>::new();
    let mut iter = postfix_expr.into_iter();
    let mut token_option = iter.next();

    println!("loop VV");
    loop {
        let token = token_option.unwrap();
        if precedence(&token.clone()) == 0 {
            calc_stack.push_front(token.clone())
        } else {
            let right = calc_stack.pop_front().unwrap();
            let left = calc_stack.pop_front().unwrap();
            let result_token = operate(left, right, token.clone());
            println!(
                "result:{}",
                result_token.clone().tok_value.unwrap().s_val.unwrap()
            );
            calc_stack.push_front(result_token.clone());
        }
        token_option = iter.next();
        if calc_stack.len() == 1 && token_option.is_none() {
            break;
        }
    }
    let ret_val = calc_stack.front().unwrap();
    println!(
        "returns {}",
        &ret_val.clone().tok_value.unwrap().s_val.unwrap()
    );
    return ret_val.clone();
}
