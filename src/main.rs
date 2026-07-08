use std::io::{self, BufRead};

use clap::{Parser, Subcommand};
use toy_ufdb::Ufdb;

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
    let mut ufdb = Ufdb::new();

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();

        let mut tokens = vec!["repl"];
        tokens.extend(line.split_whitespace());

        match Cli::try_parse_from(tokens) {
            Ok(cli) => {
                let start = std::time::Instant::now();

                match cli.command {
                    Commands::Hello => {
                        println!("Hello World");
                    },
                    Commands::Insert { key } => {
                        let inserted = ufdb.make_set(&key);
                        println!("{inserted}");
                    },
                    Commands::Exit => {
                        println!("bye.");
                        break;
                    },
                }

                let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
                println!("elapsed: {elapsed_ms:.6}ms");
            },
            Err(e) => eprintln!("{e}"),
        }
    }
}
