use crate::structures::{ParsingError::NonIdentifier, Token, TrackedError};
use std::collections::HashMap;

pub fn collect_symbols(
    tokens: &[Token],
) -> Result<(HashMap<String, usize>, Vec<Token>), TrackedError> {
    let mut symbol_table: HashMap<String, usize> = HashMap::new();
    let mut symbol_free_tokens: Vec<Token> = Vec::new();
    let mut pc_counter: usize = 0;

    for line in tokens.split(|t| *t == Token::NewLine) {
        if line.ends_with(&[Token::Colon]) && line.len() == 2 {
            let name = match &line[0] {
                Token::Identifier(a) => a,
                _ => {
                    return Err(TrackedError {
                        kind: NonIdentifier,
                        line: line!(),
                        file: file!(),
                    });
                }
            };
            symbol_table.insert(name.to_string(), pc_counter);
        } else if line.is_empty() {
            continue;
        } else {
            pc_counter += 4;
            symbol_free_tokens.append(&mut line.to_vec());
            symbol_free_tokens.push(Token::NewLine);
        }
    }

    symbol_free_tokens.pop();

    Ok((symbol_table, symbol_free_tokens))
}
