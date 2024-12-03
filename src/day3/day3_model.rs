// model types for Day3

use regex::Regex;
use crate::input::InputParser;

pub struct ProgramParser{
    total_sum: i64,
    mul_enabled: bool,
    reg_exp: Regex,
}

impl ProgramParser{
    pub fn new() -> Self{
        Self {
            total_sum: 0,
            mul_enabled: true,
            reg_exp : Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap()
        }
    }
    pub fn get_sum(&self) -> i64 {
        self.total_sum
    }
}
impl InputParser for ProgramParser{
    fn parse_line(&mut self, line: &String) -> anyhow::Result<()> {
         for m in self.reg_exp.find_iter(line) {
            let x = m.as_str();
             match x {
                 "do()" => self.mul_enabled = true,
                 "don't()" => self.mul_enabled = false,
                 _ => if self.mul_enabled {
                     self.total_sum += x[4..x.len() - 1].split(',').map(|x| x.parse::<i64>().unwrap()).reduce(|prod, v| prod * v).unwrap();
                 }
             }
        };
        Ok(())
    }
}