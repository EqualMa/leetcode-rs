/// f(i) represents the length of LIS of nums[0..=i] which ends with nums[i]
///
/// Then we want max(f(0), f(1), ..., f(n-1))
///
/// f(i) = max(
///     if nums[i] > nums[0] { f(0) + 1 } else { 1 },
///     ...
///     if nums[i] > nums[i-1] { f(i-1) + 1 } else { 1 },
/// )
///
/// f(0) = 1
pub mod dp {
    pub fn solve(nums: &[i32]) -> usize {
        let n = nums.len();
        let mut res = vec![1; n];

        for i in 1..n {
            res[i] = (0..i)
                .map(|j| if nums[i] > nums[j] { res[j] + 1 } else { 1 })
                .max()
                .unwrap();
        }

        res.into_iter().max().unwrap()
    }
}

// pub mod binary_search {
//     pub fn solve(nums: &[i32]) -> usize {
//         todo!()
//     }
// }

#[cfg(test)]
mod tests {
    #[test]
    fn problem_0300() {
        for solve in [
            //
            super::dp::solve,
        ] {
            for (piles, result) in [
                //
                (&[1][..], 1),
                (&[10, 9, 2, 5, 3, 7, 101, 18], 4),
                (&[0, 1, 0, 3, 2, 3], 4),
                (&[7, 7, 7, 7, 7, 7, 7], 1),
            ] {
                let solved = solve(piles);

                assert_eq!(solved, result);
            }
        }
    }
}
