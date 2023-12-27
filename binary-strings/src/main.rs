fn main() {
    // read arguments from the command line
    // there should only be one argument
    // it should be an integer larger than 0
    let n = parse_input();
    let ways = count_possible_binary_strings(n);

    println!("There are {} ways to arrange {} binary digits.", ways, n);
}

fn count_possible_binary_strings(n: usize) -> usize {
    let mut dp_table = vec![0; n + 1];
    dp_table[1] = 2;
    dp_table[2] = 3;

    for i in 3..=n {
        dp_table[i] = dp_table[i - 1] + dp_table[i - 2];
    }

    dp_table[n]
}

fn parse_input() -> usize {
    let mut args = std::env::args();
    let arg = args.nth(1).expect("missing argument");
    arg.parse::<usize>().expect("argument must be an integer")
}
