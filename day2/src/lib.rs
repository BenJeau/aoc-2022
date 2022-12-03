mod object;
mod round;

use crate::round::Round;
use crate::object::Object;

pub fn read_file_vec(filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path: std::path::PathBuf = [env!("CARGO_MANIFEST_DIR"), filename].into_iter().collect();
    Ok(std::fs::read_to_string(path)?)
}

pub fn solve_part_1(file_content: String) -> usize {
    file_content
        .split("\n")
        .map(|value| {
            value
                .split(" ")
                .map(Object::convert_char)
                .flatten()
                .collect::<Round>()
                .total_score()
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = r#"A Y
B X
C Z"#;

    #[test]
    fn test_solve_part_1() {
        let result = solve_part_1(SAMPLE_DATA.to_string());
        assert_eq!(result, 15);
    }
}
