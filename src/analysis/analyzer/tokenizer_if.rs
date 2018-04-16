use analysis::Token;

pub trait Tokenizer {
    fn tokenize(&self, text: &str) -> Vec<Token>;
}
