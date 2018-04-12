use index::InvertedIndex;
use store::Storage;
use document::Document;
use analysis::Tokenize;
use search::Results;

pub struct Searcher<'ii, T: Tokenize> {
    inverted_index: &'ii InvertedIndex,
    analyzer: T
}

impl <'ii, T: Tokenize> Searcher<'ii,T> {
    pub fn new (analyzer: T, inverted_index: &'ii InvertedIndex) -> Self {
        Searcher {
            inverted_index,
            analyzer
        }
    }

    pub fn search(query: &str) -> Results {
        //TODO parse query

        let mut results = Results::new(query);

        return results;
    }
}
