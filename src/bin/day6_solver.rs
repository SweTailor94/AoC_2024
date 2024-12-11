use std::collections::HashSet;
use advent_of_code_2024::input::get_vector_from_file;

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let input = get_vector_from_file("src/day6/input.txt", parse_input_line);
    println!("Day 6 part 1 ");
    let path = solve_part_one(input.clone());
    println!("{}",path.len() );
    println!("Day 6 part 2 ");
    println!("{}",solve_part_two(input.clone(), path));
      Ok(())
}

fn parse_input_line(line:&str) -> Vec<u8>{
    line.as_bytes().to_vec()
}
#[derive(Clone, Copy)]
#[derive(Eq, Hash, PartialEq)]
enum Direction  {
    Left,
    Right,
    Up,
    Down,
}

fn turn(cur: Direction) -> Direction {
    match cur {
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
    }
}
enum TryMove{
    Ok((usize, usize)),
    No,
    Out,
}

fn solve_part_one(mut lab: Vec<Vec<u8>>) -> HashSet<(usize,usize)> {
    let space_obst_guard = ".#^".as_bytes();
    let cols = lab[0].len();
    let rows = lab.len();
    let mut visited_positions:HashSet<(usize, usize)> = HashSet::new();
    let mut cur: (usize, usize) = find_start(&lab,space_obst_guard[2]);
    let mut in_lab: bool = true;
    visited_positions.insert(cur);
    lab[cur.0][cur.1] = space_obst_guard[0];
    let mut dir = Direction::Up;
    
    let mut can_move = | cur: &(usize,usize) , dir: &Direction | 
         match dir {
            Direction::Up => {
                if cur.0 == 0 {
                    in_lab = false;
                    return TryMove::Out;
                }
                if lab[cur.0 - 1][cur.1] == space_obst_guard[0] {
                    TryMove::Ok((cur.0 - 1, cur.1))
                } else {
                    TryMove::No
                }
            }
            Direction::Down => {
                if cur.0 + 1 >= rows {
                    in_lab = false;
                    return TryMove::Out;
                }
                if lab[cur.0 + 1][cur.1] == space_obst_guard[0] {
                    TryMove::Ok((cur.0 + 1, cur.1))
                } else {
                    TryMove::No
                }
            }
            Direction::Left => {
                if cur.1 == 0 {
                    in_lab = false;
                    return TryMove::Out;
                }
                if lab[cur.0][cur.1 - 1] == space_obst_guard[0] {
                    TryMove::Ok((cur.0, cur.1 - 1))
                } else {
                    TryMove::No
                }
            }
            Direction::Right => {
                if cur.1 + 1 >= cols {
                    in_lab = false;
                    return TryMove::Out;
                }
                if lab[cur.0][cur.1 + 1] == space_obst_guard[0] {
                    TryMove::Ok((cur.0, cur.1 + 1))
                } else {
                    TryMove::No
                }
            }
        }
    ;
    
    loop {
        match can_move(&cur, &dir) {
            TryMove::Ok(new_pos ) => {
                visited_positions.insert(new_pos);
                cur = new_pos;
            }
            TryMove::No => {
                dir = turn(dir);
            }
            TryMove::Out => break,
        }
    }
    visited_positions
}

fn solve_part_two(mut lab: Vec<Vec<u8>>, guard_route: HashSet<(usize,usize)>) -> u32 {
    let space_obst_guard = ".#^".as_bytes();
    let cols = lab[0].len();
    let rows = lab.len();
   
    let start: (usize, usize) = find_start(&lab,space_obst_guard[2]);
   
    lab[start.0][start.1] = space_obst_guard[0];
    let mut dir = Direction::Up;

    let can_move = | cur: &(usize,usize) , dir: &Direction , lab: &Vec<Vec<u8>>|
        match dir {
            Direction::Up => {
                if cur.0 == 0 {
                    return TryMove::Out;
                }
                if lab[cur.0 - 1][cur.1] == space_obst_guard[0] {
                    TryMove::Ok((cur.0 - 1, cur.1))
                } else {
                    TryMove::No
                }
            }
            Direction::Down => {
                if cur.0 + 1 >= rows {
                    return TryMove::Out;
                }
                if lab[cur.0 + 1][cur.1] == space_obst_guard[0] {
                    TryMove::Ok((cur.0 + 1, cur.1))
                } else {
                    TryMove::No
                }
            }
            Direction::Left => {
                if cur.1 == 0 {
                    return TryMove::Out;
                }
                if lab[cur.0][cur.1 - 1] == space_obst_guard[0] {
                    TryMove::Ok((cur.0, cur.1 - 1))
                } else {
                    TryMove::No
                }
            }
            Direction::Right => {
                if cur.1 + 1 >= cols {
                    return TryMove::Out;
                }
                if lab[cur.0][cur.1 + 1] == space_obst_guard[0] {
                    TryMove::Ok((cur.0, cur.1 + 1))
                } else {
                    TryMove::No
                }
            }
        }
        ;

    let mut try_these: HashSet<(usize,usize)> = HashSet::new();
    let mut possible = 0u32;
    for (row,col) in guard_route {
        if (row, col) == start { 
            continue;
        }
        try_these.insert((row,col));
        if row != 0 && lab[row - 1][col] == space_obst_guard[0] {
            try_these.insert((row -1 ,col));
        }
        if row < rows && lab[row + 1][col] == space_obst_guard[0] {
            try_these.insert((row + 1, col));
        }
        if col != 0 && lab[row][col - 1] == space_obst_guard[0] {
            try_these.insert((row, col - 1));
        }
        if col < cols && lab[row][col + 1] == space_obst_guard[0] {
            try_these.insert((row, col + 1));
        }
    }
    for (test_row, test_col) in try_these {
        let mut visited_positions:HashSet<((usize, usize), Direction)> = HashSet::new();
        let mut cur =  start;
        dir = Direction::Up;
        visited_positions.insert((cur, Direction::Up));
        lab[test_row][test_col] = space_obst_guard[1];
        let mut found_loop = false;
        loop {
            match can_move(&cur, &dir, &lab) {
                TryMove::Ok(new_pos) => {
                    if !visited_positions.insert((new_pos, dir)) {
                        // same position in same dir means in a loop.
                        found_loop = true;
                        break;
                    }
                    cur = new_pos;
                }
                TryMove::No => {
                    dir = turn(dir);
                }
                TryMove::Out => break,
            }
        }
        if found_loop {
            possible += 1;
        }
        lab[test_row][test_col] = space_obst_guard[0];

    }
    possible
}
fn find_start(lab: &Vec<Vec<u8>>, val: u8) -> (usize, usize) {
    let cols = lab[0].len();
    let rows = lab.len();
    for row in 0..rows {
        for col in 0..cols {
            if lab[row][col] == val {
                return (row,col);
            }
        }
    }
    panic!("There must be a start");
}