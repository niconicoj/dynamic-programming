fn main() {
    // read arguments from the command line
    // there should only be one argument
    // it should be an integer larger than 0
    let n = parse_input();
    let ways = count_possible_binary_strings(n);

    println!("There are {} ways to arrange {} binary digits.", ways, n);
}

fn count_possible_binary_strings(n: usize) -> usize {
    if n == 1 {
        2
    } else if n == 2 {
        3
    } else {
        count_possible_binary_strings(n - 1) + count_possible_binary_strings(n - 2)
    }
}

fn parse_input() -> usize {
    let mut args = std::env::args();
    let arg = args.nth(1).expect("missing argument");
    arg.parse::<usize>().expect("argument must be an integer")
}
