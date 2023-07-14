use crate::expr::operate;
use crate::parser::collect_expression_tokens;
use crate::parser::infix_to_postfix;
use crate::parser::precedence;
use crate::token::TokenType;
use crate::tokenizer::tokenize;

#[test]
pub fn test_operation_1() {
    let mut input = String::from("3+4 ");
    let tokens = tokenize(&mut input);
    tokens.iter().for_each(|t| match t.tok_type {
        TokenType::Literal => println!("TOKEN:{:?}", t.clone().tok_value.unwrap().s_val),
        _ => println!("TOKEN:{:?}", t.tok_type),
    });
    let result = operate(tokens[0].clone(), tokens[2].clone(), tokens[1].clone());

    assert_eq!(result.tok_value.unwrap().s_val.unwrap(), "7")
}

#[test]
pub fn test_operation_2() {
    let mut input = String::from("37-17 ");
    let tokens = tokenize(&mut input);
    tokens.iter().for_each(|t| match t.tok_type {
        TokenType::Literal => println!("TOKEN:{:?}", t.clone().tok_value.unwrap().s_val),
        _ => println!("TOKEN:{:?}", t.tok_type),
    });
    let result = operate(tokens[0].clone(), tokens[2].clone(), tokens[1].clone());

    assert_eq!(result.tok_value.unwrap().s_val.unwrap(), "20")
}

#[test]
pub fn test_operation_3() {
    let mut input = String::from("5.1 * 5.0 ");
    let tokens = tokenize(&mut input);
    tokens.iter().for_each(|t| match t.tok_type {
        TokenType::Literal => println!("TOKEN:{:?}", t.clone().tok_value.unwrap().s_val),
        _ => println!("TOKEN:{:?}", t.tok_type),
    });
    let result = operate(tokens[0].clone(), tokens[2].clone(), tokens[1].clone());

    assert_eq!(result.tok_value.unwrap().s_val.unwrap(), "25.5")
}

#[test]
pub fn test_operation_4() {
    let mut input = String::from("8/2 ");
    let tokens = tokenize(&mut input);
    tokens.iter().for_each(|t| match t.tok_type {
        TokenType::Literal => println!("TOKEN:{:?}", t.clone().tok_value.unwrap().s_val),
        _ => println!("TOKEN:{:?}", t.tok_type),
    });
    let result = operate(tokens[0].clone(), tokens[2].clone(), tokens[1].clone());

    assert_eq!(result.tok_value.unwrap().s_val.unwrap(), "4")
}

#[test]
pub fn test_operation_5() {
    let mut input = String::from("8.6/2 ");
    let tokens = tokenize(&mut input);
    tokens.iter().for_each(|t| match t.tok_type {
        TokenType::Literal => println!("TOKEN:{:?}", t.clone().tok_value.unwrap().s_val),
        _ => println!("TOKEN:{:?}", t.tok_type),
    });
    let result = operate(tokens[0].clone(), tokens[2].clone(), tokens[1].clone());

    assert_eq!(result.tok_value.unwrap().s_val.unwrap(), "4.3")
}

#[test]
pub fn test_operation_6() {
    let mut input = String::from("1 and false ");
    let tokens = tokenize(&mut input);
    tokens.iter().for_each(|t| match t.tok_type {
        TokenType::Literal => println!("TOKEN:{:?}", t.clone().tok_value.unwrap().s_val),
        _ => println!("TOKEN:{:?}", t.tok_type),
    });
    let result = operate(tokens[0].clone(), tokens[2].clone(), tokens[1].clone());

    assert_eq!(result.tok_value.unwrap().s_val.unwrap(), "false")
}

#[test]
pub fn test_operation_7() {
    let mut input = String::from("1.1 or false ");
    let tokens = tokenize(&mut input);
    tokens.iter().for_each(|t| match t.tok_type {
        TokenType::Literal => println!("TOKEN:{:?}", t.clone().tok_value.unwrap().s_val),
        _ => println!("TOKEN:{:?}", t.tok_type),
    });
    let result = operate(tokens[0].clone(), tokens[2].clone(), tokens[1].clone());

    assert_eq!(result.tok_value.unwrap().s_val.unwrap(), "true")
}
