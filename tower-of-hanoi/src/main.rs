use clap::Parser;

mod cli;
fn main() {
    let args = cli::Args::parse();
    solve(args.disks);
}

fn solve(disks: usize) {
    let rods = ['A', 'B', 'C'];

    tower_of_hanoi(disks, rods[0], rods[2], rods[1]);
}

fn tower_of_hanoi(n: usize, from: char, to: char, aux: char) {
    if n == 1 {
        println!("Move disk 1 from rod {} to rod {}", from, to);
        return;
    }
    tower_of_hanoi(n - 1, from, aux, to);
    println!("Move disk {} from rod {} to rod {}", n, from, to);
    tower_of_hanoi(n - 1, aux, to, from);
}
