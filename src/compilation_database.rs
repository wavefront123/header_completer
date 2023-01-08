use std::{cmp::Ordering, path::PathBuf};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CompilationDatabaseEntryForDeserialize {
    directory: PathBuf,
    file: PathBuf,
    arguments: Option<Vec<String>>,
    command: Option<String>,
}

impl CompilationDatabaseEntryForDeserialize {
    pub fn to_entry(&self) -> CompilationDatabaseEntry {
        let arguments = self.arguments.clone();
        let commands = self
            .command
            .clone()
            .map(|command| command.split(" ").map(|c| c.to_string()).collect());
        CompilationDatabaseEntry {
            directory: self.directory.clone(),
            file: self.file.clone(),
            commands: arguments
                .or(commands)
                .expect("either 'arguments' or 'command' field is necessary."),
        }
    }

    pub fn from_entry(entry: &CompilationDatabaseEntry) -> Self {
        Self {
            directory: entry.directory.clone(),
            file: entry.file.clone(),
            arguments: Some(entry.commands.clone()),
            command: None,
        }
    }
}

pub type CompilationDatabaseForDeserialize = Vec<CompilationDatabaseEntryForDeserialize>;

#[derive(Clone, Debug, Eq, Ord, PartialEq)]
pub struct CompilationDatabaseEntry {
    directory: PathBuf,
    file: PathBuf,
    commands: Vec<String>,
}

pub type CompilationDatabase = Vec<CompilationDatabaseEntry>;

impl CompilationDatabaseEntry {
    pub fn new(
        directory: PathBuf,
        file: PathBuf,
        commands: Vec<String>,
    ) -> Self {
        Self {
            directory,
            file,
            commands,
        }
    }

    pub fn get_directory(&self) -> &PathBuf {
        return &self.directory;
    }
    pub fn get_file(&self) -> &PathBuf {
        return &self.file;
    }
    pub fn get_commands(&self) -> &Vec<String> {
        return &self.commands;
    }

    pub fn skip_unnecessary_commands(self) -> Self {
        let mut result = vec![];
        let mut pos = 0;
        while pos < self.commands.len() {
            let command = self.commands.get(pos).unwrap();
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
        Self {
            directory: self.directory,
            file: self.file,
            commands: result,
        }
    }
}

impl PartialOrd for CompilationDatabaseEntry {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        let file_order = self.file.partial_cmp(&other.file);
        let command_order = self.commands.partial_cmp(&other.commands);
        let directory_order = self.directory.partial_cmp(&other.directory);

        let mut result = None;
        result = result.or(file_order);
        result = result.or(command_order);
        result = result.or(directory_order);

        result
    }
}
