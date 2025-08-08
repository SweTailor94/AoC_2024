// model types for Day13

//A dx = C, dy= D
//B dx= E, dy = F

// X = a*C + b*E
// Y = a*D + b*F

// a = (X-b*E)/C
// b = (Y*C - X*D) / (F*C - E*D)

// To be able to win:
//  (F*C - E*D) != 0
// a integer >= 0
// b integer >= 0

use crate::input::InputParser;


#[derive(Copy,Clone,Debug)]
pub struct ClawMachine{
    pub x:i64,
    pub y:i64,
    pub c:i64,
    pub d:i64,
    pub e:i64,
    pub f:i64,
}
impl ClawMachine {
    pub fn new() -> Self{
        Self{
            x: 0,
            y: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
        }
    }

    pub fn winning(&self) -> Option<(u64,u64)> { // returns (a,b)
        // a = (X-b*E)/C
        // b = (Y*C - X*D) / (F*C - E*D)
        let n = self.f*self.c - self.e*self.d;
        if n == 0 {return None;}
        let t = self.y*self.c -self.x*self.d;
        if t % n != 0 {return None;}
        let b = t/n;
        if b < 0 {return None;}
        let at  = self.x-b*self.e;
        if at % self.c != 0 {return None;}
        let a = at/self.c;
        if a < 0 {None} else
        {Some((a as u64,b as u64))}
    }

}
pub struct Arcade{
    pub claw_machines: Vec<ClawMachine>,
    current: ClawMachine,
}

impl Arcade{
    pub fn new() -> Self {
        Self{
            claw_machines: vec![],
            current: ClawMachine::new(),
        }
    }

    pub fn get_min_tokens(&self) -> u64 {
        let mut _count = 1;
        let mut tokens: u64 = 0;
        for m in &self.claw_machines {
            match m.winning() {
                Some((a,b)) => {
                    let t = a*3+b;
                    tokens += t;
                    // println!("Claw machine #{} Winning for {} tokens.",count, t);
                }
                None => {}//println!("Claw machine #{} can't winn", count),
            }
            _count+=1;
        }
        tokens
    }
    pub fn adjust_prize_pos(&mut self) {
        for c in &mut self.claw_machines {
            c.x = 10000000000000 + c.x;
            c.y = 10000000000000 + c.y;
        }
    }
}
impl InputParser for Arcade{
    fn parse_line(&mut self, line: &String) -> anyhow::Result<()> {
        if line.is_empty() {
            self.claw_machines.push(self.current);
            self.current = ClawMachine::new();
        } else if line.starts_with("Button") {
            let first:Vec<&str> = line.split(":").collect();
            let button = first[0].chars().last().unwrap();
            let values =parse_values( first[1].split(",").collect());
            for (var, steps) in values {
                match (button,var)  {
                    ('A','X') => self.current.c = steps as i64,
                    ('A','Y') => self.current.d = steps as i64,
                    ('B','X') => self.current.e = steps as i64,
                    ('B','Y') => self.current.f = steps as i64,
                    _ => panic!("Wrong input in parsing line {}" , line),
                }
            }
        } else if line.starts_with("Prize") {
            let first:Vec<&str> = line.split(":").collect();
            let values = parse_values(first[1].split(",").collect());
            for (var, steps) in values {
                match var {
                    'X' => self.current.x = steps as i64,
                    'Y' => self.current.y = steps as i64,
                    _ => panic!("Wrong input in parsing line {}", line),
                }
            }
        }
        Ok(())
    }
}

fn parse_values(v:Vec<&str>) -> Vec<(char,u64)>{
    let mut result = Vec::new();
    for val in v{
        let var_val: Vec<&str> = val.trim().split(&['+','=']).collect();
        let c:char = var_val[0].chars().next().unwrap();
        let value = var_val[1].to_string().parse().unwrap_or(0u64);
        result.push((c,value));
    }
    result
}