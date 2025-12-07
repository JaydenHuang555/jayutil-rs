use std::fmt::Display;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum FilePathError {
    ExtensionEmpty,
    ExtensionMultipleFlagsFound,
}

impl Display for FilePathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilePathError::ExtensionEmpty => {
                write!(f, "File extension was empty")
            }
            FilePathError::ExtensionMultipleFlagsFound => {
                write!(f, "Multiple file extensions flags were found")
            }
        }
    }
}
