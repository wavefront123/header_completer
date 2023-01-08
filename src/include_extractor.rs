use std::path::PathBuf;

use crate::error::Error;

pub struct IncludeExtractor<'c> {
    index: &'c clang::Index<'c>,
}

impl<'c> IncludeExtractor<'c> {
    pub fn new(index: &'c clang::Index) -> Self {
        Self { index }
    }

    pub fn extract(
        &self,
        file: &PathBuf,
        args: &Vec<String>,
    ) -> Result<Vec<PathBuf>, Error> {
        let translation_unit = self.parse(file.clone(), args.clone())?;
        Ok(Self::extract_includes(translation_unit.get_entity()).collect())
    }

    fn parse(
        &self,
        file_path: PathBuf,
        args: Vec<String>,
    ) -> Result<clang::TranslationUnit, Error> {
        let args: Vec<String> = args
            .into_iter()
            .filter(|arg| *arg != file_path.to_str().unwrap())
            .collect();
        let mut parser = self.index.parser(file_path.clone());
        let parser = parser
            .detailed_preprocessing_record(true)
            .ignore_non_errors_from_included_files(true)
            .keep_going(true)
            .skip_function_bodies(true)
            .arguments(&args);

        parser.parse().map_err(Error::from)
    }

    fn extract_includes(entity: clang::Entity) -> impl Iterator<Item = PathBuf> + '_ {
        entity.get_children().into_iter().filter_map(|child| {
            match (child.get_kind(), child.get_file()) {
                (clang::EntityKind::InclusionDirective, Some(file)) => Some(file.get_path()),
                _ => None,
            }
        })
    }
}
