use std::collections::VecDeque;

fn main() {
    let input = std::env::args().nth(1).expect("missing input");
    let input = input.parse::<usize>().expect("invalid input");

    let solutions = solve(input);

    solutions.iter().for_each(|solution| {
        pretty_print(solution);
        println!();
    });
}

fn solve(input: usize) -> Vec<Vec<usize>> {
    let mut solutions: Vec<Vec<usize>> = vec![];
    let mut current = VecDeque::new();

    _solve(input, &mut solutions, &mut current);

    solutions
}

fn _solve(input: usize, solutions: &mut Vec<Vec<usize>>, current: &mut VecDeque<usize>) {
    (0..input).for_each(|i| {
        if is_valid(current, i) {
            current.push_back(i);
            if current.len() == input {
                solutions.push(current.clone().into());
            } else {
                _solve(input, solutions, current);
            }
            current.pop_back();
        }
    });
}

fn is_valid(current: &VecDeque<usize>, i: usize) -> bool {
    if current.is_empty() {
        return true;
    }

    let row = current.len();
    let column_already_used = current.contains(&i);
    let diagonal_already_used = current.iter().enumerate().any(|(r, queen)| {
        queen.checked_sub(row - r).is_some_and(|v| v == i)
            || queen.checked_add(row - r).is_some_and(|v| v == i)
    });

    if column_already_used || diagonal_already_used {
        return false;
    }
    true
}

fn pretty_print(solution: &Vec<usize>) {
    let dim = solution.len();
    solution.iter().enumerate().for_each(|(row, queen)| {
        if row == 0 {
            print!("┌");
            (0..dim - 1).for_each(|_| print!("──┬"));
            println!("──┐");
        }
        (0..dim).for_each(|col| {
            if col == *queen {
                print!("│♕ ");
            } else {
                print!("│  ");
            }
        });
        println!("│");
        if row == dim - 1 {
            print!("└");
            (0..dim - 1).for_each(|_| print!("──┴"));
            println!("──┘");
        } else {
            print!("├");
            (0..dim - 1).for_each(|_| print!("──┼"));
            println!("──┤");
        }
    });
}

#[cfg(test)]
mod test {
    use std::collections::VecDeque;

    use crate::is_valid;

    #[test]
    fn is_valid_when_empty() {
        let current: VecDeque<usize> = VecDeque::from([]);

        assert!(is_valid(&current, 0));
        assert!(is_valid(&current, 2));
        assert!(is_valid(&current, 3));
    }

    #[test]
    fn is_valid_when_first_is_used() {
        let current: VecDeque<usize> = VecDeque::from([0]);

        assert!(!is_valid(&current, 0));
        assert!(!is_valid(&current, 1));
        assert!(is_valid(&current, 2));
        assert!(is_valid(&current, 3));
    }

    #[test]
    fn is_valid_when_two_above_are_used() {
        let current: VecDeque<usize> = VecDeque::from([0, 2]);

        assert!(!is_valid(&current, 0));
        assert!(!is_valid(&current, 1));
        assert!(!is_valid(&current, 2));
        assert!(!is_valid(&current, 3));
    }
}
