pub struct Document {
    pub id: i64,
    pub text: String,
}

impl Document {
    fn new (id: i64, text: &str) -> Document {
        Document {
            id: id,
            text: text.to_string(),
        }
    }
}