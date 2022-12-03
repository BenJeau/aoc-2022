use day1::solve_part_1;
use utils::read_file_vec;

fn main() {
    let file_content = read_file_vec("input").unwrap();
    let answer = solve_part_1(file_content);

    println!("Answer part 1: {answer}");
}
