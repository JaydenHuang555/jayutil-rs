
use crate::file_path::file_path_err::FilePathError;

pub const FLAG_CHAR: char = '.';

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum FileExtension {
    Valid(String),
    Invalid(FilePathError)
}

impl<'a> FileExtension {

    pub fn get(raw_buffer: String) -> FileExtension {
        let mut builder: String = String::new();
        let mut is_building_mode = false;
        for c in raw_buffer.chars() {
            if c == FLAG_CHAR {
                if is_building_mode {
                    return FileExtension::Invalid(FilePathError::ExtensionMultipleFlagsFound);
                }
                is_building_mode = true;
            }
            else if is_building_mode {
                builder.push(c);
            }
        }
        FileExtension::Valid(builder)
    }

    pub fn is_valid(&self) -> bool {
        match self {
            FileExtension::Valid(_) => true,
            FileExtension::Invalid(_) => false
        }
    }

    pub fn unwrap(&self) -> String {
        match self {
            FileExtension::Valid(contents) => contents.clone(),
            FileExtension::Invalid(error) => {
                panic!("Unable to unwrap due to error {}", error);
            }
        }
    }

}