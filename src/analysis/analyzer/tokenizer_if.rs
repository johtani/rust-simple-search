use analysis::Token;

pub trait Tokenize {
    fn tokenize(&self, text: &str) -> Vec<Token>;
}
