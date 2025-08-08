use crate::input::InputParser;
use std::collections::{VecDeque};
use itertools::Itertools;

pub const  NO_REG: u64 = 0xFFFFFFFF_FFFFFFFF;
#[derive(Clone, Copy,Debug)]
pub struct Plant{
    spec: char,
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
}


impl Plant {
    pub fn new(spec: char) -> Plant {
        Plant {
            spec,
            top: true,
            bottom: true,
            left: true,
            right: true,
        }
    }

    // pub fn dec(&mut self) {
    //     self.perimeter = self.perimeter - 1;
    // }
    pub fn char(&self) -> char {
        self.spec
    }
    pub fn perimeter(&self) -> u8 {
        let mut ans = if self.top {1} else {0} ;
        ans += if self.bottom {1}else{0};
        ans += if self.left {1}else{0};
        ans += if self.right {1}else{0};
        ans
    }

    pub fn no_top(&mut self){
        self.top = false;
    }
    pub fn no_bottom(&mut self) {
        self.bottom = false;
    }
    pub fn no_left(&mut self) {
        self.left = false;
    }
    pub fn no_right(&mut self) {
        self.right = false;
    }

    pub(crate) fn aligns_vertical(&self, p0: &Plant) -> u64 {

        let mut x = if self.left && p0.left {1} else {0};
        x += if self.right && p0.right {1}else{0};
        x
    }
    pub(crate) fn aligns_horizontal(&self, p0: &Plant) -> u64 {
        let mut x = if self.top && p0.top {1} else {0};
        x += if self.bottom && p0.bottom {1}else{0};
        x
    }
}

impl PartialEq for Plant {
    fn eq(&self, other: &Plant) -> bool {
        self.spec == other.spec
    }
}
impl From<&Plant> for u64{
    fn from(value: &Plant) -> Self {
        value.perimeter() as u64
    }
}

// model types for Day12
pub struct FenceAreas{
    pub rows: usize,
    pub cols: usize,
    pub map: Vec<Vec<Plant>>,
}

impl FenceAreas {
    pub fn part1(&mut self) -> (u64,u64) { // returns (Part1, Part2)
        let mut untoutched_squares: VecDeque<(usize,usize)> = VecDeque::new();
        for row in 0..self.rows {
            for col in 0..self.cols {
                untoutched_squares.push_back((row,col));
                if row > 0 {
                    if self.map[row][col] == self.map[row - 1][col] {
                        self.map[row][col].no_top();
                        self.map[row - 1][col].no_bottom();
                    }
                }
                if col > 0  {
                    if self.map[row][col] == self.map[row][col - 1] {
                        self.map[row][col].no_left();
                        self.map[row][col - 1].no_right();
                    }
                }
            }
        }
        let mut in_region: VecDeque<(usize,usize)> = VecDeque::new();
        let mut sum = 0;
        let mut p2 = 0;
        while untoutched_squares.len() > 0 {
            in_region.push_back(untoutched_squares.pop_front().unwrap());
            let mut perimeter = 0;
            let mut region_size = 0;
            let mut reduce_perimeter_part2 = 0;

            while in_region.len() > 0 {
                let (cur_row, cur_col) = in_region.pop_front().unwrap();
                let cur_plant = self.map[cur_row][cur_col];
                perimeter += cur_plant.perimeter() as u64;
                region_size += 1;

                // up
                if cur_row > 0 {
                    let next = (cur_row-1,cur_col);
                    let above = self.map[next.0][next.1];
                    if cur_plant == above {
                        reduce_perimeter_part2 +=  cur_plant.aligns_vertical(&above); // Try to find vertical sides upwards only
                        if let Some((index,_)) = untoutched_squares.iter().find_position(|p|p == &&next) {
                            untoutched_squares.remove(index);
                            in_region.push_back(next);
                        }
                    }
                }
                // down
                if cur_row < self.rows - 1 {
                    let next = (cur_row + 1,cur_col);
                    let under = self.map[next.0][next.1];
                    if cur_plant == under {
                        // reduce_perimeter_part2 += cur_plant.aligns_vertical(&under);
                        if let Some((index,_)) = untoutched_squares.iter().find_position(|p|p == &&next) {
                            untoutched_squares.remove(index);
                            in_region.push_back(next);
                        }
                    }
                }
                // left
                if cur_col > 0 {
                    let next = (cur_row,cur_col-1);
                    let left = self.map[next.0][next.1];
                    if cur_plant == left {
                        reduce_perimeter_part2 += cur_plant.aligns_horizontal(&left); // try to find horizontal sides to left only
                        if let Some((index,_)) = untoutched_squares.iter().find_position(|p|p == &&next) {
                            untoutched_squares.remove(index);
                            in_region.push_back(next);
                        }
                    }
                }
                // right
                if cur_col < self.cols - 1 {
                    let next = (cur_row,cur_col+1);
                    let right = self.map[next.0][next.1];
                    if cur_plant == right {
                        //reduce_perimeter_part2 += cur_plant.aligns_horizontal(&right);
                        if let Some((index,_)) = untoutched_squares.iter().find_position(|p|p == &&next) {
                            untoutched_squares.remove(index);
                            in_region.push_back(next);
                        }
                    }
                }
            }
            sum += region_size * perimeter;
            p2 += region_size*(perimeter-reduce_perimeter_part2)
        }


        (sum, p2)
    }
}

impl FenceAreas {
    pub fn new() -> Self{
        Self{
            rows: 0,
            cols: 0,
            map: Vec::new(),
        }
    }

    pub fn print(&self){
        println!("Fence Areas: rows {}, cols {}", self.rows, self.cols);
        for row in &self.map {
            for plant in row {
                let mut edges: String = String::new();
                edges.push(if plant.top{'T'}else{' '});
                edges.push(if plant.bottom{'B'}else{' '});
                edges.push(if plant.left{'L'}else{' '});
                edges.push(if plant.right{'R'}else{' '});
                print!("[{}|{:>4}] " ,plant.spec, edges);
            }
            println!("");
        }
        
    }
}

impl InputParser for FenceAreas {
    fn parse_line(&mut self, line: &String) -> anyhow::Result<()> {
        if line.len() == 0 {
            self.cols = self.map[0].len();
            return Ok(());
        }
        self.map.push(line.chars().map(|c|Plant::new(c)).collect());
        self.rows += 1;
        Ok(())
    }
}