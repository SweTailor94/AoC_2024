use advent_of_code_2024::day14::day14_model::Hall;
use advent_of_code_2024::input::{ parse_input_file};

fn main() ->Result<(),Box<dyn std::error::Error>> {

    let mut input = Hall::new(101,103);
    // let mut input = Hall::new(11,7);
    parse_input_file("src/day14/input.txt", &mut input)?;
    // input.print_robots();
    println!("Day 14 part 1 ");
    println!("{}",input.part1(100));
    println!("Day 14 part 2 ");
    println!("{}",input.part2());
      Ok(())
}


