use std::{fmt::Display, fs::File};



pub struct FilePath {
    raw_path: String
}

impl FilePath {

    pub fn new() -> FilePath {
        Self {
            raw_path: String::new()
        }
    }

    pub fn append_string(&mut self, buffer: String) {
        self.raw_path.push_str(buffer.as_str());
    }

    pub fn append_str(&mut self, buffer: &str) {
        self.raw_path.push_str(buffer);
    }

    pub fn append_entry(&mut self, entry: &str) {
        if !self.raw_path.is_empty() {
            self.raw_path.push('/');
        }
        self.raw_path.push_str(entry);
    }

    pub fn set_extension(&mut self, extension: &str) {
        self.raw_path.push('.');
        self.append_str(extension);
    }

    pub fn raw(&self) -> &String {
        return &self.raw_path;
    }

}

impl PartialEq for FilePath {
    fn eq(&self, other: &Self) -> bool {
        self.raw_path == other.raw_path
    }
}

impl Display for FilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw_path)
    }
}