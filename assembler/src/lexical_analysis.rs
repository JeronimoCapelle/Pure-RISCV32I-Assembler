use crate::{
    structures::ParsingError::{self, TokenizerError},
    structures::Token,
};

pub fn tokenize(contents: &str) -> Result<Vec<Token>, ParsingError> {
    let contents: Vec<char> = contents.chars().collect();

    let mut tokens: Vec<Token> = Vec::new();

    let mut i = 0;

    while i < contents.len() {
        match contents[i] {
            '\n' => {
                tokens.push(Token::NewLine);
                i += 1;
            }
            char if char.is_whitespace() => {
                i += 1;
            }
            '#' => {
                while contents[i] != '\n' {
                    i += 1;
                }
            }
            '/' => {
                if i + 1 < contents.len() && contents[i] == contents[i + 1] {
                    while contents[i] != '\n' {
                        i += 1;
                    }
                }
            }
            ',' => {
                tokens.push(Token::Coma);
                i += 1;
            }
            ':' => {
                tokens.push(Token::Colon);
                i += 1;
            }
            '(' => {
                tokens.push(Token::OpeningParenthesis);
                i += 1;
            }
            ')' => {
                tokens.push(Token::ClosingParenthesis);
                i += 1;
            }

            char if char.is_numeric() || char.eq(&'+') || char.eq(&'-') => {
                let end = contents
                    .iter()
                    .skip(i)
                    .position(|x| {
                        *x == '/'
                            || *x == '#'
                            || *x == '\n'
                            || *x == ' '
                            || *x == '\t'
                            || *x == ':'
                            || *x == ','
                            || *x == '('
                            || *x == ')'
                    })
                    .unwrap_or(contents.len());

                tokens.push(Token::Literal(
                    contents[i..i + end].iter().collect::<String>(),
                ));
                i += end;
            }

            char if char.is_alphabetic() || char.eq(&'_') => {
                let end = contents
                    .iter()
                    .skip(i)
                    .position(|x| {
                        *x == '/'
                            || *x == '#'
                            || *x == '\n'
                            || *x == ' '
                            || *x == '\t'
                            || *x == ':'
                            || *x == ','
                            || *x == '('
                            || *x == ')'
                    })
                    .unwrap_or(contents.len());

                tokens.push(Token::Identifier(
                    contents[i..i + end].iter().collect::<String>(),
                ));
                i += end;
            }
            _ => {
                return Err(TokenizerError);
            }
        };
    }

    Ok(tokens)
}
