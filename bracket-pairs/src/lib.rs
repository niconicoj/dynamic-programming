pub fn list_solutions(pairs: usize) -> Vec<String> {
    _list_solutions(pairs, 0, 0)
}

fn _list_solutions(pairs: usize, current: usize, opened: usize) -> Vec<String> {
    if current >= (pairs * 2 - 1) {
        return vec![")".to_string()];
    } else {
        let should_open = opened == 0;
        let should_close = (pairs * 2 - current) <= opened;

        if should_open {
            let mut solutions = _list_solutions(pairs, current + 1, opened + 1);
            solutions.iter_mut().for_each(|s| s.insert(0, '('));
            return solutions;
        } else if should_close {
            let mut solutions = _list_solutions(pairs, current + 1, opened - 1);
            solutions.iter_mut().for_each(|s| s.insert(0, ')'));
            return solutions;
        } else {
            let mut solutions = _list_solutions(pairs, current + 1, opened + 1);
            solutions.iter_mut().for_each(|s| s.insert(0, '('));
            let mut more_solutions = _list_solutions(pairs, current + 1, opened - 1);
            more_solutions.iter_mut().for_each(|s| s.insert(0, ')'));
            solutions.extend(more_solutions);
            return solutions;
        }
    }
}

pub fn count_solutions(pairs: usize) -> usize {
    _count_solutions(pairs, 0, 0)
}

fn _count_solutions(pairs: usize, current: usize, opened: usize) -> usize {
    if current >= (pairs * 2 - 1) {
        return 1;
    } else {
        let should_open = opened == 0;
        let should_close = (pairs * 2 - current) <= opened;

        if should_open {
            _count_solutions(pairs, current + 1, opened + 1)
        } else if should_close {
            _count_solutions(pairs, current + 1, opened - 1)
        } else {
            _count_solutions(pairs, current + 1, opened + 1)
                + _count_solutions(pairs, current + 1, opened - 1)
        }
    }
}
