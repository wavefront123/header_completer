use std::path::PathBuf;

use clap::Parser;

#[derive(clap::Parser)]
struct Cli {
    #[arg(short, long)]
    database_dir: PathBuf,

    #[arg(short, long)]
    output: PathBuf,

    #[arg(short, long)]
    pattern: Option<String>
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    let command_table = header_completer::build_command_table(cli.database_dir, cli.pattern)?;
    command_table.save(cli.output);
    Ok(())
}