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
                let mut end_offset = start_offset;
                for j in 0..self.n {
                    term.push(char_array[i - 1 + j]);
                    end_offset = end_offset + char_array[i - 1 + j].len_utf8();
                }
                token_stream.push(Token::new(term,
                                             start_offset,
                                             end_offset));
                start_offset = start_offset + char_array[i-1].len_utf8();
                if i > char_array.len() - self.n {
                    break;
                }
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
        assert_eq!(actual.len(), expected.len(), "length: actual[{}] != expected[{}]", actual.len(), expected.len());
        for i in 0..actual.len() {
            assert_eq!(actual[i].term, expected[i], "[i:{}] actual [{}] != expected [{}]", i, actual[i].term, expected[i]);

        }
    }
    #[test]
    fn bi_gram_succecss() {
        //FIXME add positions to expected or make expected vector
        let n = 2;
        let analyzer = NGramAnalyzer::new(n);
        let text = "こんにちは";
        let expected = vec!["こん", "んに", "にち", "ちは"];
        assert_text_token(analyzer.tokenize(text), expected);
    }

    #[test]
    fn tri_gram_succecss() {
        let n = 3;
        let analyzer = NGramAnalyzer::new(n);
        let text = "こんにちは";
        let expected = vec!["こんに", "んにち", "にちは"];
        assert_text_token(analyzer.tokenize(text), expected);
    }



}