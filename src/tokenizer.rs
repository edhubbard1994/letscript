use regex::Regex;
use std::str::Chars;

#[derive(Clone, Debug)]
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
    s_val: Option<String>,
}

#[derive(Clone)]
pub struct Token {
    tok_type: TokenType,
    tok_value: Option<TokenValue>,
}

pub fn tokenize(program_string: &mut String) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();

    let var_regx = Regex::new(r"var\z").unwrap();
    let is_regex = Regex::new(r"is\z").unwrap();
    let and_regex = Regex::new(r"and\z").unwrap();
    let or_regex = Regex::new(r"or\z").unwrap();
    let gte_regex = Regex::new(r">=\z").unwrap();
    let lte_regex = Regex::new(r"<=\z").unwrap();
    let not_regex = Regex::new(r"not\z").unwrap();
    let for_regex = Regex::new(r"for\z").unwrap();
    let while_regex = Regex::new(r"while\z").unwrap();
    let each_regex = Regex::new(r"each\z").unwrap();
    let loop_regex = Regex::new(r"loop\z").unwrap();

    let mut text_itr = program_string.chars();
    let mut current_char;
    current_char = text_itr.next();
    loop {
        let mut token;

        if current_char == None {
            break;
        }
        println!("current char in match: {}", current_char.as_ref().unwrap());
        match current_char.unwrap() {
            'v' => {
                let token_tuple = generate_regex_token(
                    &mut text_itr,
                    &var_regx,
                    &mut current_char.unwrap(),
                    TokenType::Assign,
                );
                token = token_tuple.0;
                current_char = Some(token_tuple.1);
            }
            'i' => {
                let token_tuple = generate_regex_token(
                    &mut text_itr,
                    &is_regex,
                    &mut current_char.unwrap(),
                    TokenType::Is,
                );
                token = token_tuple.0;
                current_char = Some(token_tuple.1);
            }
            'a' => {
                let token_tuple = generate_regex_token(
                    &mut text_itr,
                    &and_regex,
                    &mut current_char.unwrap(),
                    TokenType::And,
                );
                token = token_tuple.0;
                current_char = Some(token_tuple.1);
            }
            'o' => {
                let token_tuple = generate_regex_token(
                    &mut text_itr,
                    &or_regex,
                    &mut current_char.unwrap(),
                    TokenType::Or,
                );
                token = token_tuple.0;
                current_char = Some(token_tuple.1);
            }
            'n' => {
                let token_tuple = generate_regex_token(
                    &mut text_itr,
                    &not_regex,
                    &mut current_char.unwrap(),
                    TokenType::Not,
                );
                token = token_tuple.0;
                current_char = Some(token_tuple.1);
            }
            'f' => {
                let token_tuple = generate_regex_token(
                    &mut text_itr,
                    &for_regex,
                    &mut current_char.unwrap(),
                    TokenType::For,
                );
                token = token_tuple.0;
                current_char = Some(token_tuple.1);
            }
            'e' => {
                let token_tuple = generate_regex_token(
                    &mut text_itr,
                    &each_regex,
                    &mut current_char.unwrap(),
                    TokenType::Each,
                );
                token = token_tuple.0;
                current_char = Some(token_tuple.1);
            }
            'l' => {
                let token_tuple = generate_regex_token(
                    &mut text_itr,
                    &loop_regex,
                    &mut current_char.unwrap(),
                    TokenType::Loop,
                );
                token = token_tuple.0;
                current_char = Some(token_tuple.1);
            }
            'w' => {
                let token_tuple = generate_regex_token(
                    &mut text_itr,
                    &while_regex,
                    &mut current_char.unwrap(),
                    TokenType::While,
                );
                token = token_tuple.0;
                current_char = Some(token_tuple.1);
            }
            '=' => {
                token = generate_simple_token(TokenType::Equals);
                current_char = text_itr.next();
            }
            '+' => {
                token = generate_simple_token(TokenType::Plus);
                current_char = text_itr.next();
            }
            '-' => {
                token = generate_simple_token(TokenType::Minus);
                current_char = text_itr.next();
            }
            '*' => {
                token = generate_simple_token(TokenType::Mult);
                current_char = text_itr.next();
            }
            '/' => {
                token = generate_simple_token(TokenType::Div);
                current_char = text_itr.next();
            }
            '>' => {
                let token_tuple = generate_operator_regex_token(
                    &mut text_itr,
                    &gte_regex,
                    &mut current_char.unwrap(),
                    TokenType::Gte,
                );
                token = token_tuple.0;
                current_char = Some(token_tuple.1);
                if token.is_none() {
                    token = generate_simple_token(TokenType::GreaterThan);
                    current_char = text_itr.next();
                }
            }
            '<' => {
                let token_tuple = generate_operator_regex_token(
                    &mut text_itr,
                    &lte_regex,
                    &mut current_char.unwrap(),
                    TokenType::Lte,
                );
                token = token_tuple.0;
                current_char = Some(token_tuple.1);
                if token.is_none() {
                    token = generate_simple_token(TokenType::LessThan);
                    current_char = text_itr.next();
                }
            }
            '{' => {
                token = generate_simple_token(TokenType::OpenBrace);
                current_char = text_itr.next();
            }

            '}' => {
                token = generate_simple_token(TokenType::CloseBrace);
                current_char = text_itr.next();
            }

            '[' => {
                token = generate_simple_token(TokenType::OpenBracket);
                current_char = text_itr.next();
            }

            ']' => {
                token = generate_simple_token(TokenType::CloseBracet);
                current_char = text_itr.next();
            }

            '(' => {
                token = generate_simple_token(TokenType::OpenParen);
                current_char = text_itr.next();
            }

            ')' => {
                token = generate_simple_token(TokenType::CloseParen);
                current_char = text_itr.next();
            }
            '\n' => {
                token = generate_simple_token(TokenType::NewLine);
                current_char = text_itr.next();
            }
            ',' => {
                token = generate_simple_token(TokenType::Comma);
                current_char = text_itr.next();
            }
            ':' => {
                token = generate_simple_token(TokenType::Colon);
                current_char = text_itr.next();
            }
            '"' => {
                token = generate_simple_token(TokenType::Quote);
                current_char = text_itr.next();
            }
            ' ' => {
                token = None;
                current_char = text_itr.next();
            }
            '0'..='9' => {
                let mut acc = String::from("");
                while current_char != None
                    && (current_char.unwrap().is_numeric() || current_char.unwrap() == '.')
                {
                    acc.push(current_char.unwrap().clone());
                    current_char = text_itr.next();
                }
                token = generate_literal_token(acc);
            }
            _ => {
                let mut acc = String::from("");

                loop {
                    println!("acc: <{}>", acc);
                    println!("curr: <{}>", current_char.as_ref().unwrap());
                    if current_char == None
                        || current_char.unwrap().is_ascii_punctuation()
                        || current_char.unwrap().is_whitespace()
                        || current_char.unwrap().is_alphanumeric() != true
                    {
                        break;
                    }
                    acc.push(current_char.unwrap().clone());
                    current_char = text_itr.next();
                }
                token = generate_literal_token(acc);
            }
        }
        match token {
            Some(t) => tokens.push(t),
            None => {}
        }

        // println!("next char: <{}>", current_char.as_ref().unwrap());
    }
    tokens
}

