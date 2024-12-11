use advent_of_code_2024::input::get_vector_from_file;

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let input = get_vector_from_file("src/day10/input.txt", parse_input_line);
    println!("Day 10 part 1 ");
    println!("Day 10 part 2 ");
      Ok(())
}

fn parse_input_line(line:&str) -> usize{
    line.len()
}
