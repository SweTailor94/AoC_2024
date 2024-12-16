// model types for Day8

use std::collections::{HashMap, HashSet};
use crate::input::InputParser;

#[derive(Debug)]
pub struct AntennaMap{
    pub antennas: HashMap<u8, Vec<(i32,i32)>>, // key = frequency, val = Vec (row, col) 
    pub antinodes: HashSet<(i32,i32)>,
    rows: i32,
    cols: i32,
}

impl AntennaMap {
    pub fn new() -> Self {
        Self {
            antennas: HashMap::new(),
            antinodes: HashSet::new(),
            rows: 0,
            cols: 0,
        }
    }

    pub fn part_one(&mut self) -> i32 {
        for item in self.antennas.values() {
            let count = item.len();
            for i in 0..count - 1 {
                for j in i + 1..count {
                    let (row_a, col_a) = item[i];
                    let (row_b, col_b) = item[j];
                    let delta_row = row_a - row_b;
                    let delta_col = col_a - col_b;
                    let p_a = (row_a + delta_row, col_a + delta_col);
                    let p_b = (row_b - delta_row, col_b - delta_col);
                    if self.is_in_grid(p_a) {
                        self.antinodes.insert(p_a);
                    }
                    if self.is_in_grid(p_b) {
                        self.antinodes.insert(p_b);
                    }
                }
            }
        }
        self.antinodes.len() as i32
    }
    pub fn part_two(&mut self) -> i32 {
        for item in self.antennas.values() {
            let count = item.len();
            for i in 0..count - 1 {
                for j in i + 1..count {
                    let (row_a, col_a) = item[i];
                    let (row_b, col_b) = item[j];
                    let delta_row = row_a - row_b;
                    let delta_col = col_a - col_b;
                    self.antinodes.insert(item[i]);
                    self.antinodes.insert(item[j]);
                    
                    let mut p_a = (row_a + delta_row, col_a + delta_col);
                    loop {
                        if self.is_in_grid(p_a) {
                            self.antinodes.insert(p_a);
                        } else {
                            break;
                        }
                        p_a = (p_a.0 + delta_row, p_a.1 + delta_col);
                    }
                    let mut p_b = (row_b - delta_row, col_b - delta_col);
                    loop {
                        if self.is_in_grid(p_b) {
                            self.antinodes.insert(p_b);
                        } else {
                            break;
                        }
                        p_b = (p_b.0 - delta_row, p_b.1 - delta_col);
                    }
                }
            }
        }
        self.antinodes.len() as i32
    }
    fn is_in_grid(&self, p: (i32, i32)) -> bool {
        p.0 >= 0 && p.0 < self.rows && p.1 >= 0 && p.1 < self.cols
    }
}
impl InputParser for AntennaMap {
    fn parse_line(&mut self, line: &String) -> anyhow::Result<()> {
        if line.len() == 0 {return Ok(());}
        let dot = ".".as_bytes()[0];
        self.cols = line.len() as i32;
        for (col, element) in line.as_bytes().iter().enumerate() {
            if *element != dot {
                self.antennas.entry(*element as u8).or_insert(Vec::new()).push((self.rows,col as i32));
            }
        }
        self.rows += 1;
        Ok(())
    }
}
