fn main() {
    let input = std::env::args().nth(1).expect("No input given");

    let result = compute_subsets(&input);

    println!("Subsets of {} are {:?}", input, result);
}

fn compute_subsets(input: &str) -> Vec<String> {
    let possibility = vec!["".to_string(), input[0..1].to_string()];
    if input.len() == 1 {
        return possibility;
    } else {
        let rest = compute_subsets(&input[1..]);

        rest.iter()
            .flat_map(|r| possibility.iter().map(move |p| format!("{}{}", p, r)))
            .collect()
    }
}
