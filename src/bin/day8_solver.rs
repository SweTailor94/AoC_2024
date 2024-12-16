use advent_of_code_2024::day8::day8_model::AntennaMap;
use advent_of_code_2024::input::{get_vector_from_file, parse_input_file};

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let mut input = AntennaMap::new();
    let _ = parse_input_file("src/day8/input.txt", &mut input);
    println!("Day 8 part 1 ");
    println!("{}",input.part_one());
    println!("Day 8 part 2 ");
    println!("{}",input.part_two());
      Ok(())
}

fn parse_input_line(line:&str) -> usize{
    line.len()
}
