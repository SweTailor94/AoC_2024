use crate::input::InputParser;
use std::collections::HashMap;
use std::fmt::Display;
pub const  NO_REG: u64 = 0xFFFFFFFF_FFFFFFFF;
#[derive(Clone, Copy)]
pub struct Plant{
    spec: char,
    perimeter: u8,
    pub region: u64,
}
impl Plant {
    pub fn new(spec: char) -> Plant {
        Plant { spec, perimeter: 4 , region: NO_REG }
    }

    pub fn dec(&mut self) {
        self.perimeter = self.perimeter - 1;
    }
    pub fn char(&self) -> char {
        self.spec
    }
    pub fn perimeter(&self) -> u8 {
        self.perimeter
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
    pub fn part1(&mut self) -> u64 {
        let mut next_region = 1;
        for row in 0..self.rows {
            for col in 0..self.cols {
                //print!("({},{}) ", row, col);
                let mut region = NO_REG;
                if row > 0 && self.map[row][col] == self.map[row - 1][col] {
                    region = self.map[row - 1][col].region;
                }
                if col > 0 && self.map[row][col] == self.map[row][col - 1] {
                    region = region.min(self.map[row][col - 1].region);
                }
                if region == NO_REG {
                    self.map[row][col].region = next_region;
                    //print!("Set this to {}", next_region);
                    next_region += 1;
                } else {
                    self.map[row][col].region = region;
                }
                
                if row > 0 {
                    if self.map[row][col] == self.map[row - 1][col] {
                        self.map[row][col].dec();
                        self.map[row - 1][col].dec();
                        // print!("this {}, above {}. ",self.map[row][col].region, self.map[row - 1][col].region);
                        if self.map[row][col].region < self.map[row - 1][col].region {
                            let mut rev = row-1;
                            while self.map[rev][col] == self.map[row][col] {
                                // print!("Change ({},{}) to {}, ", rev,col,self.map[row][col].region );
                                self.map[rev][col].region = self.map[row][col].region;
                                if rev == 0 { break; }
                                rev -= 1;
                            }
                        } 
                    }
                }
                if col > 0  {
                    if self.map[row][col] == self.map[row][col - 1] {
                        self.map[row][col].dec();
                        self.map[row][col - 1].dec();
                        if self.map[row][col].region < self.map[row][col - 1].region {
                            let mut rev = col-1;
                            while  self.map[row][col] == self.map[row][rev]{
                                self.map[row][rev].region = self.map[row][col].region;
                                if rev == 0 { break; }
                                rev = rev - 1;
                            }
                        } 
                    }
                }
                          
                // println!();
            }
        }
        let mut regions : HashMap<u64, Vec<Plant>> = HashMap::new();
        for row in 0..self.rows {
            for col in 0..self.cols {
                regions.entry(self.map[row][col].region).or_insert(Vec::new()).push(self.map[row][col].clone());    
            }
        }
        // regions.iter().for_each(|(region, plants)|{
        //     let fence = plants.iter().fold(0, |acc, plant| acc + plant.perimeter as u64);
        //     // println!("{} {} {} + {} = {}", region, plants[0].spec, plants.len(), fence, plants.len() as u64 * fence );
        // });
        regions.iter().fold(0u64, |acc, (_,plants)| {
            acc + plants.len() as u64 * plants.iter().fold(0u64, |acc, plant| {
                acc + plant.perimeter as u64
            })
        })         
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
                print!("[{}|{}|{:5}] ",plant.perimeter , plant.char(), plant.region);
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