pub struct Query {
    query_string: String
}

impl Query {
    pub fn new(query_string: &str) -> Self {
        Query {
            query_string: query_string.to_string()
        }
    }
}