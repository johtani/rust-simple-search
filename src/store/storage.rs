use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use index::InvertedIndex;
use store::Encode;

pub struct Storage {
    directory: String,
    file: String,
    encoder: Encode,
}

impl Storage {
    pub fn new (directory: &str, file: &str) -> Self {
        Storage {
            directory: directory.to_string(),
            file: file.to_string(),
            encoder: Encode::new(),
        }
    }

    pub fn persist (&mut self, ii: InvertedIndex) {
        let file_path = Path::new(&self.directory).join(self.file);
        let file = match File::open(file_path){
            Ok(file) => file,
            Err(e) => {
                println!("An error occurred while opening file {}:{}", file_path.to_string(), e);
                return;
            }
        };

        // to_JSON with Encode
        // Open file
        //
    }

    pub fn open () {

    }
}

#[cfg(test)]
mod tests {
    
}