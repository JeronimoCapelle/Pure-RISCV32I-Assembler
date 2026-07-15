use std::collections::HashMap;

use crate::auxiliar::{
    error::{
        AssemblerError,
        Stage::{self, SymbolCollection},
    },
    token::Token,
};

pub fn collect_symbols(
    tokens: &[Token],
) -> Result<(HashMap<String, usize>, Vec<Token>), AssemblerError> {
    let mut symbol_table: HashMap<String, usize> = HashMap::new();
    let mut symbol_free_tokens: Vec<Token> = Vec::new();
    let mut pc_counter: usize = 0;

    for line in tokens.split_inclusive(|t| matches!(t, Token::NewLine(_))) {
        //empty line
        if line.len() == 1 {
            continue;

        // instruction statement
        } else if (line.len() == 3) && (line[1] == Token::Colon) {
            let Token::Identifier(name) = &line[0] else {
                // expected identifier before colon
                let line_number = match line.last().unwrap() {
                    Token::NewLine(a) => a,
                    _ => return Err(AssemblerError::internal(SymbolCollection)),
                };

                return Err(AssemblerError::new(Stage::SymbolCollection, *line_number));
            };

            symbol_table.insert(name.clone(), pc_counter);
        } else {
            pc_counter += 4;
            symbol_free_tokens.append(&mut line.to_vec());
        }
    }

    Ok((symbol_table, symbol_free_tokens))
}
