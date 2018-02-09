use index::InvertedIndex;
use store::Storage;
use document::Document;
use analysis::Tokenize;

pub struct Indexer<T: Tokenize> {
    inverted_index: InvertedIndex,
    analyzer: T,
    store: Storage,
    internal_id: u64
}

impl <T: Tokenize> Indexer<T> {
    pub fn new (analyzer: T, store: Storage) -> Self {
        Indexer {
            inverted_index: InvertedIndex::new(),
            analyzer,
            store,
            internal_id: 0
        }
    }

    pub fn create_internal_id(&mut self) -> u64 {
        self.internal_id = self.internal_id + 1;
        return self.internal_id;
    }

    //FIXME we should have fields instead of text. now, it is only support a field.
    pub fn add_document(&mut self, docid: u64, text: &str){
        //FIXME how to check existing doc?
        //analyze
        let doc = Document::new(docid,self.create_internal_id(), text);
        let tokens = self.analyzer.tokenize(&doc.text);
        let mut new_inverted_index = InvertedIndex::new();

        // check tokens.length
        if tokens.len() > 0 {
            new_inverted_index.create_inverted_index(&tokens, doc.id);
        }

        self.inverted_index.merge_inverted_indexes(&mut new_inverted_index);
    }

    pub fn persist_inverted_index(&mut self) {
        self.store.persist(&self.inverted_index);
    }

    pub fn load_meta_data(&mut self) {
        //FIXME should we load internal_id?
    }
}


#[cfg(test)]
mod tests {

}