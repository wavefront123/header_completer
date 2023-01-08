use std::sync::{Arc, Mutex};

use command_table::CompileCommandsTable;
use compilation_database::CompilationDatabase;
use error::Error;
use glob_pattern::GlobPattern;
use include_extractor::IncludeExtractor;

pub mod command_table;
pub mod compilation_database;
pub mod error;
pub mod glob_pattern;
pub mod include_extractor;

pub fn complete(
    database: CompilationDatabase,
    config: CompletionConfig,
) -> Result<CompilationDatabase, Error> {
    let pattern = GlobPattern::new(config.pattern)?;

    let command_table = CompileCommandsTable::from_database(database);

    let clang = clang::Clang::new()?;
    let clang_holder = Arc::new(UnsafeClangHolder::new(&clang));

    let total_entries = command_table.entries().count();
    let entry_count = Arc::new(Mutex::new(0));

    let completed_command_table = std::thread::scope(|s| -> Result<CompileCommandsTable, Error> {
        let handles = command_table
            .split(config.thread_count)
            .into_iter()
            .map(|command_table| {
                let pattern = pattern.clone();
                let clang_holder = clang_holder.clone();
                let entry_count = entry_count.clone();
                s.spawn(move || -> Result<CompileCommandsTable, Error> {
                    let clang = clang_holder.unwrap();
                    let index = clang::Index::new(&clang, false, false);
                    let extractor = IncludeExtractor::new(&index);

                    let mut completed_command_table = command_table.clone();
                    for entry in command_table.entries() {
                        {
                            let mut lock = entry_count.lock().unwrap();
                            println!(
                                "[{} / {}] completing {}...",
                                *lock,
                                total_entries,
                                entry.file().display()
                            );
                            *lock += 1;
                        }
                        for include in extractor.extract(entry.file(), entry.command())? {
                            if pattern.matches(&include) {
                                completed_command_table.insert(
                                    entry.directory(),
                                    &include,
                                    entry.command(),
                                );
                            }
                        }
                    }
                    Ok(completed_command_table)
                })
            })
            .collect::<Vec<_>>();

        let mut completed_command_tables = vec![];
        for handle in handles {
            completed_command_tables.push(handle.join().unwrap()?);
        }
        Ok(CompileCommandsTable::merge(
            completed_command_tables.into_iter(),
        ))
    })?;

    let database = completed_command_table.to_database();

    Ok(database)
}

pub struct CompletionConfig {
    pub pattern: Option<String>,
    pub thread_count: usize,
}

// clang::Clang is not marked as Send/Sync and is not permitted to initialize multiple times to use in another threads,
// so use this for unsafely share the libclang context
struct UnsafeClangHolder {
    ptr: *const clang::Clang,
}

unsafe impl std::marker::Send for UnsafeClangHolder {}
unsafe impl std::marker::Sync for UnsafeClangHolder {}

impl UnsafeClangHolder {
    pub fn new(clang: &clang::Clang) -> Self {
        Self {
            ptr: clang as *const _,
        }
    }

    pub fn unwrap(&self) -> &clang::Clang {
        unsafe { self.ptr.as_ref().unwrap() }
    }
}
