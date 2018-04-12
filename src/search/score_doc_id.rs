pub struct ScoreDocId {
    doc_internal_id: u64,
    score: f64
}

impl ScoreDocId {
    pub fn new (doc_internal_id: u64, score: f64) -> Self {
        ScoreDocId {
            doc_internal_id,
            score
        }
    }
}

//FIXME Need Document?