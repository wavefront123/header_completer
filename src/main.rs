use std::path::PathBuf;

use clap::Parser;
use header_completer::{compilation_database::CompilationDatabase, CompletionConfig};

/// An application for completing header file entries of C/C++ compilation database
#[derive(clap::Parser)]
struct Cli {
    /// input compilation database file path
    #[arg(short, long)]
    input: PathBuf,

    /// output compilation database file path
    #[arg(short, long)]
    output: PathBuf,

    /// glob patern to filter the header file paths to complete
    #[arg(short, long)]
    pattern: Option<String>,

    /// thread count to complete
    #[arg(short, long, default_value = "1")]
    thread_count: usize,
}

fn main() -> Result<(), header_completer::error::Error> {
    let cli = Cli::parse();

    let input_file =
        std::fs::File::open(cli.input).map_err(|e| format!("failed to open input file '{}'", e))?;
    let reader = std::io::BufReader::new(input_file);
    let database: CompilationDatabase =
        serde_json::from_reader(reader).map_err(|e| format!("failed to load database: {}", e))?;

    let config = CompletionConfig {
        pattern: cli.pattern,
        thread_count: cli.thread_count,
    };
    let database = header_completer::complete(database, config)?;

    let output_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(cli.output)
        .map_err(|e| format!("failed to open output file '{}'", e))?;
    let writer = std::io::BufWriter::new(output_file);
    serde_json::to_writer_pretty(writer, &database)
        .map_err(|e| format!("failed to save database: {}", e))?;

    Ok(())
}
