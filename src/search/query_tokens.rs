use std::collections::HashMap;
use analysis::Token;

pub struct QueryTokens {
    token_position_map: HashMap<String, Vec<usize>>
}

impl QueryTokens {
    pub fn new () -> Self {
        QueryTokens {
            token_position_map: HashMap::new()
        }
    }

    pub fn create_query_tokens(&mut self, tokens: &Vec<Token>) {
        if tokens.len() == 0 {
            warn!("queryhas no tokens...");
            return;
        }
        let distinct_terms = distinct_terms(tokens);
        for term in distinct_terms {
            let mut positions = vec![];
            for (i, token) in tokens.iter().enumerate() {
                if term.eq(&token.term) {
                    positions.push(i);
                }
            }
            &self.token_position_map.insert(term, positions);
        }
    }
}

//FIXME same function in inverted_index.rs...
fn distinct_terms(tokens: &Vec<Token>) -> Vec<String> {
    let mut distinct_terms: Vec<String> = vec![];
    for token in tokens.iter() {
        if !distinct_terms.contains(&token.term) {
            distinct_terms.push(token.term.to_string());
        }
    }
    return distinct_terms;
}

