use command_table::CompileCommandsTable;
use compilation_database::CompilationDatabase;
use context::Context;

pub mod command_table;
pub mod compilation_database;
pub mod context;

pub fn build_command_table(database: CompilationDatabase, completion_pattern: Option<String>) -> Result<CompileCommandsTable, String> {
    let clang = clang::Clang::new()?;
    let index = clang::Index::new(&clang, false, false);

    let context = Context::new(&index)?;

    let command_table = context.build_command_table(database)?;

    let command_table = context.complete(&command_table, completion_pattern)?;

    Ok(command_table)
}