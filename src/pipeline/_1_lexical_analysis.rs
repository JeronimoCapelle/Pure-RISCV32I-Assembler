use crate::auxiliar::{
    error::{AssemblerError, Stage::Tokenizer},
    token::Token,
};

pub fn tokenize(contents_str: &str) -> Result<Vec<Token>, AssemblerError> {
    let contents: Vec<char> = contents_str.chars().collect();

    let mut tokens: Vec<Token> = Vec::new();
    let mut line_count = 0;
    let mut i = 0;

    while i < contents.len() {
        match contents[i] {
            '\n' => {
                line_count += 1;
                tokens.push(Token::NewLine(line_count));
                i += 1;
            }
            char if char.is_whitespace() => {
                i += 1;
            }
            '#' => {
                while contents.get(i).unwrap_or(&'\n') != &'\n' {
                    i += 1;
                }
            }
            '/' => {
                if i + 1 < contents.len() && contents[i] == contents[i + 1] {
                    while contents.get(i).unwrap_or(&'\n') != &'\n' {
                        i += 1;
                    }
                }
            }
            ',' => {
                tokens.push(Token::Comma);
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
                    .unwrap_or(contents.len() - i);

                tokens.push(Token::Literal(
                    contents[i..i + end]
                        .iter()
                        .collect::<String>()
                        .trim()
                        .to_string(),
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
                    .unwrap_or(contents.len() - i);

                tokens.push(Token::Identifier(
                    contents[i..i + end]
                        .iter()
                        .collect::<String>()
                        .trim()
                        .to_string(),
                ));
                i += end;
            }
            _ => {
                return Err(AssemblerError::new(Tokenizer, line_count + 1));
            }
        }
    }
    if !matches!(tokens.last(), Some(Token::NewLine(_))) {
        tokens.push(Token::NewLine(line_count + 1));
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use crate::{auxiliar::token::Token, pipeline::_1_lexical_analysis::tokenize};

    #[test]
    fn empty() {
        let output = match tokenize("") {
            Err(a) => {
                eprintln!("{a}");
                return;
            }
            Ok(a) => a,
        };
        let expected = vec![Token::NewLine(1)];
        assert_eq!(output, expected);
    }

    #[test]
    fn instruction() {
        let output = match tokenize("add x1,x2,x3") {
            Err(a) => {
                eprintln!("{a}");
                return;
            }
            Ok(a) => a,
        };
        let expected = vec![
            Token::Identifier("add".to_string()),
            Token::Identifier("x1".to_string()),
            Token::Comma,
            Token::Identifier("x2".to_string()),
            Token::Comma,
            Token::Identifier("x3".to_string()),
            Token::NewLine(1),
        ];
        assert_eq!(output, expected);
    }
}
