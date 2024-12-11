// model types for Day5

use std::collections::{HashMap, HashSet, VecDeque};
use itertools::Itertools;
use crate::input::InputParser;
#[derive(Debug)]
pub struct PrintQueue{
    rules: HashMap<u8,HashSet<u8>>,
    prints: Vec<Vec<u8>>,
    all_pages: HashSet<u8>,
    reading_prints: bool,
}

impl PrintQueue {
    pub fn get_part_two(&self) -> u32 {
        let bad = self.prints.iter()
            .filter(|p| matches!(self.print_is_ok(p),None))
            .map(|p|p.clone())
            .collect::<Vec<Vec<u8>>>();
        let mut sum = 0;
        for b in bad {
            sum += self.make_print_ok(b);
        }
        sum
    }

    fn make_print_ok(&self, p: Vec<u8>) -> u32 {
        let mut candidates = VecDeque::new();
        let mut new_print = Vec::new();
        p.iter().for_each(|p|candidates.push_back(*p));
        while candidates.len() > 0 {
            let cur = candidates.pop_front().unwrap();
            if self.less_than_rest(cur, &candidates) {
                new_print.push(cur);
            } else {
                candidates.push_back(cur);
            }
        }
        new_print[(new_print.len()-1)/2] as u32
    }
    
    fn less_than_rest(&self, val:u8, rest: &VecDeque<u8>) -> bool {
        let empty_set = HashSet::new();
        let after = match self.rules.get(&val) {
            None => &empty_set,
            Some(set) => set,
        };
        for j in rest.iter() {
            if ! after.contains(j) {
                return false;
            }
        } 
        true
    }

    pub fn new() -> Self {
        Self{
            rules: HashMap::new(),
            prints: Vec::new(),
            reading_prints: false,
            all_pages: HashSet::new(),
        }
    }

    pub fn get_part_one(&mut self) -> u32 {

        let mut sum = 0;
        for p in self.prints.iter() {
            if let Some(value) = self.print_is_ok(p){
                sum += value as u32;
            }
        }
        sum
    }

    fn print_is_ok(&self, print: &Vec<u8>) -> Option<u8>{
        let stop = print.len()-1;
        let empty_set = HashSet::new();
        for i in 0.. stop {
            let after = match self.rules.get(&print[i]) {
                None => &empty_set,
                Some(set) => set,
            };
            for j in i+1..stop+1 {
                if ! after.contains(&print[j]) {
                    return None;
                }
            }
        }
        Some(print[stop/2])
    }
}

impl InputParser for PrintQueue {
    fn parse_line(&mut self, line: &String) -> anyhow::Result<()> {
        if line.is_empty() {
            self.reading_prints = true;
            return Ok(());
        }
        if self.reading_prints {
            self.prints.push(line.split(',').map(|p|p.parse::<u8>().unwrap()).collect_vec());
        } else {
            let vals: Vec<u8> =
            line.split("|").map(|v|v.parse::<u8>().unwrap()).collect();
            for v in &vals{
                self.all_pages.insert(*v);
            }
            self.rules.entry(vals[0]).or_insert(HashSet::new()).insert(vals[1]);
        }
        Ok(())
    }
}