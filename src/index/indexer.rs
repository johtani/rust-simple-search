use index::InvertedIndex;
use store::Storage;
use document::Document;
use analysis::Analyzer;

pub struct Indexer {
    inverted_index: InvertedIndex,
    analyzer: Analyzer,
    store: Storage
}

impl Indexer {
    pub fn new (analyzer: Analyzer, store: Storage) -> Self {
        Indexer {
            inverted_index: InvertedIndex::new(),
            analyzer,
            store
        }
    }

    pub fn add_document(&mut self, doc: Document) {
        //analyze
        let tokens = self.analyzer.tokenize(&doc.text);
        let mut new_inverted_index = InvertedIndex::new();

        // check tokens.length
        if tokens.len() > 0 {
            new_inverted_index.create_inverted_index(&tokens, doc.id);
        }

        self.inverted_index.merge_inverted_indexes(&new_inverted_index);
    }

    pub fn persist_inverted_index(&mut self) {
        self.store.persist(&self.inverted_index);
    }
}


#[cfg(test)]
mod tests {

}