fn generate_simple_token(tok_type: TokenType) -> Option<Token> {
    Some(Token {
        tok_type: tok_type,
        tok_value: None,
    })
}

fn generate_regex_token(
    stream: &mut Chars,
    regex: &Regex,
    current: &mut char,
    tok_type: TokenType,
) -> (Option<Token>, char) {
    let mut acc = String::from("");

    while *current != ' ' && current.is_alphanumeric() {
        acc.push(current.clone());
        println!("loop char regex = {}", current.clone());
        *current = stream.next().unwrap();
    }
    println!("regex text: {}", acc.clone().as_str());

    if regex.is_match(acc.as_str()) {
        return (
            Some(Token {
                tok_type: tok_type,
                tok_value: None,
            }),
            *current,
        );
    }

    return (generate_literal_token(acc), *current);
}

fn generate_operator_regex_token(
    stream: &mut Chars,
    regex: &Regex,
    current: &mut char,
    tok_type: TokenType,
) -> (Option<Token>, char) {
    let mut acc = String::from("");
    while *current != ' ' && current.is_ascii_punctuation() {
        acc.push(current.clone());
        //println!("loop char = {}", current.clone());
        *current = stream.next().unwrap();
    }
    if regex.is_match(acc.as_str()) {
        return (
            Some(Token {
                tok_type: tok_type,
                tok_value: None,
            }),
            *current,
        );
    }
    if acc == "<" {
        return (generate_simple_token(TokenType::LessThan), *current);
    } else if acc == ">" {
        return (generate_simple_token(TokenType::GreaterThan), *current);
    }
    return (generate_literal_token(acc), *current);
}

