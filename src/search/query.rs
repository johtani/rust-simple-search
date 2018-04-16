use search::query_tokens::QueryTokens;
use analysis::Tokenizer;

pub struct Query {
    query_string: String,
    pub query_tokens: QueryTokens
}

impl <T:Tokenizer> Query {
    pub fn new(query_string: &str) -> Self {
        Query {
            query_string: query_string.to_string(),
            query_tokens: QueryTokens::new(),
        }
    }

    pub fn parse_query(&self, analyzer: &T) {
        let tokens = analyzer.tokenize(self.query_string);
        &self.query_tokens.create_query_tokens(tokens);
    }
}