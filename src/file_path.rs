
pub struct FilePath {
    raw_path: String
}

impl FilePath {

    pub fn new() -> FilePath {
        Self {
            raw_path: String::new()
        }
    }

    pub fn append_from_string(&mut self, buffer: String) {
    }

}