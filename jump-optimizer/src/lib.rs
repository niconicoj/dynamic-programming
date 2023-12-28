pub fn optimize_jumps(schedule: &[usize], distance: usize) -> (usize, Vec<usize>) {
    let mut dp: Vec<(usize, Vec<usize>)> = vec![(0, vec![]); schedule.len()];

    for i in 0..schedule.len() {
        if i < distance {
            dp[i] = (schedule[i], vec![i]);
            continue;
        }

        dp[i] = dp[i - distance..i]
            .iter()
            .min_by_key(|(sum, _)| *sum)
            .map(|(sum, path)| {
                let mut path = path.clone();
                path.push(i);
                (sum + schedule[i], path)
            })
            .unwrap();
    }

    dp.last().unwrap().to_owned()
}

/*
0: 1 => 1, [0]
1: 3 => 3, [1]
2: 2 => 2, [2]
3: 3 => 4, [0, 3]
4: 3 => 5, [2, 4]
5: 1 => 3, [2, 5]
6: 1 => 4, [2, 5, 6]
7: 1 => 4, [2, 5, 7]
* */