fn generate_literal_token(from: String) -> Option<Token> {
    // let int_regex = Regex::new(r"\d+\z").unwrap();
    // let float_regex = Regex::new(r"\d+\.\d+\z").unwrap();
    return Some(Token {
        tok_type: TokenType::Literal,
        tok_value: Some(TokenValue {
            s_val: Some(from.clone()),
        }),
    });
}

#[test]
pub fn test_regex() {
    let input_string = String::from("var x = 12");
    let mut itr = input_string.chars();
    let mut current = itr.next().unwrap();
    let token = generate_regex_token(
        &mut itr,
        &mut Regex::new(r"var\z").unwrap(),
        &mut current,
        TokenType::Assign,
    )
    .0;
    assert_eq!(token.is_some(), true);
    assert_eq!(matches!(token.unwrap().tok_type, TokenType::Assign), true);
}

#[test]
pub fn test_simple() {
    let token = generate_simple_token(TokenType::Equals);
    assert_eq!(token.is_some(), true);
    assert_eq!(matches!(token.unwrap().tok_type, TokenType::Equals), true);
}

#[test]
pub fn test_literal() {
    let input_string = String::from("hello world");
    let mut token = generate_literal_token(input_string);
    assert_eq!(token.is_some(), true);
    assert_eq!(matches!(token.unwrap().tok_type, TokenType::Literal), true);
    let input_int = String::from("1");
    token = generate_literal_token(input_int);
    assert_eq!(token.is_some(), true);
    assert_eq!(matches!(token.unwrap().tok_type, TokenType::Literal), true);
    let input_float = String::from("0.2394");
    token = generate_literal_token(input_float);
    assert_eq!(token.is_some(), true);
    assert_eq!(matches!(token.unwrap().tok_type, TokenType::Literal), true);
}

#[test]
pub fn test_tokenizer_assignment() {
    let mut input = String::from(" var x= 12 \n");
    let tokens = tokenize(&mut input);
    tokens.iter().for_each(|t| println!("{:?}", t.tok_type));
    assert_eq!(matches!(tokens[0].tok_type, TokenType::Assign), true);
    assert_eq!(matches!(tokens[1].tok_type, TokenType::Literal), true);
    assert_eq!(matches!(tokens[2].tok_type, TokenType::Equals), true);
    assert_eq!(matches!(tokens[3].tok_type, TokenType::Literal), true);
    assert_eq!(matches!(tokens[4].tok_type, TokenType::NewLine), true);
    assert_eq!(tokens.len(), 5);
}

