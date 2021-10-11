/// `f(i, j)` represents:
/// when starting with `piles[i..=j]`,
/// the one who starts first will get `f(i, j)` more than the other one.
///
/// Note that `0 ≤ i ≤ j < piles.len()` and `f(i, j)` might be negative.
///
/// If `i == j`, `f(i, j) = piles[i]`
///
/// Else,
///
/// ```ignore
/// f(i, j) = max(
///     // the player takes piles[i]
///     piles[i] - f(i + 1, j),
///     // the player takes piles[j]
///     piles[j] - f(i, j - 1),
/// )
/// ```
///
/// The one who starts first will win the game if and only if `f(0, n-1) > 0`
///
/// Now the problem is to get `f(0, n-1)`
pub mod dp {
    use std::cmp::max;

    pub fn solve(piles: &[i32]) -> bool {
        let n = piles.len();
        let mut dp = vec![-1; n * n];
        for i in (0..n).rev() {
            for j in i..n {
                if i == j {
                    dp[i * n + j] = piles[i];
                } else {
                    let f1 = dp[(i + 1) * n + j];
                    let f2 = dp[i * n + j - 1];
                    assert_ne!(f1, -1);
                    assert_ne!(f2, -1);
                    dp[i * n + j] = max(piles[i] - f1, piles[j] - f2);
                }
            }
        }

        dp[n - 1] > 0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem_0877() {
        for solve in [
            //
            super::dp::solve,
        ] {
            for (piles, result) in [
                //
                (&[1][..], true),
                (&[1, 4], true),
                (&[5, 3, 4, 5], true),
                (&[3, 7, 2, 3], true),
                (&[1, 3, 1], false),
            ] {
                let solved = solve(piles);

                assert_eq!(solved, result);
            }
        }
    }
}
