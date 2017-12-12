use std::collections::HashMap;
use index::Postings;
use store::Storage;

pub struct InvertedIndex {
    ii: HashMap<String, Postings>,
    store: Storage,
}

impl InvertedIndex {
    pub fn new (store: Storage) -> InvertedIndex {
        InvertedIndex {
            ii: HashMap::new(),
            store
        }
    }

    pub fn createInvertedIndex(&self, tokens: &Vec<String>, docid: i64) {
        //loop tokens
        //
    }

    pub fn mergeInvertedIndexes(&self, original: InvertedIndex, new: InvertedIndex) {

    }
}