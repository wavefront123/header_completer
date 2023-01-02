use std::path::PathBuf;

use crate::{compilation_database::{CompilationDatabase, CompilationDatabaseEntry}, error::Error};

use super::command_table::CompileCommandsTable;

pub struct Context<'c> {
    index: &'c clang::Index<'c>
}

impl<'c> Context<'c> {
    pub fn new(index: &'c clang::Index) -> Self {
        Self { index }
    }

    pub fn build_command_table(&self, database: CompilationDatabase) -> Result<CompileCommandsTable, Error> {
        let mut compile_commands_table = CompileCommandsTable::new();

        for entry in database {
            let entry = entry.skip_unnecessary_commands();
            compile_commands_table.insert(entry.get_file().clone(), entry);
        }

        Ok(compile_commands_table)
    }

    pub fn complete(&self, command_table: &CompileCommandsTable, pattern: Option<String>) -> Result<CompileCommandsTable, Error> {
        let pattern = Self::create_glob_pattern(pattern)?;
        let mut completed_command_table = command_table.clone();
        for entry in command_table.get_entries() {
            let directory = entry.get_directory();
            let file = entry.get_file();
            let command = entry.get_command();

            let translation_unit = self.parse(file.clone(), command.split(" ").map(String::from).collect())?;
            let include_file_paths = Self::extract_includes(translation_unit.get_entity());
            for include in include_file_paths {
                if let Some(pattern) = &pattern {
                    if !pattern.matches(include.to_str().unwrap()) {
                        continue;
                    }
                }
                let entry = CompilationDatabaseEntry::new(directory.clone(), include.clone(), command.clone());
                let entry = entry.skip_unnecessary_commands();
                completed_command_table.insert(include, entry);
            }
        }
        Ok(completed_command_table)
    }

    fn create_glob_pattern(pattern: Option<String>) -> Result<Option<glob::Pattern>, Error> {
        match pattern {
            Some(pattern) => {
                Ok(Some(glob::Pattern::new(pattern.as_str()).map_err(|e| e.to_string())?))
            },
            None => Ok(None)
        }
    }

    fn parse(&self, file_path: PathBuf, args: Vec<String>) -> Result<clang::TranslationUnit, Error> {
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

        for child in entity.get_children()
        {
            match child.get_kind() {
                clang::EntityKind::InclusionDirective => {
                    included_file_paths.push(child.get_file().unwrap().get_path());
                },
                _ => {}
            }
        }

        included_file_paths
    }
}
