use advent_of_code_2024::day10::day10_model::TrailMap;
use advent_of_code_2024::input:: parse_input_file;

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let mut input = TrailMap::new();
    let _ = parse_input_file("src/day10/input.txt",&mut input );
    let (part_one, part_two) = input.part_one_and_two();
    println!("Day 10 part 1 ");
    println!("{}", part_one);    
    println!("Day 10 part 2 ");
    println!("{}", part_two);
      Ok(())
}


