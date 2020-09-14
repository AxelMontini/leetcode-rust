pub fn combination_sum3(k: u8, n: u8) -> Vec<Vec<u8>> {
    let mut results = Vec::new();

    backtrack(n, k, !0, &mut results, 0);

    results
}

/// Returns the solution as bitflag
fn backtrack(remain: u8, k: u8, possible: u16, results: &mut Vec<Vec<u8>>, start: u8) {
    // branch has been exhausted
    if k == 0 {
        // solution found
        if remain == 0 {
            results.push(
                (0..9)
                    .filter_map(|shift| {
                        if (!possible >> shift) & 1 == 1 {
                            Some(shift + 1)
                        } else {
                            None
                        }
                    })
                    .collect(),
            );
        }
        return;
    }

    (start..9)
        .filter(|&shift| (possible >> shift & 1 == 1) && remain > shift)
        .for_each(|shift| {
            backtrack(
                remain - shift - 1,
                k - 1,
                possible & !(1 << shift),
                results,
                shift,
            )
        });
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let tests: &[(u8, u8, &[&[u8]])] = &[
            (2, 8, &[&[1, 7], &[2, 6], &[3, 5]]),
            (3, 8, &[&[1, 2, 5], &[1, 3, 4]]),
            (1, 8, &[&[8]]),
            (4, 8, &[]),
            (2, 3, &[&[1, 2]]),
            (4, 11, &[&[1, 2, 3, 5]]),
            (3, 11, &[&[1, 2, 8], &[1, 3, 7], &[1, 4, 6], &[2, 3, 6], &[2, 4, 5]]),
        ];

        for (k, n, expected) in tests {
            assert_eq!(&super::combination_sum3(*k, *n), expected);
        }
    }
}
