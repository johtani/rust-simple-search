pub struct Token {
    pub term: String,
    start_offset: usize,
    end_offset: usize,
}

impl Token {
    pub fn new (term: String, start_offset: usize, end_offset: usize) -> Token {
        Token {
            term,
            start_offset,
            end_offset,
        }
    }
}