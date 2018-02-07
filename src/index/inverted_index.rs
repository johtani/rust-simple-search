use std::collections::BTreeMap;
use index::Postings;
use analysis::Token;

#[derive(Serialize, Deserialize)]
pub struct InvertedIndex {
    pub inverted_index: BTreeMap<String, Vec<Postings>>
}

impl InvertedIndex {
    pub fn new () -> InvertedIndex {
        InvertedIndex {
            inverted_index: BTreeMap::new()
        }
    }

    pub fn create_inverted_index(&mut self, tokens: &Vec<Token>, docid: u64) {
        if tokens.len() == 0 {
            warn!("doc[{}] has no tokens.", docid);
            return;
        }
        let distinct_terms = distinct_terms(tokens);
        for term in distinct_terms {
            let postings: Vec<Postings> = create_postings_per_token(docid, &term, tokens);
            &self.inverted_index.insert(term, postings);
        }
    }

    pub fn merge_inverted_indexes(&mut self, new_ii: &mut InvertedIndex) {
        for (key, value) in new_ii.inverted_index.iter_mut() {
            let mut postings_vec: Vec<Postings>;
            match self.inverted_index.get_mut(key) {
                Some(x) => postings_vec = x.to_vec(),
                None => postings_vec = Vec::new(),
            }
            postings_vec.append(value);
            &self.inverted_index.insert(key.to_string(), postings_vec.to_vec());
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

fn create_postings_per_token(docid: u64, term: &String, tokens: &Vec<Token>) -> Vec<Postings> {
    let mut postings = Postings::new(docid);
    for (i, token) in tokens.iter().enumerate() {
        if term.eq(&token.term) {
            postings.add_position(i);
        }
    }
    let mut postings_vec = Vec::new();
    postings_vec.push(postings);
    return postings_vec;
}

#[cfg(test)]
mod tests {

    mod inverted_index {
        use index::InvertedIndex;
        use analysis::Token;

        #[test]
        fn success() {
            let ii = InvertedIndex::new();
            let docid = 1;
            let tokens = vec![
                Token::new("a", 0, 1),
                Token::new("b", 2, 3),
                Token::new("a", 4, 5)
            ];
        }


    }


    mod distinct_terms {
        use index::inverted_index;
        use analysis::Token;

        #[test]
        fn success() {
            let tokens = vec![
                Token::new("a", 0, 1),
                Token::new("b", 2, 3),
                Token::new("a", 4, 5)
            ];
            let expected = vec![
                "a", "b"
            ];

            let actual = inverted_index::distinct_terms(&tokens);
            assert_eq!(actual.len(), expected.len());
            for i in 0..expected.len() {
                assert_eq!(actual[i], expected[i]);
            }

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
