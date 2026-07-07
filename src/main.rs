use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
#[command(rename_all = "UPPER")]
enum Commands {
    Hello,
    Insert { key: String },
    Exit,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hello => {
            println!("Hello World");
        },
        Commands::Insert { key } => {
            println!("key = {key}");
        },
        Commands::Exit => {
            println!("bye.");
        },
    }
}
