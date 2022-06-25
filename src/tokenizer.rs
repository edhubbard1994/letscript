use regex::{internal::Char, Regex};
use std::{collections::HashMap, str::Chars, vec};

#[derive(Clone, Debug)]
pub enum TokenType {
    ASSIGN,
    EQUALS,
    PLUS,
    MINUS,
    MULT,
    DIV,
    NOT,
    AND,
    OR,
    IS,
    GREATER_THAN,
    LESS_THAN,
    GTE,
    LTE,
    INTEGER,
    FLOAT,
    STRING,
    BOOL,
    QUOTE,
    IF,
    ELSE,
    WHILE,
    FOR,
    PERIOD,
    COLON,
    COMMA,
    OBJECT,
    OPEN_BRACE,
    CLOSE_BRACE,
    OPEN_BRACKET,
    CLOSE_BRACET,
    OPEN_PAREN,
    CLOSE_PAREN,
    NEW_LINE,
}
#[derive(Clone)]
pub struct TokenValue {
    iVal: Option<i32>,
    fVal: Option<f64>,
    sVal: Option<String>,
    bVal: Option<bool>,
}

#[derive(Clone)]
pub struct Token {
    tokType: TokenType,
    tokValue: Option<TokenValue>,
}

pub fn tokenize(program_string: &mut String) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();

    let mut var_regx = Regex::new(r"var\z").unwrap();
    let mut is_regex = Regex::new(r"is\z").unwrap();
    let mut and_regex = Regex::new(r"and\z").unwrap();
    let mut or_regex = Regex::new(r"or\z").unwrap();
    let mut gte_regex = Regex::new(r">=\z").unwrap();
    let mut lte_regex = Regex::new(r"<=\z").unwrap();
    let mut not_regex = Regex::new(r"not\z").unwrap();

    let mut text_itr = program_string.chars();
    let mut current_char;
    current_char = text_itr.next();
    loop {
        let mut token;

        if current_char == None {
            break;
        }
        println!("{}", current_char.as_ref().unwrap());
        match current_char.unwrap() {
            'v' => {
                token = generate_regex_token(
                    &mut text_itr,
                    &var_regx,
                    current_char.unwrap(),
                    TokenType::ASSIGN,
                );
            }
            'i' => {
                token = generate_regex_token(
                    &mut text_itr,
                    &is_regex,
                    current_char.unwrap(),
                    TokenType::IS,
                );
            }
            'a' => {
                token = generate_regex_token(
                    &mut text_itr,
                    &and_regex,
                    current_char.unwrap(),
                    TokenType::AND,
                );
            }
            'o' => {
                token = generate_regex_token(
                    &mut text_itr,
                    &or_regex,
                    current_char.unwrap(),
                    TokenType::OR,
                );
            }
            '=' => {
                token = generate_simple_token(current_char.unwrap(), TokenType::EQUALS);
            }
            '+' => {
                token = generate_simple_token(current_char.unwrap(), TokenType::PLUS);
            }
            '-' => {
                token = generate_simple_token(current_char.unwrap(), TokenType::MINUS);
            }
            '*' => {
                token = generate_simple_token(current_char.unwrap(), TokenType::MULT);
            }
            '/' => {
                token = generate_simple_token(current_char.unwrap(), TokenType::DIV);
            }
            '>' => {
                token = generate_regex_token(
                    &mut text_itr,
                    &gte_regex,
                    current_char.unwrap(),
                    TokenType::GTE,
                );
                if token.is_none() {
                    token = generate_simple_token(current_char.unwrap(), TokenType::GREATER_THAN);
                }
            }
            '<' => {
                token = generate_regex_token(
                    &mut text_itr,
                    &lte_regex,
                    current_char.unwrap(),
                    TokenType::LTE,
                );
                if token.is_none() {
                    token = generate_simple_token(current_char.unwrap(), TokenType::LESS_THAN);
                }
            }
            '{' => {
                token = generate_simple_token(current_char.unwrap(), TokenType::OPEN_BRACE);
            }

            '}' => {
                token = generate_simple_token(current_char.unwrap(), TokenType::CLOSE_BRACE);
            }

            '[' => {
                token = generate_simple_token(current_char.unwrap(), TokenType::OPEN_BRACKET);
            }

            ']' => {
                token = generate_simple_token(current_char.unwrap(), TokenType::CLOSE_BRACET);
            }

            '(' => {
                token = generate_simple_token(current_char.unwrap(), TokenType::OPEN_PAREN);
            }

            ')' => {
                token = generate_simple_token(current_char.unwrap(), TokenType::CLOSE_PAREN);
            }
            '\n' => {
                token = generate_simple_token(current_char.unwrap(), TokenType::NEW_LINE);
            }
            ',' => {
                token = generate_simple_token(current_char.unwrap(), TokenType::COMMA);
            }
            ':' => {
                token = generate_simple_token(current_char.unwrap(), TokenType::COLON);
            }
            '"' => {
                token = generate_simple_token(current_char.unwrap(), TokenType::QUOTE);
                current_char = text_itr.next();
                tokens.push(token.clone().unwrap());
                let mut acc = String::from("");
                while current_char.unwrap() != '"' {
                    acc.push(current_char.unwrap().clone());
                    current_char = text_itr.next();
                }
                token = generate_literal_token(acc.clone());
                tokens.push(token.clone().unwrap());
                if current_char.unwrap() == '"' {
                    token = generate_simple_token(current_char.unwrap(), TokenType::QUOTE);
                }
            }
            ' ' => {
                token = None;
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
                match token {
                    Some(t) => tokens.push(t),
                    None => {}
                }
                continue;
            }
            _ => {
                let mut acc = String::from("");

                loop {
                    println!("acc: <{}>", acc);
                    println!("curr: <{}>", current_char.as_ref().unwrap());
                    if (current_char == None
                        || current_char.unwrap().is_ascii_punctuation()
                        || current_char.unwrap().is_whitespace()
                        || current_char.unwrap().is_alphanumeric() != true)
                    {
                        break;
                    }
                    acc.push(current_char.unwrap().clone());
                    current_char = text_itr.next();
                }
                token = generate_literal_token(acc);
                match token {
                    Some(t) => tokens.push(t),
                    None => {}
                }
                continue;
            }
        }
        match token {
            Some(t) => tokens.push(t),
            None => {}
        }
        current_char = text_itr.next();
        // println!("next char: <{}>", current_char.as_ref().unwrap());
    }
    tokens
}

