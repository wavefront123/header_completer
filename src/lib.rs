use compilation_database::CompilationDatabase;
use context::Context;
use error::Error;

#[cfg(test)]
use compilation_database::CompilationDatabaseEntry;
#[cfg(test)]
use pretty_assertions::assert_eq;

pub mod compilation_database;
pub mod context;
pub mod error;
mod ordered_hash_set;

pub fn complete_compilation_database(
    database: &CompilationDatabase,
    completion_pattern: Option<String>,
) -> Result<CompilationDatabase, Error> {
    let clang = clang::Clang::new()?;
    let index = clang::Index::new(&clang, false, false);

    let context = Context::new(&index);

    let command_table = context.complete(database, completion_pattern)?;

    Ok(command_table)
}

#[test]
fn test_get_entries() -> Result<(), Error> {
    let current_dir =
        std::env::current_dir().map_err(|e| format!("failed to get curren directory: {}", e))?;
    let solve_path = |relative_path: &str| current_dir.join(relative_path);

    let cpp_project_path = solve_path("res/cpp_project");
    let cmake_build_path = solve_path("res/cpp_project/build");

    let compilation_database_path = solve_path("res/cpp_project/build/compile_commands.json");

    let cmake_output = std::process::Command::new("cmake")
        .arg("-S")
        .arg(cpp_project_path.clone())
        .arg("-B")
        .arg(cmake_build_path.clone())
        .arg("-D")
        .arg("CMAKE_CXX_COMPILER=clang++")
        .arg("-G")
        .arg("Ninja")
        .output()
        .map_err(|e| format!("cmake exeuction failed: {}", e))?;

    assert!(
        cmake_output.status.success(),
        "output: {}\n\nerror:{}",
        String::from_utf8(cmake_output.stdout).expect("failed to read stdout of cmake"),
        String::from_utf8(cmake_output.stderr).expect("failed to read stderr of cmake")
    );

    let pattern = solve_path("**/*.h").to_str().unwrap().into();

    let input_file = std::fs::File::open(compilation_database_path)
        .map_err(|e| format!("failed to open input file '{}'", e))?;
    let reader = std::io::BufReader::new(input_file);
    let database: CompilationDatabase =
        serde_json::from_reader(reader).map_err(|e| format!("failed to load database: {}", e))?;
    
    let find_args = |name: &str| database.entries.iter().find(|e| e.file.ends_with(name)).map(|e|e.command.clone()).unwrap();
    let skipped_args = crate::context::skip_unnecessary_commands(database.entries.last().ok_or(format!("failed to extract compiler arguments"))?.command.iter().map(|c| c.as_str()));

    let command_table = complete_compilation_database(&database, Some(pattern))?;
    assert_eq!(
        command_table.entries,
        vec![
            CompilationDatabaseEntry {
                directory: solve_path("res/cpp_project/build"),
                file: solve_path("res/cpp_project/src/main.cpp"),
                command: find_args("main.cpp"),
            },
            CompilationDatabaseEntry {
                directory: solve_path("res/cpp_project/build"),
                file: solve_path("res/cpp_project/src/sub.cpp"),
                command: find_args("sub.cpp"),
            },
            CompilationDatabaseEntry {
                directory: solve_path("res/cpp_project/build"),
                file: solve_path("res/cpp_project/src/a.h"),
                command: skipped_args.clone()
            },
            CompilationDatabaseEntry {
                directory: solve_path("res/cpp_project/build"),
                file: solve_path("res/cpp_project/src/b.h"),
                command: skipped_args.clone()
            },
            CompilationDatabaseEntry {
                directory: solve_path("res/cpp_project/build"),
                file: solve_path("res/cpp_project/src/c.h"),
                command: skipped_args.clone()
            },
            CompilationDatabaseEntry {
                directory: solve_path("res/cpp_project/build"),
                file: solve_path("res/cpp_project/src/d.h"),
                command: skipped_args.clone()
            },
        ]
    );
    Ok(())
}