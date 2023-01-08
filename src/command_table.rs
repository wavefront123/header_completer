use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use crate::compilation_database::{CompilationDatabase, CompilationDatabaseEntry};

#[derive(Clone)]
pub struct CompileCommandsTable {
    table: HashSet<CompileCommandsTableEntry>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CompileCommandsTableEntry {
    directory: PathBuf,
    file: PathBuf,
    command: Vec<String>,
}

impl CompileCommandsTable {
    pub fn from_database(database: CompilationDatabase) -> Self {
        let mut table = HashSet::new();

        for entry in database.entries() {
            table.insert(CompileCommandsTableEntry::new(
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
        directory: &Path,
        file: &Path,
        command: &Vec<String>,
    ) {
        self.table
            .insert(CompileCommandsTableEntry::new(directory, file, command));
    }

    pub fn split(
        self,
        n: usize,
    ) -> Vec<Self> {
        let mut result = vec![];
        let entries: Vec<_> = self.entries().into_iter().collect();

        let len = self.table.len();

        for i in 0..n {
            let mut table = HashSet::new();

            let begin = len * i / n;
            let end = len * (i + 1) / n;

            for entry in &entries[begin..end] {
                table.insert(CompileCommandsTableEntry::new(
                    entry.directory(),
                    entry.file(),
                    entry.command(),
                ));
            }

            result.push(Self { table });
        }

        result
    }

    pub fn merge<I: Iterator<Item = Self>>(selves: I) -> Self {
        let mut table = HashSet::new();
        for s in selves {
            table.extend(s.table);
        }

        Self { table }
    }
}

impl CompileCommandsTableEntry {
    pub fn new(
        directory: &Path,
        file: &Path,
        command: &Vec<String>,
    ) -> Self {
        Self {
            directory: directory.to_path_buf(),
            file: file.to_path_buf(),
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
