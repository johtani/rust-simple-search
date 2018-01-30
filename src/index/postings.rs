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

}