pub fn read_file_vec(filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path: std::path::PathBuf = [env!("CARGO_MANIFEST_DIR"), filename].into_iter().collect();
    Ok(std::fs::read_to_string(path)?)
}

fn get_calories(file_content: String) -> Vec<usize> {
    let mut calories: Vec<usize> = file_content
        .split("\n\n")
        .into_iter()
        .map(|chunk| {
            chunk
                .split("\n")
                .into_iter()
                .map(|value| value.parse::<usize>().unwrap_or_default())
                .sum()
        })
        .collect();

    calories.sort_by(|a, b| b.cmp(a));

    calories
}

pub fn solve_part_1(file_content: String) -> usize {
    let calories = get_calories(file_content);

    *calories.first().unwrap()
}

pub fn solve_part_2(file_content: String) -> usize {
    let calories = get_calories(file_content);

    calories.into_iter().take(3).sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

    #[test]
    fn test_get_calories() {
        let result = get_calories(SAMPLE_DATA.to_string());
        assert_eq!(result, vec![24000, 11000, 10000, 6000, 4000]);
    }

    #[test]
    fn test_solve_part_1() {
        let result = solve_part_1(SAMPLE_DATA.to_string());
        assert_eq!(result, 24000);
    }

    #[test]
    fn test_solve_part_2() {
        let result = solve_part_2(SAMPLE_DATA.to_string());
        assert_eq!(result, 45000);
    }
}
