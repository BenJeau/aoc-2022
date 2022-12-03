use day1::solve_part_2;
use utils::read_file_vec;

fn main() {
    let file_content = read_file_vec("input").unwrap();
    let answer = solve_part_2(file_content);

    println!("Answer part 2: {answer}");
}
