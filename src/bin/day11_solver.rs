use std::collections::HashMap;
use num::Integer;
use advent_of_code_2024::input::get_vector_from_file;

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let mut input = get_vector_from_file("src/day11/input.txt", parse_input_line);
    let mut stones : HashMap<String,u64> = HashMap::new();
    for (_, s) in input[0].iter() {
        *stones.entry(s.to_string()).or_insert(0) += 1;
    }
    println!("{:?}", input[0]); 
    for _  in 0..25 {
        stones = mutate2(&mut stones);
    }
    println!("Day 11 part 1 ");
    println!("{}", stones.values().sum::<u64>());
    println!("Day 11 part 2 ");
    for _  in 0..50 {
        stones = mutate2(&mut stones);
    }
    println!("{}", stones.values().sum::<u64>());
      Ok(())
}

fn mutate2(stones: &mut HashMap<String, u64>) -> HashMap<String,u64> {
    let mut new_stones = HashMap::new();
    stones.iter_mut().for_each(|(s,count)| {
        if s.as_str() == "0" {
            *new_stones.entry("1".to_string()).or_insert(0) += *count;
        } else if s.len().mod_floor(&2)  == 0  {
            let len = s.len()/2;
            let s1 = s[0..len].to_string();
            let s2= format!("{}",s[len..].parse::<u64>().unwrap()).to_string();
            *new_stones.entry(s1).or_insert(0) += *count;
            *new_stones.entry(s2).or_insert(0) += *count;
        } else {
            let val = format!("{}",s.parse::<u64>().unwrap() * 2024).to_string();
            *new_stones.entry(val).or_insert(0) += *count;
        }
    });
    new_stones
}

/// This naive solution Works for part 1.
/// Indexes are used to keep order as the problem stated that stones keep their order. 
/// (I thought it would be important in part 2, wrong guess :-) 
fn mutate(stones: &mut Vec<(usize,String)>)  {
    let mut new_stones = Vec::new();
    let mut index_move = 0;
    stones.iter_mut().for_each(|(index,s)| {
        if s.as_str() == "0" {
            *index += index_move;
            *s = "1".to_string();
        } else if s.len().mod_floor(&2)  == 0  {
            let len = s.len()/2;
            let s1 = s[0..len].to_string();
            let s2= format!("{}",s[len..].parse::<u64>().unwrap()).to_string();
            *s = s1;
            *index += index_move;
            index_move += 1;
            new_stones.push((*index+index_move,s2));
        } else {
            *s = format!("{}",s.parse::<u64>().unwrap() * 2024).to_string();
            *index += index_move;
        }
    });
    stones.extend_from_slice(&new_stones);
}

fn parse_input_line(line:&str) -> Vec<(usize,String)> {
    line.split_whitespace().enumerate().map(|(index,m)|(index, m.to_string())).collect::<Vec<(usize, String)>>()
}
