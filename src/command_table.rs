use std::{collections::HashMap, path::PathBuf};

use crate::compilation_database::CompilationDatabaseEntry;

#[derive(Clone)]
pub struct CompileCommandsTable {
    table: HashMap<PathBuf, Vec<CompilationDatabaseEntry>>
}

impl CompileCommandsTable {
    pub fn new() -> Self {
        Self {
            table: HashMap::new()
        }
    }

    pub fn insert(&mut self, path: PathBuf, entry: CompilationDatabaseEntry) {
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

    pub fn get_entries(&self) -> Vec<&CompilationDatabaseEntry> {
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