#[test]
pub fn test_tokenizer_combinator() {
    let mut input = String::from("1+ 4 -3.5 *97 / 4 \n");
    let tokens = tokenize(&mut input);
    tokens.iter().for_each(|t| println!("{:?}", t.tok_type));
    assert_eq!(matches!(tokens[0].tok_type, TokenType::Literal), true);
    assert_eq!(matches!(tokens[1].tok_type, TokenType::Plus), true);
    assert_eq!(matches!(tokens[2].tok_type, TokenType::Literal), true);
    assert_eq!(matches!(tokens[3].tok_type, TokenType::Minus), true);
    assert_eq!(matches!(tokens[4].tok_type, TokenType::Literal), true);
    assert_eq!(tokens.len(), 10);
}

#[test]
pub fn test_tokenizer_boolean_expression() {
    let mut input = String::from("3 < 5 or 7 is 2 and 5>=4 or 0.77<= y\n");
    let tokens = tokenize(&mut input);
    tokens.iter().for_each(|t| println!("{:?}", t.tok_type));

    assert_eq!(matches!(tokens[0].tok_type, TokenType::Literal), true);
    assert_eq!(matches!(tokens[3].tok_type, TokenType::Or), true);
    assert_eq!(matches!(tokens[5].tok_type, TokenType::Is), true);
    assert_eq!(matches!(tokens[7].tok_type, TokenType::And), true);
    assert_eq!(matches!(tokens[9].tok_type, TokenType::Gte), true);
    assert_eq!(tokens.len(), 16);
}

#[test]
pub fn test_tokenizer_bracket() {
    //TODO : DEBUG INF LOOP IN STRING
    let mut input = String::from("[4, 3] {\"hello\": \"worlds\"} ((x + 4)- 19)\n");
    let tokens = tokenize(&mut input);
    tokens.iter().for_each(|t| println!("{:?}", t.tok_type));
    assert_eq!(matches!(tokens[0].tok_type, TokenType::OpenBracket), true);
    assert_eq!(matches!(tokens[4].tok_type, TokenType::CloseBracet), true);
    assert_eq!(matches!(tokens[5].tok_type, TokenType::OpenBrace), true);
    assert_eq!(matches!(tokens[13].tok_type, TokenType::CloseBrace), true);
    assert_eq!(matches!(tokens[14].tok_type, TokenType::OpenParen), true);
    assert_eq!(matches!(tokens[15].tok_type, TokenType::OpenParen), true);
    assert_eq!(matches!(tokens[19].tok_type, TokenType::CloseParen), true);
    assert_eq!(matches!(tokens[22].tok_type, TokenType::CloseParen), true);
    assert_eq!(tokens.len(), 24);
}

#[test]
pub fn test_tokenizer_loops() {
    let mut input = String::from("for each(var x in y) {} loop while (x not = 3)\n");
    let tokens = tokenize(&mut input);
    tokens.iter().for_each(|t| println!("{:?}", t.tok_type));
    assert_eq!(matches!(tokens[0].tok_type, TokenType::For), true);
    assert_eq!(matches!(tokens[1].tok_type, TokenType::Each), true);
    assert_eq!(matches!(tokens[2].tok_type, TokenType::OpenParen), true);
    assert_eq!(matches!(tokens[7].tok_type, TokenType::CloseParen), true);
    assert_eq!(matches!(tokens[9].tok_type, TokenType::CloseBrace), true);
    assert_eq!(tokens.len(), 19);
}

#[test]
pub fn test_tokenizer_functions() {
    let mut input = String::from("for each(var x in y) {} loop while (x not = 3)\n");
    let tokens = tokenize(&mut input);
    tokens.iter().for_each(|t| println!("{:?}", t.tok_type));
    assert_eq!(matches!(tokens[0].tok_type, TokenType::For), true);
    assert_eq!(matches!(tokens[1].tok_type, TokenType::Each), true);
    assert_eq!(matches!(tokens[2].tok_type, TokenType::OpenParen), true);
    assert_eq!(matches!(tokens[7].tok_type, TokenType::CloseParen), true);
    assert_eq!(matches!(tokens[9].tok_type, TokenType::CloseBrace), true);
    assert_eq!(tokens.len(), 19);
}
