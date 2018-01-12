use analysis::Token;

pub struct Analyzer {
    // TODO we will have settings
    // Need to find how to implement interface/abstract in Rust
}

impl Analyzer {
    pub fn new () -> Self {
        Analyzer {

        }
    }

    pub fn tokenize(&self, text: &str) -> Vec<Token> {
        let mut tokens = vec![];

        let pos = 0;
        tokens.push(Token::new(text.to_string(), pos, text.len()));
        // TODO treat...

        //

        return tokens;
    }
}


#[cfg(test)]
mod tests {
    extern crate env_logger;

    #[test]
    fn succecss() {
        env_logger::init().unwrap();
        info!("simple test");



    }

}