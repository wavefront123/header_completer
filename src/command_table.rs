use std::path::PathBuf;

use crate::compilation_database::{CompilationDatabase, CompilationDatabaseEntry};

#[derive(Clone)]
pub struct CompileCommandsTable {
    table: Vec<CompileCommandsTableEntry>,
}

#[derive(Clone)]
pub struct CompileCommandsTableEntry {
    directory: PathBuf,
    file: PathBuf,
    command: Vec<String>,
}

impl CompileCommandsTable {
    pub fn from_database(database: CompilationDatabase) -> Self {
        let mut table = Vec::new();

        for entry in database.entries() {
            table.push(CompileCommandsTableEntry::new(
                entry.directory(),
                entry.file(),
                entry.command(),
            ));
        }

        Self { table }
    }

    pub fn to_database(&self) -> CompilationDatabase {
        let mut entries = vec![];
        for entry in self.table.iter() {
            entries.push(CompilationDatabaseEntry::new(
                entry.directory(),
                entry.file(),
                entry.command(),
            ));
        }
        CompilationDatabase::new(entries)
    }

    pub fn entries(&self) -> impl Iterator<Item = &CompileCommandsTableEntry> + '_ {
        self.table.iter()
    }

    pub fn insert(
        &mut self,
        directory: &PathBuf,
        file: &PathBuf,
        command: &Vec<String>,
    ) {
        self.table
            .push(CompileCommandsTableEntry::new(directory, file, command));
    }
}

impl CompileCommandsTableEntry {
    pub fn new(
        directory: &PathBuf,
        file: &PathBuf,
        command: &Vec<String>,
    ) -> Self {
        Self {
            directory: directory.clone(),
            file: file.clone(),
            command: Self::skip_unnecessary_commands(command),
        }
    }

    pub fn directory(&self) -> &PathBuf {
        &self.directory
    }

    pub fn file(&self) -> &PathBuf {
        &self.file
    }

    pub fn command(&self) -> &Vec<String> {
        &self.command
    }

    pub fn skip_unnecessary_commands(command: &Vec<String>) -> Vec<String> {
        let mut result = vec![];
        let mut pos = 0;
        while pos < command.len() {
            let command = command.get(pos).unwrap();
            match command.as_str() {
                "-c" | "-o" => {
                    // skip
                    pos += 1;
                }
                _ => {
                    result.push(command.clone());
                }
            }
            pos += 1;
        }
        result
    }
}
