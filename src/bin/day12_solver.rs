use advent_of_code_2024::day12::day12_model::FenceAreas;
use advent_of_code_2024::input::parse_input_file;

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let mut input = FenceAreas::new();
    let _ = parse_input_file ("src/day12/input.txt", &mut input);
    println!("Day 12 part 1 ");
   
    println!("{}",input.part1());
    
    input.print();
    //println!("Day 12 part 2 ");
      Ok(())
}

