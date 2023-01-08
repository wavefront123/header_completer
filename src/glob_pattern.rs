use std::path::PathBuf;

use crate::error::Error;

#[derive(Clone)]
pub struct GlobPattern {
    pattern: Option<glob::Pattern>,
}

impl GlobPattern {
    pub fn new(pattern: Option<String>) -> Result<Self, Error> {
        let pattern = match pattern {
            Some(pattern) => Some(glob::Pattern::new(pattern.as_str())?),
            None => None,
        };

        Ok(Self { pattern })
    }

    pub fn matches(
        &self,
        path: &PathBuf,
    ) -> bool {
        match (&self.pattern, path.to_str()) {
            (Some(pattern), Some(path)) => pattern.matches(path),
            (Some(_), None) => false,
            (None, Some(_)) => true,
            (None, None) => false,
        }
    }
}
