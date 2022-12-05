use day5::{read_file_vec, solve_part_1};

fn main() {
    let file_content = read_file_vec("input").unwrap();
    let answer = solve_part_1(file_content);

    println!("Answer part 1: {answer}");
}
