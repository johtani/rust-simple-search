use serde_json;
use index::InvertedIndex;

pub struct Encode {

}

impl Encode {
    pub fn new () -> Encode {
        Encode {}
    }

    pub fn encode (&mut self, ii: InvertedIndex) -> String {
        //FIXME handling error
        return serde_json::to_string(&ii).unwrap();
    }
}