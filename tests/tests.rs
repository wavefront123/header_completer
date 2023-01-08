use header_completer::{
    command_table::CompileCommandsTableEntry,
    compilation_database::{CompilationDatabase, CompilationDatabaseEntry},
    complete,
    error::Error,
    CompletionConfig,
};

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
        .arg(cpp_project_path)
        .arg("-B")
        .arg(cmake_build_path)
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

    let config = CompletionConfig {
        pattern: Some(solve_path("**/*.h").to_str().unwrap().into()),
        thread_count: 10,
    };

    let input_file = std::fs::File::open(compilation_database_path)
        .map_err(|e| format!("failed to open input file '{}'", e))?;
    let reader = std::io::BufReader::new(input_file);
    let database: CompilationDatabase =
        serde_json::from_reader(reader).map_err(|e| format!("failed to load database: {}", e))?;
    let args = CompileCommandsTableEntry::skip_unnecessary_commands(
        database
            .entries()
            .map(|e| e.command())
            .last()
            .ok_or("failed to extract compiler arguments".to_string())?,
    );

    let database = complete(database, config)?;
    assert_eq!(
        database.entries().collect::<Vec<_>>(),
        vec![
            &CompilationDatabaseEntry::new(
                &solve_path("res/cpp_project/build"),
                &solve_path("res/cpp_project/src/a.h"),
                &args
            ),
            &CompilationDatabaseEntry::new(
                &solve_path("res/cpp_project/build"),
                &solve_path("res/cpp_project/src/b.h"),
                &args
            ),
            &CompilationDatabaseEntry::new(
                &solve_path("res/cpp_project/build"),
                &solve_path("res/cpp_project/src/c.h"),
                &args
            ),
            &CompilationDatabaseEntry::new(
                &solve_path("res/cpp_project/build"),
                &solve_path("res/cpp_project/src/d.h"),
                &args
            ),
            &CompilationDatabaseEntry::new(
                &solve_path("res/cpp_project/build"),
                &solve_path("res/cpp_project/src/main.cpp"),
                &args
            ),
        ]
    );
    Ok(())
}
