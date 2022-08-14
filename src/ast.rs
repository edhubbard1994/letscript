use crate::token::Token;
use crate::token::TokenType;

pub struct LSValue<T> {
    value: T,
}

//let int_regex = Regex::new(r"\d+\z").unwrap();
//let float_regex = Regex::new(r"\d+\.\d+\z").unwrap();

// fn parse_literal(&mut token: Token, &mut tokens: Vec<Token>) -> Expression {
//     let current_expression: Expression;
//    while token && token.tok_type == TokenType::Literal {
//        if !token.token_value {
//             todo!("Return error type")
//         }
//         if float_regex.is_match(token.tok_value) {
//             current_expression = LSValue<f64>();
//             current_expression.value = token.token_value.parse<f64>().unwrap();
//         } else if int_regex.is_match(token.token_value) {
//             current_expression = LSValue<i32>();
//             current_expression.value = token.token_value.parse<i32>().unwrap();
//         } else if token.tok_value == "true" || token.token_value == "false" {
//             current_expression = LSValue<Bool>();
//             current_expression.value = token.token_value.parse<Bool>().unwrap();
//         }
//         else {
//             todo!("Return Parse error Value not recognized")
//         }
//     }
//     current_expression;
// }

fn infix_to_postfix(tokens: &Vec<Token>) {}

pub fn expression(tokens: &Vec<Token>) {}
