use clang::SourceError;

#[derive(Debug)]
pub enum Error {
    RawMessage(String),
    ParseError(SourceError),
    PatternError(glob::PatternError),
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::RawMessage(value)
    }
}

impl From<SourceError> for Error {
    fn from(value: SourceError) -> Self {
        Self::ParseError(value)
    }
}

impl From<glob::PatternError> for Error {
    fn from(value: glob::PatternError) -> Self {
        Self::PatternError(value)
    }
}
