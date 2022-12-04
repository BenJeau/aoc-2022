use std::ops::RangeInclusive;

pub fn read_file_vec(filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path: std::path::PathBuf = [env!("CARGO_MANIFEST_DIR"), filename].into_iter().collect();
    Ok(std::fs::read_to_string(path)?)
}

fn convert_string_to_range(string: &str) -> RangeInclusive<u32> {
    let result: Vec<_> = string
        .split("-")
        .map(|value| value.parse::<u32>().unwrap())
        .collect();

    result[0]..=result[1]
}

fn are_ranges_completly_overlapping<T: Ord>(
    range1: &RangeInclusive<T>,
    range2: &RangeInclusive<T>,
) -> bool {
    range1.contains(range2.start()) && range1.contains(range2.end())
}

fn are_ranges_overlapping<T: Ord>(range1: &RangeInclusive<T>, range2: &RangeInclusive<T>) -> bool {
    (range1.contains(range2.start()) && !range1.contains(range2.end()))
        || are_ranges_completly_overlapping(range1, range2)
}

pub fn solve_part_1(file_content: String) -> usize {
    file_content
        .split("\n")
        .filter(|line| {
            let ranges: Vec<_> = line.split(",").map(convert_string_to_range).collect();
            let range1 = &ranges[0];
            let range2 = &ranges[1];

            are_ranges_completly_overlapping(range1, range2)
                || are_ranges_completly_overlapping(range2, range1)
        })
        .count()
}

pub fn solve_part_2(file_content: String) -> usize {
    file_content
        .split("\n")
        .filter(|line| {
            let ranges: Vec<_> = line.split(",").map(convert_string_to_range).collect();
            let range1 = &ranges[0];
            let range2 = &ranges[1];

            are_ranges_overlapping(range1, range2) || are_ranges_overlapping(range2, range1)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    #[test]
    fn test_solve_part_1() {
        let result = solve_part_1(SAMPLE_DATA.to_string());
        assert_eq!(result, 2);
    }

    #[test]
    fn test_solve_part_2() {
        let result = solve_part_2(SAMPLE_DATA.to_string());
        assert_eq!(result, 4);
    }
}
