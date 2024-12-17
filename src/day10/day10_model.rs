// model types for Day10 

use std::collections::{HashMap, HashSet};
use crate::input::InputParser;

#[derive(Debug)]
pub struct TrailMap{ 
    map: Vec<Vec<u8>>,
    heads : HashSet<(usize,usize)>, // (roe,col)
    rows : usize,
    cols : usize,
}

impl TrailMap {
    pub fn new() -> Self {
        Self{
            map: Vec::new(),
            heads: HashSet::new(),
            rows : 0,
            cols : 0,
        }
    }
    
    pub fn part_one_and_two(&mut self) -> (u32, u32) {
        let mut score_sum = 0;
        let mut rating_sum=0;
        self.heads.iter().for_each(|&pos|{
            let (points, rating) = self.find_trail_point_and_rating(pos);
            score_sum += points;
            rating_sum += rating;
        });
        (score_sum, rating_sum)
    }
    
    pub fn find_trail_point_and_rating(& self, start:(usize,usize )) -> (u32, u32){
        let mut visited = HashMap::new();
        self.try_next(start, 0, &mut visited);
        (visited.keys().len() as u32, visited.values().sum())
    }
    
    pub fn try_next(&self,pos:(usize,usize ), level: u8, tops: &mut HashMap<(usize,usize),u32>)   {
        let (row,col) = pos;
        if row > 0 && self.map[row-1][col] == level+1 { 
            if level+1 == 9 {
               *tops.entry((row-1,col)).or_insert(0) += 1 ;
            } else {
                self.try_next((row - 1, col), level + 1, tops);
            }
        }  
        if row < self.rows-1 && self.map[row+1][col] == level+1 {
            if level+1 == 9 {
                *tops.entry((row+1,col)).or_insert(0) += 1 ;
            } else { 
                self.try_next((row+1, col), level + 1, tops); 
            }
        }
        if col > 0 && self.map[row][col-1] == level+1 {
            if level+1 == 9 {
                *tops.entry((row,col-1)).or_insert(0) += 1 ;
            } else {
                self.try_next((row,col-1), level + 1, tops); }
        }
        if col < self.cols-1 && self.map[row][col+1] == level+1 {
            if level+1 == 9 {
                *tops.entry((row,col+1)).or_insert(0) += 1 ;
            } else {
                self.try_next((row,col+1), level + 1, tops); 
            }
        }
    }
}

impl InputParser for TrailMap {
    fn parse_line(&mut self, line: &String) -> anyhow::Result<()> {
        if line.len() == 0 { 
            self.cols = self.map[0].len();
            return Ok(()); 
        }
        self.map.push( line.chars().enumerate().map(|(i,cs)| {
             let v: u8 = cs as u8 - b'0';
             if v == 0 {self.heads.insert((self.rows,i));} 
             return v;
         }).collect() );
        self.rows += 1;
        Ok(())
    }
}