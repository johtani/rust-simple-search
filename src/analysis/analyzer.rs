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

    pub fn tokenize(&self, text: &String) -> Vec<Token> {
        let tokens = vec![];

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
        warn!("hoge");
    }

}