fn check_end_of_str(current: &char) -> bool {
    return current.is_ascii_punctuation() == true || current.is_ascii_whitespace() == true;
}

fn generate_simple_token(current: char, tok_type: TokenType) -> Option<Token> {
    Some(Token {
        tokType: tok_type,
        tokValue: None,
    })
}

fn generate_regex_token(
    stream: &mut Chars,
    regex: &Regex,
    mut current: char,
    tok_type: TokenType,
) -> Option<Token> {
    let mut acc = String::from("");
    while current != ' ' {
        acc.push(current.clone());
        current = stream.next().unwrap();
    }
    if regex.is_match(acc.as_str()) {
        return Some(Token {
            tokType: tok_type,
            tokValue: None,
        });
    }
    if acc == "<" {
        return generate_simple_token('<', TokenType::LESS_THAN);
    } else if acc == ">" {
        return generate_simple_token('>', TokenType::GREATER_THAN);
    }
    return generate_literal_token(acc);
}

fn generate_literal_token(from: String) -> Option<Token> {
    let int_regex = Regex::new(r"\d+\z").unwrap();
    let float_regex = Regex::new(r"\d+\.\d+\z").unwrap();
    if float_regex.is_match(&from) {
        let float_val = from.parse::<f64>().unwrap();
        return Some(Token {
            tokType: TokenType::FLOAT,
            tokValue: Some(TokenValue {
                sVal: None,
                fVal: Some(float_val.clone()),
                iVal: None,
                bVal: None,
            }),
        });
    } else if int_regex.is_match(&from) {
        let int_val = from.parse::<i32>().unwrap();
        return Some(Token {
            tokType: TokenType::INTEGER,
            tokValue: Some(TokenValue {
                sVal: None,
                fVal: None,
                iVal: Some(int_val.clone()),
                bVal: None,
            }),
        });
    } else if from == "true" || from == "false" {
        let bool_val = from.parse::<bool>().unwrap();
        return Some(Token {
            tokType: TokenType::BOOL,
            tokValue: Some(TokenValue {
                sVal: None,
                fVal: None,
                iVal: None,
                bVal: Some(bool_val.clone()),
            }),
        });
    } else {
        return Some(Token {
            tokType: TokenType::STRING,
            tokValue: Some(TokenValue {
                sVal: Some(from.clone()),
                fVal: None,
                iVal: None,
                bVal: None,
            }),
        });
    }
}

#[test]
pub fn test_regex() {
    let mut input_string = String::from("var x = 12");
    let mut itr = input_string.chars();
    let mut current = itr.next().unwrap();
    let token = generate_regex_token(
        &mut itr,
        &mut Regex::new(r"var\z").unwrap(),
        current,
        TokenType::ASSIGN,
    );
    assert_eq!(token.is_some(), true);
    assert_eq!(matches!(token.unwrap().tokType, TokenType::ASSIGN), true);
}

