use index::InvertedIndex;
use store::Storage;
use document::Document;
use analysis::Tokenizer;
use search::Results;
use search::parseQuery;

pub struct Searcher<'ii, T: Tokenizer> {
    inverted_index: &'ii InvertedIndex,
    analyzer: T
}

impl <'ii, T: Tokenizer> Searcher<'ii,T> {
    pub fn new (analyzer: T, inverted_index: &'ii InvertedIndex) -> Self {
        Searcher {
            inverted_index,
            analyzer
        }
    }

    pub fn search(&self, query: &str) -> Results {
        let mut results = Results::new(query);
        //TODO parse query -> there is no support query syntax right now
        //TODO how create query with analyzer...
        let query = parseQuery(query, &self.analyzer);
        //TODO convert string to token w/ position
        let query_tokens = query.query_tokens;
        //TODO search tokens from inverted index
        //TODO

        return results;
    }
}
