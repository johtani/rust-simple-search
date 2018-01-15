use std::collections::HashMap;
use index::Postings;
use analysis::Token;

#[derive(Serialize, Deserialize)]
pub struct InvertedIndex {
    pub inverted_index: HashMap<String, Postings>
}

impl InvertedIndex {
    pub fn new () -> InvertedIndex {
        InvertedIndex {
            inverted_index: HashMap::new()
        }
    }

    pub fn create_inverted_index(&mut self, tokens: &Vec<Token>, docid: u64) {
        if tokens.len() == 0 {
            warn!("doc[{}] has no tokens.", docid);
            return;
        }
        let distinct_terms = distinct_terms(tokens);
        for term in distinct_terms {
            let postings: Postings = create_postings_per_token(docid, &term, tokens);
            &self.inverted_index.insert(term, postings);
        }
    }

    pub fn merge_inverted_indexes(&mut self, new_ii: &InvertedIndex) {
        for (key, value) in &new_ii.inverted_index {
            &self.inverted_index.insert(key.to_string(), value.clone());
        }
    }
}

fn distinct_terms(tokens: &Vec<Token>) -> Vec<String> {
    let mut distinct_terms: Vec<String> = vec![];
    for token in tokens.iter() {
        if !distinct_terms.contains(&token.term) {
            distinct_terms.push(token.term.to_string());
        }
    }
    return distinct_terms;
}

fn create_postings_per_token(docid: u64, term: &String, tokens: &Vec<Token>) -> Postings {
    let mut postings = Postings::new(docid);
    for (i, token) in tokens.iter().enumerate() {
        if term.eq(&token.term) {
            postings.add_position(i);
        }
    }
    return postings;
}

#[cfg(test)]
mod tests {
    use super::InvertedIndex;

    mod distinct_terms {
        #[test]
        fn success() {

        }

        #[test]
        fn empty_tokens() {

        }
    }

    mod create_postings_per_token {
        #[test]
        fn success() {

        }

        #[test]
        fn empty_tokens() {

        }

    }
}