#[test]
pub fn test_simple() {
    let mut input_string = String::from("= 12");
    let mut itr = input_string.chars();
    let mut current = itr.next().unwrap();
    let token = generate_simple_token(current, TokenType::EQUALS);
    assert_eq!(token.is_some(), true);
    assert_eq!(matches!(token.unwrap().tokType, TokenType::EQUALS), true);
}

#[test]
pub fn test_literal() {
    let input_string = String::from("hello world");
    let mut token = generate_literal_token(input_string);
    assert_eq!(token.is_some(), true);
    assert_eq!(matches!(token.unwrap().tokType, TokenType::STRING), true);
    let input_int = String::from("1");
    token = generate_literal_token(input_int);
    assert_eq!(token.is_some(), true);
    assert_eq!(matches!(token.unwrap().tokType, TokenType::INTEGER), true);
    let input_float = String::from("0.2394");
    token = generate_literal_token(input_float);
    assert_eq!(token.is_some(), true);
    assert_eq!(matches!(token.unwrap().tokType, TokenType::FLOAT), true);
}

#[test]
pub fn test_tokenizer_assignment() {
    let mut input = String::from(" var x= 12 \n");
    let tokens = tokenize(&mut input);
    tokens.iter().for_each(|t| println!("{:?}", t.tokType));
    assert_eq!(matches!(tokens[0].tokType, TokenType::ASSIGN), true);
    assert_eq!(matches!(tokens[1].tokType, TokenType::STRING), true);
    assert_eq!(matches!(tokens[2].tokType, TokenType::EQUALS), true);
    assert_eq!(matches!(tokens[3].tokType, TokenType::INTEGER), true);
    assert_eq!(matches!(tokens[4].tokType, TokenType::NEW_LINE), true);
    assert_eq!(tokens.len(), 5);
}

#[test]
pub fn test_tokenizer_combinator() {
    let mut input = String::from("1+ 4 -3.5 *97 / 4 \n");
    let tokens = tokenize(&mut input);
    tokens.iter().for_each(|t| println!("{:?}", t.tokType));
    assert_eq!(matches!(tokens[0].tokType, TokenType::INTEGER), true);
    assert_eq!(matches!(tokens[1].tokType, TokenType::PLUS), true);
    assert_eq!(matches!(tokens[2].tokType, TokenType::INTEGER), true);
    assert_eq!(matches!(tokens[3].tokType, TokenType::MINUS), true);
    assert_eq!(matches!(tokens[4].tokType, TokenType::FLOAT), true);
    assert_eq!(tokens.len(), 10);
}

#[test]
pub fn test_tokenizer_boolean_expression() {
    let mut input = String::from("3 < 5 or 7 is 2 and 5>= 4 or 0.77<= y\n");
    let tokens = tokenize(&mut input);
    tokens.iter().for_each(|t| println!("{:?}", t.tokType));
    assert_eq!(matches!(tokens[0].tokType, TokenType::INTEGER), true);
    assert_eq!(matches!(tokens[3].tokType, TokenType::OR), true);
    assert_eq!(matches!(tokens[5].tokType, TokenType::IS), true);
    assert_eq!(matches!(tokens[7].tokType, TokenType::AND), true);
    assert_eq!(matches!(tokens[9].tokType, TokenType::GTE), true);
    assert_eq!(tokens.len(), 16);
}

#[test]
pub fn test_tokenizer_bracket() {
    //TODO : DEBUG INF LOOP IN STRING
    let mut input = String::from("[4, 3] {\"hello\": \"worlds\"} ((x + 4)- 19)\n");
    let tokens = tokenize(&mut input);
    tokens.iter().for_each(|t| println!("{:?}", t.tokType));
    assert_eq!(matches!(tokens[0].tokType, TokenType::OPEN_BRACKET), true);
    assert_eq!(matches!(tokens[4].tokType, TokenType::CLOSE_BRACET), true);
    assert_eq!(matches!(tokens[5].tokType, TokenType::OPEN_BRACE), true);
    assert_eq!(matches!(tokens[13].tokType, TokenType::CLOSE_BRACE), true);
    assert_eq!(matches!(tokens[14].tokType, TokenType::OPEN_PAREN), true);
    assert_eq!(matches!(tokens[15].tokType, TokenType::OPEN_PAREN), true);
    assert_eq!(matches!(tokens[19].tokType, TokenType::CLOSE_PAREN), true);
    assert_eq!(matches!(tokens[22].tokType, TokenType::CLOSE_PAREN), true);
    assert_eq!(tokens.len(), 24);
}
