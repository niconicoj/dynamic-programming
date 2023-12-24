use clap::Parser;
use sudoku_solver::Sudoku;

#[derive(Parser)]
#[command(author, about)]
struct Args {
    path: String,
}

fn main() {
    let args = Args::parse();

    let mut sudoku = Sudoku::from_file(&args.path).unwrap();

    println!("Unsolved puzzle :\n{}", sudoku);
    println!("solving...\n");
    sudoku.solve();
    println!("Solution :\n{}", sudoku);
}
