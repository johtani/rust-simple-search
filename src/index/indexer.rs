use index::InvertedIndex;
use store::Storage;
use document::Document;
use analysis::Analyzer;

pub struct Indexer {
    invertedIndex: InvertedIndex,
    analyzer: Analyzer,
}

impl Indexer {
    pub fn new (analyzer: Analyzer, store: Storage) -> Self {
        Indexer {
            analyzer,
            invertedIndex: InvertedIndex::new(store)
        }
    }

    pub fn addDocument(&self, doc: Document) {
        //analyze
        let tokens = self.analyzer.tokenize(&doc.text);
        let newInvertedIndex = InvertedIndex::new(store);

        // check tokens.length
        if tokens.len() > 0 {
            newInvertedIndex.createInvertedIndex(&tokens, doc.id);
        }

        self.invertedIndex.mergeInvertedIndexes(self.invertedIndex, newInvertedIndex);
    }

}

