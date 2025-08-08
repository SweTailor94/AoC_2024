// model types for Day16

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use crate::input::InputParser;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self{
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

pub struct Maze{
    start: (u64,u64), // (row,col)
    end: (u64,u64),
    rows: u64,
    cols:u64,
    walls: HashSet<(u64,u64)>,
}

struct Pos {
    pub cost: u64,
    pub dir: Direction,
    pub pos: (u64,u64),
}
impl Eq for Pos {
}
impl PartialEq<Self> for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}
impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}
impl Maze{
    pub fn new() -> Self{

        Self {
            start: (0,0),
            end: (0,0),
            rows: 0,
            cols: 0,
            walls: Default::default(),
        }
    }


    pub fn part1(&mut self) -> u64 {
        let mut lowest_cost_at_end = u64::MAX;
        let mut crossroads = HashMap::new();
        crossroads.insert(self.start,0u64);
        let mut next_to_explore: BinaryHeap<Pos> = BinaryHeap::new();
        next_to_explore.push(Pos{
            cost: 0,
            dir: Direction::East,
            pos: self.start,
        });

        while let Some(cur) = next_to_explore.pop() {
            println!("Next: at {:?} going {:?}",cur.pos, cur.dir);
            match crossroads.get(&cur.pos) {
                None => {}
                Some(c) => {if cur.cost > *c && cur.cost.abs_diff(*c) > 1000  {continue;};}
            };
            let possible = self.get_possible_directions(cur.pos);
            let back = cur.dir.opposite();

            // try each direction until dead end or crossroad, adding cost.
            for dir in possible {
                if dir == back { continue;}
                println!("Try go {:?}", dir);
                let mut cost = cur.cost;
                let (mut row, mut col) = cur.pos;
                let mut cur_dir = if cur.dir == dir {dir} else {
                    cost += 1000;
                    dir
                };

                loop {
                    println!("at {:?} going {:?}", (row, col), cur_dir);
                    match cur_dir {
                        Direction::North => {row -= 1;}
                        Direction::South => {row += 1;}
                        Direction::East => {col += 1;}
                        Direction::West => {col -= 1;}
                    };
                    cost += 1;
                    if (row,col) == self.end {
                        lowest_cost_at_end = lowest_cost_at_end.min(cost);
                        break;
                    } else {
                        let directions = self.get_possible_directions((row, col));
                        if directions.len() >= 3 {
                            // New crossroad
                            let result = crossroads.get_mut(&(row,col));
                            match result {
                                None => {
                                    crossroads.insert((row,col), cost);
                                    next_to_explore.push(Pos {
                                        cost,
                                        dir: cur_dir,
                                        pos: (row, col),
                                    });
                                }
                                Some(&mut ref mut c) => {
                                    let old_val = *c;
                                    if cost < *c {
                                        *c = cost;
                                    }
                                    if  cost < old_val || cost.abs_diff(old_val) < 1000 {
                                        next_to_explore.push(Pos {
                                            cost,
                                            dir: cur_dir,
                                            pos: (row, col),
                                        });
                                    }
                                }
                            }
                            break;
                        } else {
                            if directions.len() == 1 {
                                // Dead end
                                break;
                            }
                            let new_dir = directions.iter().find(|x| **x != cur_dir.opposite()).unwrap();
                            if *new_dir != cur_dir {
                                cost += 1000;
                                cur_dir = *new_dir;
                            }
                        }
                    }
                    // Now take a step again
                } // until end or crossroad
            }
        }
        lowest_cost_at_end
    }

    fn get_possible_directions (&self, pos:(u64,u64)) -> Vec<Direction> {
        let (row, col) = pos;
        let mut v = Vec::new();
        if !self.walls.contains(&(row-1,col)) {v.push(Direction::North)};
        if !self.walls.contains(&(row+1, col)) {v.push(Direction::South)};
        if !self.walls.contains(&(row, col+1)) {v.push(Direction::East)};
        if !self.walls.contains(&(row, col-1)) {v.push(Direction::West)};
        v
    }
    pub fn print(&self) {
        println!("rows {}, cols {}", self.rows, self.cols);
        println!("Start {:?}", self.start);
        println!("End {:?}", self.end);
    }
}
const WALL: u8 = 0x23; // '#'
const VOID: u8 = 0x2E; // '.'
const START: u8 = 0x53; // 'S'
const END: u8 = 0x45; // 'E'
impl InputParser for Maze{
    fn parse_line(&mut self, line: &String) -> anyhow::Result<()> {
        if line.is_empty() {return Ok(());}


        let line_as_u8 = line.as_bytes();
        for (index, c) in line_as_u8.iter().enumerate() {
            match *c {
                WALL => {self.walls.insert((self.rows, index as u64));}
                END => {self.end = (self.rows, index as u64);}
                START => {self.start = (self.rows, index as u64);}
                VOID => {}
                _ => panic!("Unknown character"),
            };
        }
        self.rows += 1;
        self.cols = line_as_u8.len() as u64;
        Ok(())
    }
}