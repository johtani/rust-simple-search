use search::score_doc_id::ScoreDocId;


pub struct Results {
    query_string: String,
    docs: Vec<ScoreDocId>
}

impl Results {
    pub fn new (query: &str) -> Self {
        Results {
            query_string: query.to_string(),
            docs: vec![]
        }
    }
}