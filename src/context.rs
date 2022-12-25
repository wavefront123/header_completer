use std::path::PathBuf;

use super::{command_table::CompileCommandsTable, command_table_entry::CompileCommandsTableEntry};

pub struct Context<'c> {
    index: &'c clang::Index<'c>,
    database: clang::CompilationDatabase
}

impl<'c> Context<'c> {
    pub fn new(index: &'c clang::Index, path: PathBuf) -> Result<Self, String> {
        let database = clang::CompilationDatabase::from_directory(path).map_err(|_| "failed load Compilation Database")?;
        Ok(Self { index, database })
    }

    pub fn build_command_table(&self) -> Result<CompileCommandsTable, String> {
        let mut compile_commands_table = CompileCommandsTable::new();

        for command in self.database.get_all_compile_commands().get_commands() {
            let directory = command.get_directory();
            let file_path = command.get_filename();
            let arguments = command.get_arguments();

            let entry = CompileCommandsTableEntry::new(directory.clone(), file_path, arguments.clone());
            compile_commands_table.insert(command.get_filename(), entry);
        }

        Ok(compile_commands_table)
    }

    pub fn complete(&self, command_table: &CompileCommandsTable, pattern: Option<String>) -> Result<CompileCommandsTable, String> {
        let pattern = Self::create_glob_pattern(pattern)?;
        let mut completed_command_table = command_table.clone();
        for entry in command_table.get_entries() {
            let directory = entry.get_directory();
            let file = entry.get_file();
            let command = entry.get_command();

            let translation_unit = self.parse(file.clone(), command.clone())?;
            let include_file_paths = Self::extract_includes(translation_unit.get_entity());
            for include in include_file_paths {
                if let Some(pattern) = &pattern {
                    if !pattern.matches(include.to_str().unwrap()) {
                        continue;
                    }
                }
                let entry = CompileCommandsTableEntry::new(directory.clone(), include.clone(), command.clone());
                completed_command_table.insert(include, entry);
            }
        }
        Ok(completed_command_table)
    }

    fn create_glob_pattern(pattern: Option<String>) -> Result<Option<glob::Pattern>, String> {
        match pattern {
            Some(pattern) => {
                Ok(Some(glob::Pattern::new(pattern.as_str()).map_err(|e| e.to_string())?))
            },
            None => Ok(None)
        }
    }

    fn parse(&self, file_path: PathBuf, args: Vec<String>) -> Result<clang::TranslationUnit, String> {
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
            
        parser.parse().map_err(|e| format!("failed to parse: {}", e))
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
