pub struct Document {
    pub id: u64,
    pub text: String,
}

impl Document {
    fn new (id: u64, text: &str) -> Document {
        Document {
            id,
            text: text.to_string(),
        }
    }
}