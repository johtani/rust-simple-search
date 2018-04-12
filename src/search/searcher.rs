use index::InvertedIndex;
use store::Storage;
use document::Document;
use analysis::Tokenize;
use search::Results;

pub struct Searcher<T: Tokenize> {
    inverted_index: InvertedIndex,
    analyzer: T
}

impl <T: Tokenize> Searcher<T> {
    pub fn new (analyzer: T, inverted_index: InvertedIndex) -> Self {
        Searcher {
            inverted_index,
            analyzer
        }
    }

    pub fn search(query: &str) -> Results {
        //FIXME implemnetation

        let mut results = Results::new(query);

        return results;
    }
}
