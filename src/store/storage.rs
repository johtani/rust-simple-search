pub struct Storage {
    directory: String,
    file: String
}

impl Storage {
    pub fn new (directory: &str, file: &str) -> Self {
        Storage {
            directory: directory.to_string(),
            file: file.to_string(),
        }
    }

    pub fn persist () {

    }

    pub fn open () {

    }
}