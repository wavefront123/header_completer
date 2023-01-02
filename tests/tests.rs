use header_completer::{build_command_table, compilation_database::{CompilationDatabaseEntry, CompilationDatabase}, error::Error};


#[test]
fn test_get_entries() -> Result<(), Error> {
    let current_dir = std::env::current_dir().map_err(|e| format!("failed to get curren directory: {}", e))?;
    let solve_path = |relative_path: &str| current_dir.join(relative_path);

    let cpp_project_path          = solve_path("res/cpp_project");
    let cmake_build_path          = solve_path("res/cpp_project/build");

    let compilation_database_path = solve_path("res/cpp_project/build/compile_commands.json");

    let cmake_output = std::process::Command::new("cmake")
        .arg("-S").arg(cpp_project_path.clone())
        .arg("-B").arg(cmake_build_path.clone())
        .arg("-D").arg("CMAKE_CXX_COMPILER=clang++")
        .arg("-G").arg("Ninja")
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
    let database: CompilationDatabase = serde_json::from_reader(reader)
        .map_err(|e| format!("failed to load database: {}", e))?;
    let args = database
        .clone()
        .into_iter()
        .map(|e| e.skip_unnecessary_commands().get_command().clone())
        .last()
        .ok_or(format!("failed to extract compiler arguments"))?;

    let command_table = build_command_table(database, Some(pattern))?;
    assert_eq!(command_table.get_entries(), vec![
        &CompilationDatabaseEntry::new(
            solve_path("res/cpp_project/build"),
            solve_path("res/cpp_project/src/a.h"),
            args.clone()
        ),
        &CompilationDatabaseEntry::new(
            solve_path("res/cpp_project/build"),
            solve_path("res/cpp_project/src/b.h"),
            args.clone()
        ),
        &CompilationDatabaseEntry::new(
            solve_path("res/cpp_project/build"),
            solve_path("res/cpp_project/src/c.h"),
            args.clone()
        ),
        &CompilationDatabaseEntry::new(
            solve_path("res/cpp_project/build"),
            solve_path("res/cpp_project/src/d.h"),
            args.clone()
        ),
        &CompilationDatabaseEntry::new(
            solve_path("res/cpp_project/build"),
            solve_path("res/cpp_project/src/main.cpp"),
            args.clone()
        ),
    ]);
    Ok(())
}