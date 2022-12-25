use std::{path::PathBuf, cmp::Ordering};

#[derive(Clone, Debug, Eq, Ord, PartialEq, serde::Serialize)]
pub struct CompileCommandsTableEntry {
    directory: PathBuf,
    file: PathBuf,
    command: Vec<String>
}

impl CompileCommandsTableEntry {
    pub fn new(directory: PathBuf, file: PathBuf, command: Vec<String>) -> Self {
        let command = Self::skip_unnecessary_commands(command);
        Self {
            directory,
            file,
            command
        }
    }

    pub fn get_directory(&self) -> &PathBuf { return &self.directory; }
    pub fn get_file(&self) -> &PathBuf { return &self.file; }
    pub fn get_command(&self) -> &Vec<String> { return &self.command; }

    fn skip_unnecessary_commands(commands: Vec<String>) -> Vec<String> {
        let mut result = vec![];
        let mut pos = 0;
        while pos < commands.len() {
            let command = commands.get(pos).unwrap();
            match command.as_str() {
                "-c" | "-o" => {
                    // skip
                    pos += 1;
                },
                _ => {
                    result.push(command.clone());
                }
            }
            pos += 1;
        }
        result
    }
}

impl PartialOrd for CompileCommandsTableEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let file_order = self.file.partial_cmp(&other.file);
        let command_order = self.command.partial_cmp(&other.command);
        let directory_order = self.directory.partial_cmp(&other.directory);

        let mut result = None;
        result = result.or(file_order);
        result = result.or(command_order);
        result = result.or(directory_order);

        result
    }
}