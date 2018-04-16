#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rust_simple_search;

use rust_simple_search::store::Storage;
use rust_simple_search::index::Indexer;
use rust_simple_search::analysis::analyzer::NGramAnalyzer;
use rust_simple_search::search::Searcher;
use rust_simple_search::search::Results;

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

    //Indexing Phase
    info!("indexsing...");
    for (index, text) in texts.iter().enumerate() {
        indexer.add_document(index as u64, text);
    }

    info!("Indexed {} documents", texts.len());
    indexer.persist_inverted_index();
    info!("Persisted Inverted Index");

    //Search Phase
    let query = "Doc";
    info!("search \"{}\"", query);
    //FIXME consider lifetime in Rust...
    //FIXME
    let search_analyzer = NGramAnalyzer::new(2);
    let searcher = Searcher::new(search_analyzer, indexer.get_inverted_index());

}

/* persisted inverted index

{
  "inverted_index": {
    "1!": [
      {
        "docid": 0,
        "positions": [
          3
        ]
      }
    ],
    "2!": [
      {
        "docid": 1,
        "positions": [
          3
        ]
      }
    ],
    "Do": [
      {
        "docid": 0,
        "positions": [
          0
        ]
      },
      {
        "docid": 1,
        "positions": [
          0
        ]
      }
    ],
    "c1": [
      {
        "docid": 0,
        "positions": [
          2
        ]
      }
    ],
    "c2": [
      {
        "docid": 1,
        "positions": [
          2
        ]
      }
    ],
    "oc": [
      {
        "docid": 0,
        "positions": [
          1
        ]
      },
      {
        "docid": 1,
        "positions": [
          1
        ]
      }
    ],
    "キュ": [
      {
        "docid": 2,
        "positions": [
          1
        ]
      }
    ],
    "ト3": [
      {
        "docid": 2,
        "positions": [
          5
        ]
      }
    ],
    "ドキ": [
      {
        "docid": 2,
        "positions": [
          0
        ]
      }
    ],
    "メン": [
      {
        "docid": 2,
        "positions": [
          3
        ]
      }
    ],
    "ュメ": [
      {
        "docid": 2,
        "positions": [
          2
        ]
      }
    ],
    "ント": [
      {
        "docid": 2,
        "positions": [
          4
        ]
      }
    ]
  }
}

*/
