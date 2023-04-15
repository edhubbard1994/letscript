use crate::parser::collect_expression_tokens;
use crate::parser::eval_expression;
use crate::parser::infix_to_postfix;
use crate::parser::resolve_unary_operators;
use crate::token::TokenType;
use crate::tokenizer::tokenize;

#[test]
pub fn test_collect_expressions() {
    let mut input = String::from("3+ 4 + (6- 5) \n");
    let tokens = tokenize(&mut input);
    let current = tokens.iter().next().unwrap();
    let collected = collect_expression_tokens(current, &mut tokens.iter());
    assert_eq!(collected.1.len(), 9);
}

#[test]
pub fn test_resolve_unary_operators() {
    let mut input = String::from("3 + -5");
    let tokens = tokenize(&mut input);
    let result = resolve_unary_operators(tokens);
    assert_eq!(result.len(), 3);
    assert_eq!(result[1].tok_type, TokenType::Plus)
}

#[test]
pub fn test_resolve_unary_operators_1() {
    let mut input = String::from("not true");
    let tokens = tokenize(&mut input);
    let result = resolve_unary_operators(tokens);
    
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].tok_type, TokenType::Literal);
    assert_eq!(result[0].clone().tok_value.unwrap().s_val.unwrap(), "false");
}

#[test]
pub fn test_resolve_unary_operators_2() {
    let mut input = String::from("-5");
    let tokens = tokenize(&mut input);
    let result = resolve_unary_operators(tokens);
    assert_eq!("-5", result[0].clone().tok_value.unwrap().s_val.unwrap());
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].tok_type, TokenType::Literal);
    
}

#[test]
pub fn test_resolve_unary_operators_3() {
    let mut input = String::from("- 2");
    let tokens = tokenize(&mut input);
    let result = resolve_unary_operators(tokens);
    assert_eq!("-2", result[0].clone().tok_value.unwrap().s_val.unwrap());
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].tok_type, TokenType::Literal);
    
}

pub fn test_resolve_unary_operators_4() {
    let mut input = String::from(" x not= 2");
    let tokens = tokenize(&mut input);
    let result = resolve_unary_operators(tokens);
    assert_eq!(result.len(), 3);
    assert_eq!(result[1].tok_type, TokenType::NotEqual);
    
}




#[test]
pub fn test_infix_to_postfix_1() {
    let mut input = String::from("5+ 7 or 3 * 5");
    let tokens = tokenize(&mut input);
    let postfix = infix_to_postfix(tokens);

    postfix.iter().for_each(|t| {
        if t.tok_type == TokenType::Literal {
            println!("Token({})", t.tok_value.clone().unwrap().s_val.unwrap())
        } else {
            println!("Token({:?})", t.tok_type);
        }
    });
    assert_eq!(postfix.len(), 7);
}

#[test]
pub fn test_infix_to_postfix_2() {
    let mut input = String::from("(3 - 5) * 12");
    let tokens = tokenize(&mut input);
    let postfix = infix_to_postfix(tokens);

    postfix.iter().for_each(|t| {
        if t.tok_type == TokenType::Literal {
            println!("Token({})", t.tok_value.clone().unwrap().s_val.unwrap())
        } else {
            println!("Token({:?})", t.tok_type);
        }
    });
    assert_eq!(postfix.len(), 5);
}

#[test]
pub fn test_eval_postfix_1() {
    let mut input = String::from("5+ 7 or 3 * 5");
    let tokens = tokenize(&mut input);
    let mut postfix = infix_to_postfix(tokens);
    let evaled = eval_expression(&mut postfix);
    postfix.iter().for_each(|t| {
        if t.tok_type == TokenType::Literal {
            println!("Token({})", t.tok_value.clone().unwrap().s_val.unwrap())
        } else {
            println!("Token({:?})", t.tok_type);
        }
    });

    assert_eq!(evaled.tok_value.unwrap().s_val.unwrap(), "true");
}

