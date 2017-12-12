extern crate rust_simple_search;

use rust_simple_search::store::Storage;
use rust_simple_search::index::Indexer;
use rust_simple_search::analysis::Analyzer;

fn main() {
    let store= Storage::new("", "");
    let analyzer = Analyzer::new();
    let indexer = Indexer::new(analyzer, store);

    // TODO new Document & loop documents
    //indexer.
}
