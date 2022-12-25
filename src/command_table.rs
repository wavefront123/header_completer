use std::{collections::HashMap, path::PathBuf, fs::OpenOptions, io::BufWriter};

use super::command_table_entry::CompileCommandsTableEntry;

#[derive(Clone)]
pub struct CompileCommandsTable {
    table: HashMap<PathBuf, Vec<CompileCommandsTableEntry>>
}

impl CompileCommandsTable {
    pub fn new() -> Self {
        Self {
            table: HashMap::new()
        }
    }

    pub fn insert(&mut self, path: PathBuf, entry: CompileCommandsTableEntry) {
        match self.table.get_mut(&path) {
            Some(entries) => {
                entries.push(entry)
            },
            None => {
                let entries = vec![entry];
                let prev_entry = self.table.insert(path, entries);
                assert!(prev_entry.is_none());
            }
        }
    }

    pub fn save(&self, path: PathBuf) {
        let all_entries = self.get_entries();

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .unwrap();
        let writer = BufWriter::new(file);

        serde_json::to_writer_pretty(writer, &all_entries).unwrap();
    }

    pub fn get_entries(&self) -> Vec<&CompileCommandsTableEntry> {
        let mut all_entries = vec![];

        for entries in self.table.values() {
            for entry in entries {
                all_entries.push(entry);
            }
        }

        all_entries.sort_by(|a, b| a.cmp(b));

        all_entries
    }
}
