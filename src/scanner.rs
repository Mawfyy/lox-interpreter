use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub enum TokenType {
    String,
    Number,
    Semicolon,
    LeftParen,
    Slash,
    RightParen,
    Comma,
    Plus,
    LeftBrace,
    RightBrace,
    Dot,
    Minus,
    Star,
    Equal,
    EqualEqual,
    BangEqual,
    Bang,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Identifer,
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

#[derive(Debug)]
pub enum Errors {
    SyntaxError(String),
    FileNotFound,
}

#[derive(Debug)]
pub struct Token {
    t_type: TokenType,
    value: String,
    line: usize,
}

impl Token {
    pub fn new(t_type: TokenType, value: String, line: usize) -> Self {
        Token {
            t_type,
            value,
            line,
        }
    }
}

pub fn scan_tokens(file_content: String) -> Result<Vec<Token>, Errors> {
    let mut current_character_index = 0;
    let mut line = 1;
    let mut tokens: Vec<Token> = Vec::new();
    let mut errors_scan: Result<Errors, ()> = Err(());

    let keywords: HashMap<&str, TokenType> = HashMap::from([
        ("and", TokenType::And),
        ("class", TokenType::Class),
        ("else", TokenType::Else),
        ("false", TokenType::False),
        ("for", TokenType::For),
        ("fun", TokenType::Fun),
        ("if", TokenType::If),
        ("nil", TokenType::Nil),
        ("or", TokenType::Or),
        ("print", TokenType::Print),
        ("return", TokenType::Return),
        ("super", TokenType::Super),
        ("this", TokenType::This),
        ("true", TokenType::True),
        ("var", TokenType::Var),
        ("while", TokenType::While),
    ]);

    while current_character_index < file_content.len() {
        let current_char = file_content.chars().nth(current_character_index).unwrap();

        match current_char {
            //Single tokens
            '+' => {
                current_character_index += 1;
                tokens.push(Token::new(TokenType::Plus, current_char.to_string(), line));
            }

            '\n' => {
                current_character_index += 1;
                line += 1
            }

            '(' => {
                current_character_index += 1;
                tokens.push(Token::new(
                    TokenType::LeftParen,
                    current_char.to_string(),
                    line,
                ));
            }

            ')' => {
                current_character_index += 1;
                tokens.push(Token::new(
                    TokenType::RightParen,
                    current_char.to_string(),
                    line,
                ))
            }

            '{' => {
                current_character_index += 1;
                tokens.push(Token::new(
                    TokenType::LeftBrace,
                    current_char.to_string(),
                    line,
                ))
            }

            '}' => {
                current_character_index += 1;
                tokens.push(Token::new(
                    TokenType::RightBrace,
                    current_char.to_string(),
                    line,
                ))
            }

            '*' => {
                current_character_index += 1;
                tokens.push(Token::new(TokenType::Star, current_char.to_string(), line))
            }

            ';' => {
                current_character_index += 1;
                tokens.push(Token::new(
                    TokenType::Semicolon,
                    current_char.to_string(),
                    line,
                ))
            }

            '-' => {
                current_character_index += 1;
                tokens.push(Token::new(TokenType::Minus, current_char.to_string(), line))
            }

            ',' => {
                current_character_index += 1;
                tokens.push(Token::new(TokenType::Comma, current_char.to_string(), line))
            }

            '.' => {
                current_character_index += 1;
                tokens.push(Token::new(TokenType::Dot, current_char.to_string(), line))
            }

            //Operators logical tokens
            '=' => {
                if file_content
                    .chars()
                    .nth(current_character_index + 1)
                    .is_some_and(|char| char == '=')
                {
                    let equals_operator = file_content
                        .chars()
                        .skip(current_character_index)
                        .take(2)
                        .collect::<String>();
                    tokens.push(Token::new(TokenType::EqualEqual, equals_operator, line));
                    current_character_index += 2;
                } else {
                    current_character_index += 1;
                    tokens.push(Token::new(TokenType::Equal, current_char.to_string(), line))
                }
            }

            '!' => {
                if file_content
                    .chars()
                    .nth(current_character_index + 1)
                    .is_some_and(|char| char == '=')
                {
                    let equals_operator = file_content
                        .chars()
                        .skip(current_character_index)
                        .take(2)
                        .collect::<String>();
                    tokens.push(Token::new(TokenType::BangEqual, equals_operator, line));
                    current_character_index += 2;
                } else {
                    current_character_index += 1;
                    tokens.push(Token::new(TokenType::Bang, current_char.to_string(), line))
                }
            }

            '<' => {
                if file_content
                    .chars()
                    .nth(current_character_index + 1)
                    .is_some_and(|char| char == '=')
                {
                    let equals_operator = file_content
                        .chars()
                        .skip(current_character_index)
                        .take(2)
                        .collect::<String>();
                    tokens.push(Token::new(TokenType::LessEqual, equals_operator, line));
                    current_character_index += 2;
                } else {
                    current_character_index += 1;
                    tokens.push(Token::new(TokenType::Less, current_char.to_string(), line))
                }
            }

            '>' => {
                if file_content
                    .chars()
                    .nth(current_character_index + 1)
                    .is_some_and(|char| char == '=')
                {
                    let equals_operator = file_content
                        .chars()
                        .skip(current_character_index)
                        .take(2)
                        .collect::<String>();
                    tokens.push(Token::new(TokenType::GreaterEqual, equals_operator, line));
                    current_character_index += 2;
                } else {
                    current_character_index += 1;
                    tokens.push(Token::new(
                        TokenType::Greater,
                        current_char.to_string(),
                        line,
                    ))
                }
            }

            //Comments
            '/' => {
                if file_content
                    .chars()
                    .nth(current_character_index + 1)
                    .is_some_and(|char| char == '/')
                {
                    file_content
                        .chars()
                        .skip(current_character_index)
                        .take_while(|char| char != &'\n')
                        .for_each(|_| {
                            current_character_index += 1;
                        });
                } else {
                    current_character_index += 1;
                    tokens.push(Token::new(TokenType::Slash, current_char.to_string(), line))
                }
            }

            //String
            '"' => {
                let string_content = file_content
                    .chars()
                    .skip(current_character_index + 1)
                    .take_while(|char| char != &'"')
                    .map(|char| {
                        current_character_index += 1;
                        if current_character_index == file_content.len() - 1 {
                            errors_scan = Ok(Errors::SyntaxError(format!(
                                "Unterminated String at line {}",
                                line
                            )));
                        }
                        char
                    })
                    .collect::<String>();

                if errors_scan.is_ok() {
                    //This is because the before loop only iterate until ", so it may match with " end string and iterates all the file.
                    current_character_index += 2;
                    tokens.push(Token::new(TokenType::String, string_content, line));
                }
            }

            '0'..='9' => {
                let mut number_content = file_content
                    .chars()
                    .skip(current_character_index)
                    .take_while(|char| char.is_numeric())
                    .map(|char| {
                        current_character_index += 1;
                        char
                    })
                    .collect::<String>();
                if file_content
                    .chars()
                    .nth(current_character_index + 1)
                    .is_some_and(|char| char.is_numeric())
                {
                    let float_part = file_content
                        .chars()
                        .skip(current_character_index)
                        .take_while(|char| char.is_numeric() || char == &'.')
                        .map(|char| {
                            current_character_index += 1;
                            char
                        })
                        .collect::<String>();
                    number_content.push_str(float_part.as_str());
                }

                tokens.push(Token::new(TokenType::Number, number_content, line))
            }

            'a'..='z' | 'A'..='Z' | '_' => {
                let word = file_content
                    .chars()
                    .skip(current_character_index)
                    .take_while(|char| char.is_ascii_alphabetic() || char == &'_')
                    .map(|char| {
                        current_character_index += 1;
                        char
                    })
                    .collect::<String>();
                if let Some(keyword) = keywords.get(word.as_str()).take() {
                    tokens.push(Token::new(*keyword, word, line));
                } else {
                    tokens.push(Token::new(TokenType::Identifer, word, line))
                }
            }

            ' ' => {
                current_character_index += 1;
            }

            char => {
                return Err(Errors::SyntaxError(format!(
                    "Unsupported token {char} at line {line}"
                )));
            }
        }
    }

    if let Ok(error) = errors_scan {
        return Err(error);
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_tokens() {
        let source_code = String::from("var a = 1;\nwhile (a < 10) { print a; a = a + 1;}");
        let tokens = scan_tokens(source_code);
        assert_eq!(tokens.is_ok(), true);
    }

    #[test]
    fn it_should_return_error() {
        let source_code = String::from("var a = 1;\n var b = º;");
        let tokens = scan_tokens(source_code);
        assert_eq!(tokens.is_err(), true);
    }

    #[test]
    fn it_should_return_error_string() {
        let source_code = String::from("");
        let tokens = scan_tokens(source_code);
        assert_eq!(tokens.is_err(), true);
    }
}
