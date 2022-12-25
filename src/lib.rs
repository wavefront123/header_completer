use std::path::PathBuf;

use command_table::CompileCommandsTable;
use context::Context;

pub mod command_table;
pub mod command_table_entry;
pub mod context;

pub fn build_command_table(database_dir: PathBuf, completion_pattern: Option<String>) -> Result<CompileCommandsTable, String> {
    let clang = clang::Clang::new()?;
    let index = clang::Index::new(&clang, false, false);

    let context = Context::new(&index, database_dir)?;

    let command_table = context.build_command_table()?;

    let command_table = context.complete(&command_table, completion_pattern)?;

    Ok(command_table)
}