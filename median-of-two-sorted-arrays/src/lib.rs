use std::cmp::{max, min};

pub fn median(a: &[i64], b: &[i64]) -> f64 {
    if a.len() == 0 && b.len() == 0 {
        return 0.0;
    }

    // Swap a and b if needed, so that a.len() <= b.len()
    let (a, b) = if a.len() > b.len() { (b, a) } else { (a, b) };

    let mut index_min = 0;
    let mut index_max = a.len();

    println!("A len : {}, b len : {}", a.len(), b.len());

    let median = loop {
        if index_min > index_max {
            break 0.0;
        }

        let index_a = (index_min + index_max) / 2;
        let index_b = (a.len() + b.len() + 1) / 2 - index_a;

        println!(
            "ITER; index ({}, {}), value ({:?}, {:?})",
            index_a,
            index_b,
            a.get(index_a),
            b.get(index_b)
        );

        if index_a < index_max && b[index_b - 1] > a[index_a] {
            index_min = index_a + 1;
        } else if index_a > index_min && a[index_a - 1] > b[index_b] {
            index_max = index_a - 1;
        } else {
            let max_left = if index_a == 0 {
                b[index_b - 1]
            } else if index_b == 0 {
                a[index_a - 1]
            } else {
                max(a[index_a - 1], b[index_b - 1])
            };

            if (a.len() + b.len()) & 1 == 1 {
                break max_left as f64;
            }

            let max_right = if index_a == a.len() {
                b[index_b]
            } else if index_b == b.len() {
                a[index_a]
            } else {
                min(a[index_a], b[index_b])
            };

            break (max_left + max_right) as f64 / 2.0;
        }
    };

    median
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATASET: &[(&[i64], &[i64], f64)] = &[
        (&[], &[], 0.0),
        (&[0], &[], 0.0),
        (&[], &[0], 0.0),
        (&[0], &[0], 0.0),
        (&[-1], &[1], 0.0),
        (&[1, 2], &[2], 2.0),
        (&[1, 2], &[3, 4], 2.5),
        (&[1, 2], &[1, 2], 1.5),
    ];

    #[test]
    fn correctness() {
        for (a, b, expected) in DATASET {
            let output = median(a, b);
            assert_eq!(
                *expected, output,
                "Expected {}, found {}. Dataset {:?} and {:?}",
                expected, output, a, b
            );
        }
    }
}
