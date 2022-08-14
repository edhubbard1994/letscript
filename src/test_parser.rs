use crate::parser::collect_expression_tokens;
use crate::tokenizer::tokenize;

#[test]
pub fn test_collect_expressions() {
    let mut input = String::from("3+ 4 + (6- 5)\n");
    let tokens = tokenize(&mut input);
    let current = tokens.iter().next().unwrap();
    let collected = collect_expression_tokens(&mut *current, &mut tokens.iter());
}
