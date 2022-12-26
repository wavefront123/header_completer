use std::{path::PathBuf, cmp::Ordering};

#[derive(Clone, Debug, Eq, Ord, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CompilationDatabaseEntry {
    directory: PathBuf,
    file: PathBuf,
    command: String
}

pub type CompilationDatabase = Vec<CompilationDatabaseEntry>;

impl CompilationDatabaseEntry {

    pub fn new(directory: PathBuf, file: PathBuf, command: String) -> Self {
        Self {
            directory,
            file,
            command
        }
    }

    pub fn get_directory(&self) -> &PathBuf { return &self.directory; }
    pub fn get_file(&self) -> &PathBuf { return &self.file; }
    pub fn get_command(&self) -> &String { return &self.command; }

    pub fn skip_unnecessary_commands(self) -> Self {
        let mut result = vec![];
        let mut pos = 0;
        let commands: Vec<&str> = self.command.split(" ").collect();
        while pos < commands.len() {
            let command = commands.get(pos).unwrap();
            match *command {
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
        Self {
            directory: self.directory,
            file: self.file,
            command: commands.join(" "),
        }
    }
}

impl PartialOrd for CompilationDatabaseEntry {
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