mod movement;
mod supplies;

use crate::supplies::Supplies;

pub fn read_file_vec(filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path: std::path::PathBuf = [env!("CARGO_MANIFEST_DIR"), filename].into_iter().collect();
    Ok(std::fs::read_to_string(path)?)
}

pub fn solve_part_1(file_content: String) -> String {
    Supplies::new(file_content)
        .make_single_movements()
        .get_top_crates()
}

pub fn solve_part_2(file_content: String) -> String {
    Supplies::new(file_content)
        .make_bulk_movements()
        .get_top_crates()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    #[test]
    fn test_solve_part_1() {
        let result = solve_part_1(SAMPLE_DATA.to_string());
        assert_eq!(result, "CMZ".to_string());
    }

    #[test]
    fn test_solve_part_2() {
        let result = solve_part_2(SAMPLE_DATA.to_string());
        assert_eq!(result, "MCD".to_string());
    }
}
