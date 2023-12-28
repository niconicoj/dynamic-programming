use clap::Parser;

#[derive(Parser)]
#[command(author, about)]
struct Args {
    path: String,
    #[clap(short, long)]
    distance: usize,
}

fn main() {
    let Args { path, distance } = Args::parse();

    let schedule = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter_map(|l| l.parse::<usize>().ok())
        .collect::<Vec<_>>();

    let best = jump_optimizer::optimize_jumps(&schedule, distance);

    println!("Best: {:?}", best);
}
