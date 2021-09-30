/// ``` txt
///  Permutations for  1234
///
///       4, 3, 2
///  0 => 0, 0, 0   => 1234
///  1 => 1, 0, 0   => 2134
///  2 => 2, 0, 0   => 3214
///  3 => 3, 0, 0   => 4231
///  4 => 0, 1, 0   => 1324
///  5 => 1, 1, 0   => 2314
///  6 => 2, 1, 0   => 3124
///  7 => 3, 1, 0   => 4321
///  8 => 0, 2, 0   => 1432
///  9 => 1, 2, 0   => 2431
/// 10 => 2, 2, 0   => 3412
/// 11 => 3, 2, 0   => 4132
/// 12 => 0, 0, 1   => 1243
/// 13 => 1, 0, 1   => 2143
/// 14 => 2, 0, 1   => 3241
/// 15 => 3, 0, 1   => 4213
/// ```
pub mod hash_factorial {
    pub fn solve(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();

        let count: usize = (1..=n).product();

        let mut results = vec![nums; count];

        for i in 0..count {
            let result = &mut results[i];

            let mut v = i;
            for j in (2..=n).rev() {
                let remainder = v % j;

                if remainder != 0 {
                    let start = n - j;
                    result.swap(start, start + remainder);
                }

                v = (v - remainder) / j;
            }
        }

        results
    }
}

pub mod flat_map {
    pub fn solve(nums: Vec<i32>) -> Vec<Vec<i32>> {
        struct State {
            choices: Vec<i32>,
            result: Vec<i32>,
        }

        let n = nums.len();

        (0..n)
            .fold(
                vec![State {
                    choices: nums,
                    result: Vec::with_capacity(n),
                }],
                |results, _| {
                    results
                        .into_iter()
                        .flat_map(|state| {
                            let State { choices, result } = state;

                            (0..choices.len())
                                .map(|i| {
                                    let mut choices = choices.clone();
                                    let mut result = result.clone();
                                    let num = choices.remove(i);
                                    result.push(num);
                                    State { choices, result }
                                })
                                .collect::<Vec<_>>()
                        })
                        .collect()
                },
            )
            .into_iter()
            .map(|state| state.result)
            .collect()
    }
}

pub mod backtracking {
    fn backtrack(results: &mut Vec<Vec<i32>>, result: &mut Vec<i32>, choices: Vec<i32>) {
        if choices.len() == 0 {
            results.push(result.clone())
        }

        for (i, choice) in choices.iter().enumerate() {
            result.push(*choice);

            let mut choices = choices.clone();
            choices.remove(i);

            backtrack(results, result, choices);

            result.pop();
        }
    }

    pub fn solve(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut results = vec![];
        let mut result = vec![];

        backtrack(&mut results, &mut result, nums);

        results
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, iter::FromIterator};

    #[test]
    fn problem_0064() {
        let algorithms = [
            //
            super::hash_factorial::solve,
            super::flat_map::solve,
            super::backtracking::solve,
        ];
        for solve in algorithms {
            {
                let permutations = solve(vec![1, 2, 3]);

                let permutations: HashSet<Vec<i32>> = permutations.into_iter().collect();

                let res: HashSet<Vec<i32>> = [
                    [1, 2, 3],
                    [1, 3, 2],
                    [2, 1, 3],
                    [2, 3, 1],
                    [3, 1, 2],
                    [3, 2, 1],
                ]
                .iter()
                .map(|arr| arr.to_vec())
                .collect();

                assert_eq!(permutations, res);
            }

            {
                let permutations = solve(vec![1, 2, 3, 4]);

                let permutations: HashSet<Vec<i32>> = permutations.into_iter().collect();

                let res: HashSet<Vec<i32>> = [
                    [1, 2, 3, 4],
                    [1, 2, 4, 3],
                    [1, 3, 2, 4],
                    [1, 3, 4, 2],
                    [1, 4, 2, 3],
                    [1, 4, 3, 2],
                    [2, 1, 3, 4],
                    [2, 1, 4, 3],
                    [2, 3, 1, 4],
                    [2, 3, 4, 1],
                    [2, 4, 1, 3],
                    [2, 4, 3, 1],
                    [3, 1, 2, 4],
                    [3, 1, 4, 2],
                    [3, 2, 1, 4],
                    [3, 2, 4, 1],
                    [3, 4, 1, 2],
                    [3, 4, 2, 1],
                    [4, 1, 2, 3],
                    [4, 1, 3, 2],
                    [4, 2, 1, 3],
                    [4, 2, 3, 1],
                    [4, 3, 1, 2],
                    [4, 3, 2, 1],
                ]
                .iter()
                .map(|arr| arr.to_vec())
                .collect();

                assert_eq!(permutations, res);
            }
        }

        let mut result = None;

        for solve in algorithms {
            let res = HashSet::<Vec<i32>>::from_iter(solve(vec![1, 2, 3, 4, 5]));

            match result {
                Some(ref m) => assert_eq!(&res, m),
                None => result = Some(res),
            }
        }
    }
}
