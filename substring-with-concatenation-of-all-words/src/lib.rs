pub fn find_substrings(s: &str, words: &[&str]) -> Vec<usize> {
    use std::collections::HashMap;
    let mut indices = Vec::new();

    if s.len() == 0 || words.len() == 0 {
        return indices;
    }

    let len: usize = words.iter().map(|s| s.len()).sum();
    let words_map: HashMap<&str, usize> = words.iter().fold(HashMap::new(), |mut map, word| {
        map.entry(word).and_modify(|count| *count += 1).or_insert(1);
        map
    });
    let (longest_word, shortest_word) =
        words
            .iter()
            .map(|word| word.len())
            .fold((0, 0), |acc, len| {
                if acc.0 < len {
                    (len, acc.1)
                } else if acc.1 > len {
                    (acc.0, len)
                } else {
                    acc
                }
            });

    let mut start = 0;

    while start + len <= s.len() {
        let mut remaining = words_map.clone();
        let mut cursor = start;
        println!("ITER");

        while let Some((word, count)) = (shortest_word..=longest_word)
            .rev()
            .filter_map(|word_len| s.get(cursor..cursor + word_len))
            .find_map(|substr| {
                remaining.get_mut(substr).map(|count| {
                    *count -= 1;
                    (substr, *count)
                })
            })
        {
            println!("Removed {:?}", word);
            cursor += word.len();

            if count == 0 {
                remaining.remove(word);
            }
        }

        if remaining.len() == 0 {
            indices.push(start);
        }

        start = std::cmp::max(start + 1, start + shortest_word);
    }

    println!("DONE, output {:?}", indices);

    indices
}

#[cfg(test)]
mod tests {
    const DATASET: &[(&str, &[&str], &[usize])] = &[
        ("foobar", &["foo", "bar"], &[0]),
        ("foobar", &["bar", "foo"], &[0]),
        ("barfoo", &["bar", "foo"], &[0]),
        ("barfoo", &["foo", "bar"], &[0]),
        ("afoobar", &["bar", "foo"], &[1]),
        ("afoobarb", &["bar", "foo"], &[1]),
        ("fooafoobar", &["bar", "foo"], &[4]),
        ("foobarafoobar", &["bar", "foo"], &[0, 7]),
        ("afoobarafoobar", &["bar", "foo"], &[1, 8]),
        ("", &["HELO"], &[]),
        ("testword", &[], &[]),
        ("alongwordlo", &["a", "lo", "long", "word"], &[0]),
        ("doublematchyesdoublematch", &["match", "double"], &[0, 14]),
        (
            "barfoofoobarthefoobarman",
            &["bar", "foo", "the"],
            &[6, 9, 12],
        ),
        (
            "lingmindraboofooowingdingbarrwingmonkeypoundcake",
            &["fooo", "barr", "wing", "ding", "wing"],
            &[13],
        ),
    ];

    #[test]
    fn it_works() {
        for (string, words, expected) in DATASET {
            let output = super::find_substrings(string, words);
            assert_eq!(
                expected,
                &output.as_slice(),
                "Expected {:?}, found {:?} on string {:?} with words {:?}",
                expected,
                output,
                string,
                words
            );
        }
    }
}
