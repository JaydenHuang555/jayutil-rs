

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

    pub fn get_extension(&self) -> String {

    }

    pub fn set_extension(&mut self, extension: String) {
        
    }

}