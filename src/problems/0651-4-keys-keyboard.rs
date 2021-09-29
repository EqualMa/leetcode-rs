/// f(n) = max(
///     // A
///     f(n - 1) + 1,
///     // Ctrl-V
///     g(n, 1),
/// )
///
/// f(1) = 1
/// f(2) = 2
///
/// g(i, j) means the last j steps are Ctrl-V (j <= i - 3)
///
/// g(i, j) = max(
///     // Either the previous step is also Ctrl-V
///     g(i, j + 1),
///     // or the previous two steps are Ctrl-A + Ctrl-C
///     f(i - j - 2) * (j + 1)
/// ) }

pub mod recursion {
    use std::cmp::max;

    pub fn f(n: usize) -> usize {
        if n <= 2 {
            return n;
        }

        max(f(n - 1) + 1, g(n, 1))
    }

    fn g(n: usize, j: usize) -> usize {
        if j > n - 3 {
            return n - j;
        }

        max(g(n, j + 1), f(n - j - 2) * (j + 1))
    }

    pub use f as solve;
}

pub mod dp {
    use std::cmp::max;

    pub fn solve(n: usize) -> usize {
        let mut dpt = vec![0; n + 1];

        if n >= 1 {
            dpt[1] = 1;
        }
        if n >= 2 {
            dpt[2] = 2;
        }
        if n >= 3 {
            dpt[3] = 3;
        }

        if n >= 4 {
            for i in 4..=n {
                let v1 = dpt[i - 1] + 1;

                let v2 = {
                    let mut prev = 2usize; // j = i - 2
                    for j in (1..=(i - 3)).rev() {
                        // println!("prev = {}, j = {}, dpt = {:?}", prev, j, dpt);
                        prev = max(prev, dpt[i - j - 2] * (j + 1));
                    }

                    prev
                };

                // println!("i = {}, v1 = {}, v2 = {}, dpt = {:?}", i, v1, v2, dpt);

                dpt[i] = max(v1, v2);
            }
        }

        dpt[n]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem_0651() {
        for solve in [
            //
            // super::recursion::solve,
            super::dp::solve,
        ] {
            assert_eq!(solve(0), 0);
            assert_eq!(solve(1), 1);
            assert_eq!(solve(2), 2);
            assert_eq!(solve(3), 3);
            assert_eq!(solve(4), 4);
            assert_eq!(solve(5), 5);
            assert_eq!(solve(6), 6);
            assert_eq!(solve(7), 9);
            assert_eq!(solve(8), 12);
            assert_eq!(solve(9), 16);
            assert_eq!(solve(10), 20);
            assert_eq!(solve(11), 27);
            assert_eq!(solve(12), 36);
            assert_eq!(solve(13), 48);
            assert_eq!(solve(14), 64);
            assert_eq!(solve(15), 81);
            assert_eq!(solve(16), 108);
            assert_eq!(solve(17), 144);
            assert_eq!(solve(18), 192);
            assert_eq!(solve(19), 256);
            assert_eq!(solve(20), 324);
            assert_eq!(solve(21), 432);
            assert_eq!(solve(22), 576);
            assert_eq!(solve(23), 768);
            assert_eq!(solve(24), 1024);
            assert_eq!(solve(25), 1296);
            assert_eq!(solve(26), 1728);
            assert_eq!(solve(27), 2304);
            assert_eq!(solve(28), 3072);
            assert_eq!(solve(29), 4096);
            assert_eq!(solve(30), 5184);
        }
    }
}
