use std::{cmp::Ordering, path::PathBuf, slice::Iter};

use serde::{Deserialize, Serialize};

pub struct CompilationDatabase {
    entries: Vec<CompilationDatabaseEntry>,
}

#[derive(Clone, Debug, PartialEq, Eq, Ord)]
pub struct CompilationDatabaseEntry {
    directory: PathBuf,
    file: PathBuf,
    command: Vec<String>,
}

#[derive(serde::Serialize)]
struct CompilationDatabaseEntryForSerialize {
    directory: PathBuf,
    file: PathBuf,
    arguments: Vec<String>,
}

#[derive(serde::Deserialize)]
struct CompilationDatabaseEntryForDeserialize {
    directory: PathBuf,
    file: PathBuf,
    arguments: Option<Vec<String>>,
    command: Option<String>,
}

impl CompilationDatabase {
    pub fn new(mut entries: Vec<CompilationDatabaseEntry>) -> Self {
        entries.sort_by(|a, b| a.cmp(b));
        Self { entries }
    }

    pub fn entries(&self) -> Iter<CompilationDatabaseEntry> {
        self.entries.iter()
    }
}

impl CompilationDatabaseEntry {
    pub fn new(
        directory: &PathBuf,
        file: &PathBuf,
        command: &Vec<String>,
    ) -> Self {
        Self {
            directory: directory.clone(),
            file: file.clone(),
            command: command.clone(),
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
}

impl Serialize for CompilationDatabase {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let entries: Vec<_> = self
            .entries
            .iter()
            .map(|e| CompilationDatabaseEntryForSerialize {
                file: e.file.clone(),
                directory: e.directory.clone(),
                arguments: e.command.clone(),
            })
            .collect();
        entries.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CompilationDatabase {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        type EntryVec = Vec<CompilationDatabaseEntryForDeserialize>;
        let entries = EntryVec::deserialize(deserializer)?;
        let entries: Vec<_> = entries
            .into_iter()
            .map(|e| {
                let arguments = e.arguments;
                let commands = e
                    .command
                    .map(|cmd| cmd.split(" ").map(|arg| arg.to_string()).collect());
                CompilationDatabaseEntry {
                    file: e.file,
                    directory: e.directory,
                    command: arguments
                        .or(commands)
                        .expect("either 'arguments' or 'command' is required."),
                }
            })
            .collect();
        Ok(Self { entries })
    }
}

impl PartialOrd for CompilationDatabaseEntry {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        let file_order = self.file.partial_cmp(&other.file);
        let command_order = self.command().partial_cmp(&other.command());
        let directory_order = self.directory.partial_cmp(&other.directory);

        let mut result = None;
        result = result.or(file_order);
        result = result.or(command_order);
        result = result.or(directory_order);

        result
    }
}
