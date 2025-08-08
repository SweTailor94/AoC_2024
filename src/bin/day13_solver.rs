use advent_of_code_2024::day13::day13_model::Arcade;
use advent_of_code_2024::input::{get_vector_from_file, parse_input_file};

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let mut input = Arcade::new();
        parse_input_file("src/day13/input.txt", &mut input);
    println!("Day 13 part 1 ");
    // for c in &input.claw_machines {
    //     println!("{:?}", c);
    // }
    println!("Part1 {}", input.get_min_tokens());
    println!("Day 13 part 2 ");
    input.adjust_prize_pos();
    println!("Part2 {}", input.get_min_tokens());
      Ok(())
}

fn parse_input_line(line:&str) -> usize{
    line.len()
}
