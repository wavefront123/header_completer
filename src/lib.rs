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
    completion_pattern: Option<String>,
) -> Result<CompilationDatabase, Error> {
    let pattern = GlobPattern::new(completion_pattern)?;

    let command_table = CompileCommandsTable::from_database(database);

    let clang = clang::Clang::new()?;
    let index = clang::Index::new(&clang, false, false);
    let extractor = IncludeExtractor::new(&index);

    let mut completed_command_table = command_table.clone();
    for entry in command_table.entries() {
        if pattern.matches(entry.file()) {
            for include in extractor.extract(entry.file(), entry.command())? {
                completed_command_table.insert(entry.directory(), &include, entry.command());
            }
        }
    }

    let database = command_table.to_database();

    Ok(database)
}