#[test]
pub fn test_eval_postfix_2() {
    let mut input = String::from("(3 - 5) * 12");
    let tokens = tokenize(&mut input);
    let mut postfix = infix_to_postfix(tokens);
    let evaled = eval_expression(&mut postfix);
    postfix.iter().for_each(|t| {
        if t.tok_type == TokenType::Literal {
            println!("Token({})", t.tok_value.clone().unwrap().s_val.unwrap())
        } else {
            println!("Token({:?})", t.tok_type);
        }
    });

    assert_eq!(evaled.tok_value.unwrap().s_val.unwrap(), "-24");
}

#[test]
pub fn test_eval_postfix_3() {
    let mut input = String::from("(3 * 13) mod 12");
    let tokens = tokenize(&mut input);
    let mut postfix = infix_to_postfix(tokens);
    let evaled = eval_expression(&mut postfix);
    postfix.iter().for_each(|t| {
        if t.tok_type == TokenType::Literal {
            println!("Token({})", t.tok_value.clone().unwrap().s_val.unwrap())
        } else {
            println!("Token({:?})", t.tok_type);
        }
    });

    assert_eq!(evaled.tok_value.unwrap().s_val.unwrap(), "3");
}

#[test]
pub fn test_eval_postfix_4() {
    let mut input = String::from("(42.6 /6) + 1");
    let tokens = tokenize(&mut input);
    let mut postfix = infix_to_postfix(tokens);
    let evaled = eval_expression(&mut postfix);
    postfix.iter().for_each(|t| {
        if t.tok_type == TokenType::Literal {
            println!("Token({})", t.tok_value.clone().unwrap().s_val.unwrap())
        } else {
            println!("Token({:?})", t.tok_type);
        }
    });

    assert_eq!(
        evaled.tok_value.unwrap().s_val.unwrap(),
        "8.100000000000001"
    );
}

#[test]
pub fn test_eval_postfix_5() {
    let mut input = String::from("5 > 7");
    let tokens = tokenize(&mut input);
    let mut postfix = infix_to_postfix(tokens);
    let evaled = eval_expression(&mut postfix);
    postfix.iter().for_each(|t| {
        if t.tok_type == TokenType::Literal {
            println!("Token({})", t.tok_value.clone().unwrap().s_val.unwrap())
        } else {
            println!("Token({:?})", t.tok_type);
        }
    });

    assert_eq!(evaled.tok_value.unwrap().s_val.unwrap(), "false");
}

#[test]
pub fn test_eval_postfix_6() {
    let mut input = String::from("5 >= 5");
    let tokens = tokenize(&mut input);
    let mut postfix = infix_to_postfix(tokens);
    let evaled = eval_expression(&mut postfix);
    postfix.iter().for_each(|t| {
        if t.tok_type == TokenType::Literal {
            println!("Token({})", t.tok_value.clone().unwrap().s_val.unwrap())
        } else {
            println!("Token({:?})", t.tok_type);
        }
    });

    assert_eq!(evaled.tok_value.unwrap().s_val.unwrap(), "true");
}

#[test]
pub fn test_eval_postfix_7() {
    let mut input = String::from("5 = 5 and 3 = 2");
    let tokens = tokenize(&mut input);
    let mut postfix = infix_to_postfix(tokens);
    let evaled = eval_expression(&mut postfix);
    postfix.iter().for_each(|t| {
        if t.tok_type == TokenType::Literal {
            println!("Token({})", t.tok_value.clone().unwrap().s_val.unwrap())
        } else {
            println!("Token({:?})", t.tok_type);
        }
    });

    assert_eq!(evaled.tok_value.unwrap().s_val.unwrap(), "false");
}

#[test]
pub fn test_eval_postfix_8() {
    let mut input = String::from("((1 +4)  = 5) and 2 = 2");
    let tokens = tokenize(&mut input);
    let mut postfix = infix_to_postfix(tokens);
    let evaled = eval_expression(&mut postfix);
    postfix.iter().for_each(|t| {
        if t.tok_type == TokenType::Literal {
            println!("Token({})", t.tok_value.clone().unwrap().s_val.unwrap())
        } else {
            println!("Token({:?})", t.tok_type);
        }
    });

    assert_eq!(evaled.tok_value.unwrap().s_val.unwrap(), "true");
}
