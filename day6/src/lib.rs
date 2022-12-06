use std::collections::HashMap;

pub fn read_file_vec(filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path: std::path::PathBuf = [env!("CARGO_MANIFEST_DIR"), filename].into_iter().collect();
    Ok(std::fs::read_to_string(path)?)
}

fn check_for_n_distinct_chars(data: String, length: usize) -> usize {
    let mut map: HashMap<char, u32> = HashMap::new();
    let (start, rest) = data.split_at(length);
    let letters = data.chars().collect::<Vec<_>>();

    start.chars().for_each(|letter| {
        *map.entry(letter).or_default() += 1;
    });

    if map.keys().len() == length {
        return 4;
    }

    for (index, letter) in rest.char_indices() {
        // remove last char
        let previous_char = letters[index];
        map.entry(previous_char).and_modify(|value| *value -= 1);
        if map[&previous_char] == 0 {
            map.remove(&previous_char);
        }

        // add current char
        *map.entry(letter).or_default() += 1;

        // check if there's just 4 chars
        if map.keys().len() == length {
            return length + 1 + index;
        }
    }

    data.len()
}

pub fn solve_part_1(file_content: String) -> usize {
    check_for_n_distinct_chars(file_content, 4)
}

pub fn solve_part_2(file_content: String) -> usize {
    check_for_n_distinct_chars(file_content, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: [&str; 5] = [
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];

    const ANSWERS_PART_1: [usize; 5] = [7, 5, 6, 10, 11];
    const ANSWERS_PART_2: [usize; 5] = [19, 23, 23, 29, 26];

    #[test]
    fn test_solve_part_1() {
        for (data, answer) in SAMPLE_DATA.iter().zip(ANSWERS_PART_1) {
            let result = solve_part_1(data.to_string());
            assert_eq!(result, answer);
        }
    }

    #[test]
    fn test_solve_part_2() {
        for (data, answer) in SAMPLE_DATA.iter().zip(ANSWERS_PART_2) {
            let result = solve_part_2(data.to_string());
            assert_eq!(result, answer);
        }
    }
}
