#[derive(Clone, Serialize, Deserialize)]
pub struct Postings {
    docid: u64,
    positions: Vec<usize>,
}

impl Postings {
    pub fn new (docid: u64) -> Self {
        Postings {
            docid,
            positions: vec![]
        }
    }

    pub fn add_position (&mut self, position: usize) {
        self.positions.push(position);
    }

    pub fn position_count (&mut self) -> usize {
        return self.positions.len();
    }

}


#[cfg(test)]
mod tests {
    extern crate env_logger;
    use index::Postings;

    #[test]
    fn success() {
        let mut postings = Postings::new(1);
        let expected = vec![1,2];
        postings.add_position(1);
        postings.add_position(2);
        assert_eq!(postings.position_count(), expected.len());
        assert_eq!(postings.docid, 1);
        for i in 0..expected.len() {
            assert_eq!(postings.positions[i], expected[i]);
        }
    }
}