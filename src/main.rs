#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rust_simple_search;

use rust_simple_search::store::Storage;
use rust_simple_search::index::Indexer;
use rust_simple_search::analysis::Analyzer;
use rust_simple_search::document::Document;

fn main() {
    env_logger::init().unwrap();
    info!("start!!");
    let store= Storage::new("", "");
    let analyzer = Analyzer::new();
    let indexer = Indexer::new(analyzer, store);

    // TODO new Document & loop documents
    let docid = 1;
    let text = "New Document!";
    let doc = Document::new(docid, text);
    indexer.add_document(doc);
}
