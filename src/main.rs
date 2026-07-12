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
    Same { key_a: String, key_b: String },
    Merge { key_a: String, key_b: String },
    Groups,
    SEED,
    Exit,
}

fn main() {
    let mut ufdb = Ufdb::new();

    let mut lines = io::stdin().lock().lines();

    while let Some(line) = lines.next() {
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
                    Commands::Merge { key_a, key_b } => {
                        let res = ufdb.unite(&key_a, &key_b);
                        println!("{res}");
                    },
                    Commands::Same { key_a, key_b } => {
                        let res = ufdb.same(&key_a, &key_b);
                        println!("{res}");
                    }
                    Commands::Groups => {
                        let mut groups: Vec<Vec<&String>> = ufdb.groups().into_values().collect();

                        for group in &mut groups {
                            group.sort();
                        }

                        groups.sort_by(|a, b| b.len().cmp(&a.len()));

                        for group in groups {
                            let line: String = group.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ");

                            println!("{line}");
                        }
                    },
                    Commands::SEED => {
                        if ufdb.is_empty() {
                            ufdb.seed();
                        } else {
                            println!("既存のデータがあります。SEEDを実行しますか？ (y/n)");

                            let answer = lines.next().unwrap().unwrap();

                            if answer.trim() == "y" {
                                ufdb.seed();
                            }
                        }
                    },
                    Commands::Exit => {
                        println!("bye.");
                        break;
                    },
                }

                let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
                println!("elapsed: {elapsed_ms:.6}ms\n");
            },
            Err(e) => eprintln!("{e}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_same_command() {
        let cli = Cli::try_parse_from(["repl", "SAME", "a", "b"]).unwrap();

        assert!(matches!(cli.command, Commands::Same { key_a, key_b } if key_a == "a" && key_b == "b"));
    }

    #[test]
    fn parses_merge_command() {
        let cli = Cli::try_parse_from(["repl", "MERGE", "a", "b"]).unwrap();

        assert!(matches!(cli.command, Commands::Merge { key_a, key_b } if key_a == "a" && key_b == "b"));
    }
}
