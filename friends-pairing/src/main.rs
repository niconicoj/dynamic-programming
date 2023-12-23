fn main() {
    let n = parse_input();

    let ways = count_ways(n);

    println!("There are {} ways", ways);
}

fn parse_input() -> usize {
    let mut args = std::env::args();
    let arg = args.nth(1).expect("missing argument");
    arg.parse::<usize>().expect("argument must be an integer")
}

fn count_ways(n: usize) -> usize {
    if n < 3 {
        n
    } else {
        count_ways(n - 1) + (n - 1) * count_ways(n - 2)
    }
}
