pub struct Postings {
    docid: i64,
    positions: Vec<i64>,
}

impl Postings {
    fn new (docid: i64) -> Postings {
        Postings {
            docid: docid,
            positions: vec![]
        }
    }

    fn add_position (&mut self, position: i64) {
        self.positions.push(position);
    }

    fn position_count (&self) -> usize {
        return self.positions.len();
    }

}