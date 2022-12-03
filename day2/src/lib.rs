use std::cmp::Ordering;

pub fn read_file_vec(filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path: std::path::PathBuf = [env!("CARGO_MANIFEST_DIR"), filename].into_iter().collect();
    Ok(std::fs::read_to_string(path)?)
}

#[derive(Eq, PartialEq)]
enum Object {
    Rock,
    Paper,
    Scissor,
}

fn convert_char_to_object(char: &str) -> Option<Object> {
    match char {
        "A" | "X" => Some(Object::Rock),
        "B" | "Y" => Some(Object::Paper),
        "C" | "Z" => Some(Object::Scissor),
        _ => None,
    }
}

fn convert_object_to_value(object: &Object) -> usize {
    match object {
        Object::Rock => 1,
        Object::Paper => 2,
        Object::Scissor => 3,
    }
}

fn value_of_round(player1: &Object, player2: &Object) -> usize {
    match player1.cmp(player2) {
        Ordering::Less => 6,
        Ordering::Equal => 3,
        Ordering::Greater => 0,
    }
}

fn total_score(player1: &Object, player2: &Object) -> usize {
    convert_object_to_value(&player2) + value_of_round(&player1, &player2)
}

impl Ord for Object {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Object::Rock => match other {
                Object::Rock => Ordering::Equal,
                Object::Paper => Ordering::Less,
                Object::Scissor => Ordering::Greater,
            },
            Object::Paper => match other {
                Object::Rock => Ordering::Greater,
                Object::Paper => Ordering::Equal,
                Object::Scissor => Ordering::Less,
            },
            Object::Scissor => match other {
                Object::Rock => Ordering::Less,
                Object::Paper => Ordering::Greater,
                Object::Scissor => Ordering::Equal,
            },
        }
    }
}

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn solve_part_1(file_content: String) -> usize {
    file_content
        .split("\n")
        .map(|value| {
            let objects = value
                .split(" ")
                .map(convert_char_to_object)
                .flatten()
                .collect::<Vec<Object>>();
            total_score(&objects[0], &objects[1])
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
