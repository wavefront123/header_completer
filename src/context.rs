use std::path::PathBuf;

use crate::{
    compilation_database::{CompilationDatabase, CompilationDatabaseEntry},
    error::Error,
    ordered_hash_set::OrderedHashSet,
};

pub struct Context<'c> {
    index: &'c clang::Index<'c>,
}

impl<'c> Context<'c> {
    pub fn new(index: &'c clang::Index) -> Self {
        Self { index }
    }

    pub fn complete(
        &self,
        compilation_database: &CompilationDatabase,
        pattern: Option<String>,
    ) -> Result<CompilationDatabase, Error> {
        let path_matcher: Box<dyn Fn(&str) -> bool> = match pattern {
            Some(pattern) => {
                let pattern = Self::create_glob_pattern(pattern.as_str())?;
                Box::new(move |path: &str| pattern.matches(path))
            }
            None => Box::new(|_: &str| true),
        };

        let mut entries = OrderedHashSet::from_iter(compilation_database.entries.clone().into_iter());

        for entry in &compilation_database.entries {
            let command = skip_unnecessary_commands(entry.command.iter().map(|c| c.as_str()));

            let translation_unit = self.parse(entry.file.clone(), command.clone())?;
            let include_file_paths = Self::extract_includes(translation_unit.get_entity());
            entries.extend(
                include_file_paths
                    .into_iter()
                    .filter(|path| path_matcher(path.to_str().unwrap()))
                    .map(|path| CompilationDatabaseEntry {
                        directory: entry.directory.clone(),
                        file: path,
                        command: command.clone(),
                    }),
            );
        }

        Ok(CompilationDatabase { entries: entries.into_iter().collect::<Vec<_>>() })
    }

    fn create_glob_pattern(pattern: &str) -> Result<glob::Pattern, Error> {
        match glob::Pattern::new(pattern) {
            Ok(pattern) => Ok(pattern),
            Err(e) => Err(Error::RawMessage(e.to_string())),
        }
    }

    fn parse(
        &self,
        file_path: PathBuf,
        args: Vec<String>,
    ) -> Result<clang::TranslationUnit<'_>, Error> {
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

    fn extract_includes(entity: clang::Entity) -> Vec<PathBuf> {
        let mut included_file_paths = vec![];

        for child in entity.get_children() {
            match child.get_kind() {
                clang::EntityKind::InclusionDirective => {
                    included_file_paths.push(child.get_file().unwrap().get_path());
                }
                _ => {}
            }
        }

        included_file_paths
    }
}

pub(crate) fn skip_unnecessary_commands<'a, I: std::iter::Iterator<Item=&'a str>>(mut commands: I) -> Vec<String>
{
    let mut result = vec![];
    while let Some(command) = commands.next().take() {
        match command {
            "-c" | "-o" => {
                commands.next();
            }
            _ => result.push(command.to_string()),
        }
    }
    result
}

