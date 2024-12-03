use advent_of_code_2024::input::{get_vector_from_file, parse_input_file};
use regex::Regex;
use advent_of_code_2024::day3::day3_model::ProgramParser;

fn main() ->Result<(),Box<dyn std::error::Error>> {

    let input = get_vector_from_file("src/day3/input.txt", parse_input_line);
    println!("Day 3 part 1 ");
    let sum = input.into_iter().reduce(|acc,v|acc+v).unwrap();
    println!("{}",sum);
    println!("Day 3 part 2 ");
    let mut parser = ProgramParser::new();
    parse_input_file("src/day3/input.txt", &mut parser);
    println!("{}",parser.get_sum());

      Ok(())
}

fn parse_input_line(line:&str) -> i64{
    let reg_exp = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    reg_exp.find_iter(line).map(|m| {
        let x = m.as_str();
        return x[4..x.len() - 1].split(',').map(|x| x.parse::<i64>().unwrap()).reduce(|prod, v| prod * v).unwrap();
    }).reduce(|acc, v| acc + v).unwrap_or_else(|| 0)

}

