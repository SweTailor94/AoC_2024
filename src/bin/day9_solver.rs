use advent_of_code_2024::day9::day9_model::Haddrive;
use advent_of_code_2024::input::{ parse_input_file};

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let mut c = Haddrive::new();
    let _ = parse_input_file("src/day9/input.txt", &mut c);
    println!("Day 9 part 1 ");
    println!("{}",c.compact());
    println!("Day 9 part 2 ");
    println!("{}", c.compact2());
      Ok(())
}


