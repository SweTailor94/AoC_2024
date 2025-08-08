use advent_of_code_2024::day15::day15_model::{Warehouse, World};
use advent_of_code_2024::input::parse_input_file;

fn main() ->Result<(),Box<dyn std::error::Error>> {

    let mut input = Warehouse::new();
    let _ = parse_input_file("src/day15/input.txt", &mut input);
    println!("Day 15 part 1 ");
    println!("{}",input.part1());

    println!("Day 15 part 2 ");
      Ok(())
}

fn parse_input_line(line:&str) -> usize{
    line.len()
}
