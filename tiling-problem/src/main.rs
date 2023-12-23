fn main() {
    // read input args, there should be only one argument and it should be an integer greater than
    // zero
    let n = parse_input();

    let ways = count_ways(n);

    println!(
        "There {} {} {} to arrange tiles in a 4x{} stairs.",
        if ways == 1 { "is" } else { "are" },
        if ways == 1 { "way" } else { "ways" },
        ways,
        n
    );
}

fn parse_input() -> usize {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <integer>", args[0]);
        std::process::exit(1);
    }
    match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Error: argument must be an integer");
            std::process::exit(1);
        }
    }
}

fn count_ways(n: usize) -> usize {
    if n < 4 {
        1
    } else {
        count_ways(n - 1) + count_ways(n - 4)
    }
}
