use crate::parser::collect_expression_tokens;
use crate::parser::infix_to_postfix;
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
pub fn test_infix_to_postfix() {
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
