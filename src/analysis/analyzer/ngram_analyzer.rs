use analysis::Token;
use analysis::Tokenize;

pub struct NGramAnalyzer {
    n: usize,
}

impl NGramAnalyzer {
    pub fn new (n: usize) -> NGramAnalyzer {
        NGramAnalyzer {
            n
        }
    }
}

impl Tokenize for NGramAnalyzer {
    fn tokenize(&self, text: &str) -> Vec<Token> {
        //FIXME need to decide how to handle control characters
        let mut token_stream = vec![];
        let char_counts = text.chars().count();
        if char_counts < self.n {
            info!("text chars count is lower than n");
            //FIXME should return text? or empty array?
        } else {
            let char_array = text.chars().collect::<Vec<char>>();
            let mut start_offset: usize = 0;

            for i in 1..char_array.len() {
                let mut term: String = String::new();
                // FIXME need to loop with self.n
                term.push(char_array[i-1]);
                term.push(char_array[i]);
                token_stream.push(Token::new(term,
                                             start_offset,
                                             start_offset + char_array[i-1].len_utf8() + char_array[i].len_utf8()));
                start_offset = start_offset + char_array[i-1].len_utf8();
            }
        }
        return token_stream;
    }
}

#[cfg(test)]
mod tests {
    extern crate env_logger;
    use analysis::Token;
    use analysis::Tokenize;
    use analysis::analyzer::NGramAnalyzer;

//    env_logger::init().unwrap();
//    info!("simple test");
//    info!("𠝏");
//    let hoge = "𠝏";
//    // &str.len()はバイト数 = 4
//    info!("len() = {}", hoge.len());
//    // &str.chars()は文字（char）配列を扱うIFっぽい = 1
//    info!("chars().count() = {}", hoge.chars().count());
//    // &str.bytes() はバイト配列かな？ = 4
//    info!("bytes().len() = {}", hoge.bytes().len());

    // TODO it should check offsets
    fn assert_text_token(actual: Vec<Token>, expected: Vec<&str>) {
        assert_eq!(actual.len(), expected.len());
        for i in 0..actual.len() {
            assert_eq!(actual[i].term, expected[i]);
        }
    }

    #[test]
    fn succecss() {
        let n = 2;
        let analyzer = NGramAnalyzer::new(n);
        let text = "こんにちは";
        let expected = vec!["こん", "んに", "にち", "ちは"];
        assert_text_token(analyzer.tokenize(text), expected);
    }

}