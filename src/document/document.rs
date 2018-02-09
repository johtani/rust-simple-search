pub struct Document {
    pub id: u64,
    pub internal_id: u64,
    pub text: String,
}

impl Document {
    pub fn new (id: u64, internal_id: u64, text: &str) -> Document {
        Document {
            id,
            internal_id,
            text: text.to_string(),
        }
    }

}