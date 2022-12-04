use std::collections::HashSet;

use itertools::Itertools;

pub fn read_file_vec(filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path: std::path::PathBuf = [env!("CARGO_MANIFEST_DIR"), filename].into_iter().collect();
    Ok(std::fs::read_to_string(path)?)
}

fn char_value(letter: char) -> u32 {
    match letter {
        'a'..='z' => letter as u32 - 96,
        'A'..='Z' => letter as u32 - 64 + 26,
        _ => 0,
    }
}

pub fn solve_part_1(file_content: String) -> u32 {
    file_content
        .split("\n")
        .map(|items| items.chars().collect::<Vec<_>>())
        .map(|items| {
            let (first_bag, second_bag) = items.split_at(items.len() / 2);
            let hash: HashSet<&char> = HashSet::from_iter(first_bag);
            let hash2: HashSet<&char> = HashSet::from_iter(second_bag);
            **hash.intersection(&hash2).next().unwrap()
        })
        .map(char_value)
        .sum()
}

pub fn solve_part_2(file_content: String) -> u32 {
    file_content
        .split("\n")
        .map(|items| items.chars().collect::<Vec<_>>())
        .chunks(3)
        .into_iter()
        .map(|mut chunk| {
            let hash1 = chunk.next().unwrap();
            let hash2 = chunk.next().unwrap();
            let hash3 = chunk.next().unwrap();

            hash1
                .into_iter()
                .filter(|letter| hash2.contains(letter) && hash3.contains(letter))
                .next()
                .unwrap()
        })
        .map(char_value)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

    #[test]
    fn test_solve_part_1() {
        let result = solve_part_1(SAMPLE_DATA.to_string());
        assert_eq!(result, 157);
    }

    #[test]
    fn test_solve_part_2() {
        let result = solve_part_2(SAMPLE_DATA.to_string());
        assert_eq!(result, 70);
    }
}
