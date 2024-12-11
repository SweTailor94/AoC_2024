use advent_of_code_2024::day5::day5_model::PrintQueue;
use advent_of_code_2024::input::parse_input_file;

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let mut print_queue = PrintQueue::new();
    let _ = parse_input_file("src/day5/input.txt", &mut print_queue)?;
    println!("Day 5 part 1 ");
    println!("{}",print_queue.get_part_one());
    println!("Day 5 part 2 ");
    println!("{}", print_queue.get_part_two());
      Ok(())
}

