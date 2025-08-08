use advent_of_code_2024::day16::day16_model::Maze;
use advent_of_code_2024::input:: parse_input_file;

fn main() ->Result<(),Box<dyn std::error::Error>> {
    // let chars = "#.SE".as_bytes();
    // println!("{:02X?}", chars);

    let mut input = Maze::new();
    let _ = parse_input_file("src/day16/input.txt", &mut input);
    println!("Day 16 part 1 ");
    input.print();
    println!("Lowest Cost {}", input.part1() );
    println!("Day 16 part 2 ");
      Ok(())
}

