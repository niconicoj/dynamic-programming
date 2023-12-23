use clap::{Parser, Subcommand};
use n_queens::{count_all_possibility, list_all_possibility, pretty_print};

#[derive(Parser)]
#[command(author, about)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List { n: usize },
    Count { n: usize },
}

fn main() {
    let input = Args::parse().command;

    match input {
        Commands::List { n } => {
            let solutions = list_all_possibility(n);

            if solutions.len() == 0 {
                println!("There are no solutions for {n} queens");
            }

            solutions.iter().for_each(|solution| {
                pretty_print(solution);
                println!();
            });
        }
        Commands::Count { n } => {
            let count = count_all_possibility(n);
            println!("There are {} solutions for {} queens", count, n);
        }
    }
}
