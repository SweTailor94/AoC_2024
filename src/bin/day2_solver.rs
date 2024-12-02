use advent_of_code_2024::input::get_vector_from_file;

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let input = get_vector_from_file("src/day2/input.txt", parse_input_line);
    println!("Day 2 part 1 ");
    // println!("{:?}",input);
    let safe = input.iter().filter(|x|check_safe(x)).count();
    println!("{}",safe);
    println!("Day 2 part 2 ");
    let safe = input.iter().filter(|x|check_safe2(x)).count();
    println!("{}",safe);
      Ok(())
}

fn parse_input_line(line:&str) -> Vec<i64>{
   line.split_ascii_whitespace().map(|x|x.parse().unwrap()).collect()
}

fn check_safe(measures: &Vec<i64>) -> bool{
    if measures[0] > measures[1] {
        check_decreasing(measures)
    } else
    {
        check_increasing(measures)
    }
}

fn check_decreasing(p0: &Vec<i64>) -> bool {
    for i in 0..p0.len()-1 {
        let diff= p0[i] - p0[i+1];
        if diff < 1 || diff > 3 {return false;}
    }
    true
}

fn check_increasing(p0: &Vec<i64>) -> bool {
    for i in 0..p0.len()-1 {
        let diff= p0[i+1] - p0[i];
        if diff < 1 || diff > 3 {return false;}
    }
    true
}

fn check_safe2(p0: &Vec<i64>) -> bool {
    do_check(p0, false)
}

fn do_check(measures: &Vec<i64>, second_try: bool) -> bool {
    let res =  if measures[0] > measures[1] {
        check_decreasing(measures)
    } else
    {
        check_increasing(measures)
    };

    if second_try {
        return res;
    } else if res {
        return true;
    }

    for i in 0..measures.len() {
        let mut removed = Vec::new();
        removed.extend_from_slice(&measures[0..i]);
        removed.extend_from_slice(&measures[i+1..measures.len()]);
        if do_check(&removed,true) {
            return true;
        }
    }

    false
}