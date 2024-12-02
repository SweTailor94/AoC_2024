use std::collections::HashMap;
use advent_of_code_2024::input::get_vector_from_file;

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let input = get_vector_from_file("src/day1/input.txt", parse_input_line);
    let (mut left,mut right): (Vec<u32>, Vec<u32>) = input.into_iter().unzip();
    left.sort();
    right.sort();
    let mut sum: u32=0;
    for i in 0..left.len(){
        sum += left[i].abs_diff(right[i]);
    }
    println!("Day 1 part 1 ");
    println!("{}",sum);
    println!("Day 1 part 2 ");
    let right= right.into_iter().fold(HashMap::new(), |mut map, val|{
        map.entry(val)
            .and_modify(|frq|*frq+=1)
            .or_insert(1u32);
        map
    });


    let mut sim = 0;
    for l in left{
        match right.get(&l){
            None => {}
            Some(count) => sim += l*count,
        }
    }
    println!("{}",sim);
      Ok(())
}

fn parse_input_line(line:&str) -> (u32,u32){
    let values: Vec<u32> = line.split_ascii_whitespace().map(|x|x.parse().unwrap()).collect();
    (values[0],values[1])
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse() {
        let inp= "88276   66757";
        let(a,b) = parse_input_line(inp);
        assert_eq!(a, 88276);
        assert_eq!(b, 66757);
    }
}