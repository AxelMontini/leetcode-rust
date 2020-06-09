// # Explain
// 1) We find the length of each row and store them in row_lengths;
// 2) We iterate over every char. We have a "row_index" iterator that is used to get the row length
// 2.1) We find the length of the elements before this row
// 2.2) We calculate the offset relative to the first element of this row. If `rem != 0 || rem != num_rows - 1`,
// then this element is in a middle row (1..num_rows - 1): this means that the offset is given by index / (num_rows - 1). Why?
//
// Given the following short thingy, indexes in 0..num_rows are at offset 0, while index 5, 6 and 7 are at offset 1;
// Index 8 is offset 1 (not in a middle row), while index 9, 10, 11 are at offset 2.
//
// PAYPALISHIRING
//
// P       H -> offsets 0, 1
// A     S I -> offsets 0, 1, 2
// Y   I   R -> offsets 0, 1, 2
// P L     I G -> offsets 0, 1, 2, 3
// A       N -> offsets 0, 1

pub fn zigzag<S: Into<String> + AsRef<str>>(input: S, num_rows: usize) -> String {
    let input: &str = input.as_ref();

    if input.len() <= 1 || num_rows == 1 {
        return input.into();
    }

    let mut output: String = input.into();
    let part_len = num_rows * 2 - 2;
    let row_index_iter = (0..num_rows).chain((1..num_rows - 1).rev()).cycle();

    let mut row_lengths: Vec<usize> = vec![0; num_rows];
    // Find the row length
    for (_index, row_index) in (0..input.len()).zip(row_index_iter.clone()) {
        row_lengths[row_index] += 1;
    }

    let mut elements_before_row = vec![0; num_rows];

    for row_index in 0..num_rows {
        elements_before_row[row_index] = row_lengths[0..row_index].iter().sum();
    }

    let mut buf = [0u8; 4];

    for ((index, cur_char), row_index) in input.chars().enumerate().zip(row_index_iter) {
        let row_output_index: usize = elements_before_row[row_index];

        let rem = index % part_len;

        // If middle row, the offset is index / (num_rows - 1)
        let offset = if rem != 0 && rem != num_rows - 1 {
            index / (num_rows - 1)
        } else {
            index / part_len
        };

        output.replace_range(
            row_output_index + offset..=row_output_index + offset,
            cur_char.encode_utf8(&mut buf),
        );
    }

    output
}

// Stolen from Leetcode:
// let mut zigzags: Vec<_> = (0..num_rows)
//             .chain((1..num_rows-1).rev())
//             .cycle()
//             .zip(s.chars())
//             .collect();
//         zigzags.sort_by_key(|&(row, _)| row);
//         zigzags.into_iter()
//             .map(|(_, c)| c)
//             .collect()

#[cfg(test)]
mod tests {
    const DATASET: &[(&str, usize, &str)] = &[
        ("PAYPALISHIRING", 3, "PAHNAPLSIIGYIR"),
        ("PAYPALISHIRING", 4, "PINALSIGYAHRPI"),
        ("PAYPALISHIRING", 5, "PHASIYIRPLIGAN"),
    ];

    #[test]
    fn it_works() {
        for (input, num_rows, expected) in DATASET {
            let output = super::zigzag(*input, *num_rows);
            assert_eq!(
                expected, &output,
                "Expected {:?}, found {:?} for string {:?} with num_rows {}",
                expected, output, input, num_rows
            );
        }
    }
}
