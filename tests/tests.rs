use header_completer::{*, command_table_entry::CompileCommandsTableEntry};

extern crate header_completer;

#[test]
fn test_get_entries() -> Result<(), String> {
    let current_dir = std::env::current_dir().map_err(|e| format!("failed to get curren directory: {}", e))?;
    let solve_path = |relative_path: &str| current_dir.join(relative_path);

    let cpp_project_path = solve_path("res/cpp_project");
    let cmake_build_path = solve_path("res/cpp_project/build");
    let cpp_project_include_path = solve_path("res/cpp_project/src");

    std::process::Command::new("cmake")
        .arg("-S").arg(cpp_project_path.clone())
        .arg("-B").arg(cmake_build_path.clone())
        .arg("-D").arg("CMAKE_CXX_COMPILER=clang++")
        .output()
        .map_err(|e| format!("cmake exeuction failed: {}", e))?;

    let pattern = solve_path("**/*.h").to_str().unwrap().into();

    let command_table = build_command_table(std::path::PathBuf::from(cmake_build_path), Some(pattern))?;
    assert_eq!(command_table.get_entries(), vec![
        &CompileCommandsTableEntry::new(
            solve_path("res/cpp_project/build"),
            solve_path("res/cpp_project/src/a.h"),
            vec![
                "/usr/bin/clang++".into(),
                "--driver-mode=g++".into(),
                format!("-I{}", cpp_project_include_path.display())
            ]
        ),
        &CompileCommandsTableEntry::new(
            solve_path("res/cpp_project/build"),
            solve_path("res/cpp_project/src/b.h"),
            vec![
                "/usr/bin/clang++".into(),
                "--driver-mode=g++".into(),
                format!("-I{}", cpp_project_include_path.display())
            ]
        ),
        &CompileCommandsTableEntry::new(
            solve_path("res/cpp_project/build"),
            solve_path("res/cpp_project/src/c.h"),
            vec![
                "/usr/bin/clang++".into(),
                "--driver-mode=g++".into(),
                format!("-I{}", cpp_project_include_path.display())
            ]
        ),
        &CompileCommandsTableEntry::new(
            solve_path("res/cpp_project/build"),
            solve_path("res/cpp_project/src/d.h"),
            vec![
                "/usr/bin/clang++".into(),
                "--driver-mode=g++".into(),
                format!("-I{}", cpp_project_include_path.display())
            ]
        ),
        &CompileCommandsTableEntry::new(
            solve_path("res/cpp_project/build"),
            solve_path("res/cpp_project/src/main.cpp"),
            vec![
                "/usr/bin/clang++".into(),
                "--driver-mode=g++".into(),
                format!("-I{}", cpp_project_include_path.display())
            ]
        ),
    ]);
    Ok(())
}