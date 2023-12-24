use clap::Parser;

#[derive(Parser)]
#[command(author, about)]
struct Args {
    path: String,
}

fn main() {
    let path = Args::parse().path;

    let graph = hamiltonian_paths::Graph::from_file(&path);

    graph.hamiltonian_paths().iter().for_each(|path| {
        println!("{}", path.join(" -> "));
    });
}
