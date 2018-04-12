#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rust_simple_search;

use rust_simple_search::store::Storage;
use rust_simple_search::index::Indexer;
use rust_simple_search::analysis::analyzer::NGramAnalyzer;

// this is example implementation...
fn main() {
    env_logger::init();
    info!("start!!");
    let store= Storage::new("/tmp", "hoge_index.json");
    let analyzer = NGramAnalyzer::new(2);
    let mut indexer = Indexer::new(analyzer, store);

    let texts = vec![
        "Doc1!", "Doc2!", "ドキュメント3"
    ];

    for (index, text) in texts.iter().enumerate() {
        indexer.add_document(index as u64, text);
    }

    indexer.persist_inverted_index();

}
