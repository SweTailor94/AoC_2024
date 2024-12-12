use advent_of_code_2024::input::get_vector_from_file;

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let input = get_vector_from_file("src/day7/input.txt", parse_input_line);
    println!("Day 7 part 1 ");
    println!("{}", part_one(&input));
    println!("Day 7 part 2 ");
    println!("{}", part_two(&input));
      Ok(())
}

fn parse_input_line(line:&str) -> (u64,Vec<u64>) {
    println!("Parse input line: {}", line);
    let res_parts= line.split(':').map(|s|s.trim_ascii()).collect::<Vec<&str>>();
    let ans = res_parts[0].parse::<u64>().unwrap();
    let terms = res_parts[1].split_ascii_whitespace().map(|v|v.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    (ans,terms)
}

fn part_one(input: &Vec<(u64,Vec<u64>)>) -> u64{
    let mut ans = 0;
    for eq in input {
        if can_solve(&eq) {
            ans += eq.0;
        }
    }
    ans
}

fn part_two(input: &Vec<(u64,Vec<u64>)>) -> u64{
    let mut ans = 0;
    for eq in input {
        if can_solve(&eq) {
            ans += eq.0;
        } else if can_solve2(&eq) {
            ans += eq.0;
        }
    }
    ans
}

fn can_solve2(eq: &&(u64, Vec<u64>)) -> bool {
    let (ans, terms) = eq;
    let mut ops = Vec::new();
    for c in 0..terms.len()-1 {
        ops.push(Op::Add)
    };
    
    for i in 0..3u32.pow(ops.len() as u32) {
        let mut carry = true;
        let mut index = 0;
        let mut new_op ;
        while carry && index < ops.len()  {
            (new_op, carry) = next_op(ops[index]);
            ops[index] = new_op;
            index += 1;
        }
        if *ans == eval(terms,&ops) {
            return true;
        }
    }
    false
}
fn next_op(o: Op) -> (Op, bool) {
    match o {
        Op::Add => (Op::Mul, false),
        Op::Mul => (Op::Concat, false),
        Op::Concat => (Op::Add, true),
    }
}
#[derive(Copy,Clone,PartialEq,Eq,Debug)]
enum Op {
    Add,
    Mul,
    Concat,
}

fn can_solve(eq: &&(u64, Vec<u64>)) -> bool {
    let (ans, terms) = eq;
    let mut ops = Vec::new();
    for c in 0..terms.len()-1 { 
        ops.push(Op::Add) 
    };
   
    for i in 0..2u32.pow(ops.len() as u32) {
        let mut ops_mut = ops.clone();
        let mut index = 0;
        let mut bit = 1;
        for count in 0..ops_mut.len() {
            if i & bit == bit {
                ops_mut[index] = Op::Mul; 
            }
            index += 1;
            bit  = bit << 1;
        }
        if *ans == eval(terms,&ops_mut) {
            return true;
        }
    }
    false
}

fn eval(terms: &Vec<u64>, ops: &Vec<Op>) -> u64 {
    if ops.len() != terms.len() -1 {
        panic!("Wrong number of operators");
    }
    let mut ans = terms[0];
    for i in 1..terms.len() {
        match ops[i-1] {
            Op::Add => ans += terms[i],
            Op::Mul => ans *= terms[i],
            Op::Concat => ans = concat(ans, terms[i]),
        } 
    }
    ans
}

fn concat(a:u64, b:u64) -> u64 {
    let mut bb = b;
    let mut aa = a;
    while bb > 0{
        aa = aa * 10;
        bb = bb / 10;
    }
    aa+b
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_concat() {
        let a = 12;
        let b = 128;
        assert_eq!(concat(a, b), 12128);
    }
}