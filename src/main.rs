use std::path::PathBuf;

use clap::Parser;

#[derive(clap::Parser)]
struct Cli {
    #[arg(short, long)]
    input: PathBuf,

    #[arg(short, long)]
    output: PathBuf,

    #[arg(short, long)]
    pattern: Option<String>
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    let input_file = std::fs::File::open(cli.input)
        .map_err(|e| format!("failed to open input file '{}'", e))?;
    let reader = std::io::BufReader::new(input_file);
    let database = serde_json::from_reader(reader)
        .map_err(|e| format!("failed to load database: {}", e))?;

    let command_table = header_completer::build_command_table(database, cli.pattern)?;
    
    let output_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(cli.output)
        .map_err(|e| format!("failed to open output file '{}'", e))?;
    let writer = std::io::BufWriter::new(output_file);
    serde_json::to_writer_pretty(writer, &command_table.get_entries())
        .map_err(|e| format!("failed to save database: {}", e))?;

    Ok(())
}