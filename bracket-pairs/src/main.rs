use clap::{Parser, Subcommand};

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
    match Args::parse().command {
        Commands::List { n } => {
            let solutions = bracket_pairs::list_solutions(n);
            solutions
                .iter()
                .enumerate()
                .for_each(|(i, s)| println!("{}: {}", i + 1, s));
        }
        Commands::Count { n } => {
            let solutions = bracket_pairs::count_solutions(n);
            println!(
                "there are {} possible arrangements for {} bracket pairs",
                solutions, n
            );
        }
    }
}
