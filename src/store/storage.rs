use std::io::prelude::*;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use index::InvertedIndex;
use store::Encode;

pub struct Storage {
    directory: String,
    file_name: String,
    encoder: Encode,
    file_path: Path,
}

impl Storage {
    pub fn new (directory: &str, file: &str) -> Self {
        Storage {
            directory: &directory.to_string(),
            file_name: &file.to_string(),
            encoder: Encode::new(),
            file_path: Path::new(&directory.to_string()).join(&file.to_string()),
        }
    }

    pub fn persist (&mut self, ii: InvertedIndex) {
        let file = match File::open(&self.file_path){
            Ok(file) => file,
            Err(e) => {
                println!("An error occurred while opening file {}:{}", &self.file_path.to_str().unwrap(), e);
                return;
            }
        };
        let mut file_writer = BufWriter::new(file);
        file_writer.write(&self.encoder.encode(ii).as_bytes());
        file_writer.flush();
    }

    pub fn open () {
        //FIXME
    }
}

#[cfg(test)]
mod tests {
    
}