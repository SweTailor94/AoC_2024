// model types for Day14

use std::collections::{HashMap, HashSet};
use crate::input::InputParser;


pub struct Hall {
    x_count: i64,
    y_count: i64,
    robots: Vec<Robot>,
}

impl Hall {
    pub fn print_robots(&self) {
        for r in &self.robots {
            r.print();
        }
    }
    pub fn new(x_size: i64, y_size: i64) -> Self {
        Self {
            robots: Vec::new(),
            x_count: x_size,
            y_count: y_size,
        }
    }

    pub fn part1(&self, n: i64) -> u64 {
        let mut quadrants = vec![0u64, 0, 0, 0];
        let x_middle = self.x_count / 2;
        let y_middle = self.y_count / 2;
        //println!("middle (x,y) {},{}",x_middle,y_middle);
        for robot in &self.robots {
            let (x, y) = robot.steps(n, self.x_count, self.y_count);
            if x > x_middle {
                if y > y_middle {
                    quadrants[3] += 1;
                } else if y < y_middle {
                    quadrants[1] += 1;
                } //else { println!("({},{}) not counted", x,y)}
            } else if x < x_middle {
                if y > y_middle {
                    quadrants[2] += 1;
                } else if y < y_middle {
                    quadrants[0] += 1;
                } //else { println!("({},{}) not counted", x,y)}
            } //else { println!("({},{}) not counted", x,y)}
        }

        // println!("{:?}",quadrants);
        quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]
    }

    pub fn part2(&self) -> u64 {
        let mut n = 8000;
        let mut line_max= 0;
        loop{
            let mut pos: HashSet<(i64,i64)> = HashSet::new();
            let mut w: HashMap<i64,HashSet<i64>> = HashMap::new();

            for r in &self.robots{
                let r = r.steps(n, self.x_count, self.y_count);
                let (x,y) = r;
                pos.insert(r);
                w.entry(y).or_insert(HashSet::new()).insert(x);
            }


            // Check for tree!
            
            // if score < score_limit { continue;}
            // The idee here is that there need to be as many robots as possible in unique posisitons.
            // With my input all 500 robots were visible when tree was found. So I guess A bit of luck to find it.
            // A better way is to process each pos and see if there are robots in adjacent positions
            //  and find a large number of adjacent robots to see if they form a tree.
            if pos.len() < line_max {
                println!("n {}, #robots {:>10}, max {}",n ,pos.len(), line_max);
                n +=1 ;
                continue;
            }
            line_max = pos.len();
            self.display_hall(&pos);
            println!("n {}, #robots {:>10}, max {}",n ,pos.len(), line_max);
            println!("A christmas tree? y/n ?");

            let mut buff: String = String::new();
            let _ = std::io::stdin().read_line(&mut buff);
            if buff.starts_with("y") {return n as u64;}

            n += 1;
        }
    }

    fn display_hall(&self, positions: &HashSet<(i64, i64)>) {
        for y in 0..self.y_count  {
            for x in 0..self.x_count  {
                if positions.contains(&(x, y)) {
                    print!("*");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}

impl InputParser for Hall {
    fn parse_line(&mut self, line: &String) -> anyhow::Result<()> {
        if line.is_empty() { return Ok(()); }
        let p_v: Vec<&str> = line.split(" ").collect();
        let p: &str = p_v[0].split("=").last().unwrap();
        let v: &str = p_v[1].split("=").last().unwrap();
        let p_x_y: Vec<&str> = p.split(",").collect();
        let v_x_y: Vec<&str> = v.split(",").collect();
        self.robots.push(
            Robot {
                x: p_x_y[0].parse()?,
                y: p_x_y[1].parse()?,
                vx: v_x_y[0].parse()?,
                vy: v_x_y[1].parse()?,
            }
        );
        Ok(())
    }
}

pub struct Robot {
    pub x: i64,
    pub y: i64,
    pub vx: i64,
    pub vy: i64,
}

impl Robot {
    pub fn print(&self) {
        println!("p={},{} v={},{}", self.x, self.y, self.vx, self.vy);
    }
    pub fn steps(&self, n: i64, x_size: i64, y_size: i64) -> (i64, i64) {
        let mut x = self.x + n * self.vx;
        // print!("X: {} -> {}",self.x, x);
        x = x % x_size;
        //print!(" -> {}",x);
        if x < 0 { x = x_size + x; }
        //println!(" -> {}", x);
        let mut y = self.y + n * self.vy;
        //print!("Y: {} -> {}",self.y, y);
        y = y % y_size;
        //print!(" -> {}",y);
        if y < 0 { y = y_size + y; }
        //println!(" -> {}",y);
        // println!();
        (x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //  .....    .....    .....    .....    .....
    //  .1...    .....    .....    .....    .....
    //  .....    .....    ..1..    .....    .....
    //  ..... => ..... => ..... => ..... => ...1.
    //  .....    .....    .....    .....    .....
    //  .....    ....1    .....    .....    .....
    //  .....    .....    .....    1....    .....
    #[test]
    fn movement() {
        let xs = 5;
        let ys = 7;

        let r = Robot {
            x: 1,
            y: 1,
            vx: -2,
            vy: -3,
        };

        let (nx, ny) = r.steps(1, xs, ys);
        assert_eq!(nx, 4);
        assert_eq!(ny, 5);
        let mut p = r.steps(2, xs, ys);
        assert_eq!((2, 2), p);
        p = r.steps(3, xs, ys);
        assert_eq!((0, 6), p);
        p = r.steps(4, xs, ys);
        assert_eq!((3, 3), p);
    }
    //  .....    ..1..    .....    .....    .....
    //  .....    .....    .1...    .....    .....
    //  .....    .....    .....    1....    .....
    //  ..... => ..... => ..... => ..... => ....1
    //  .....    .....    .....    .....    .....
    //  .....    .....    .....    .....    .....
    //  ...1.    .....    .....    .....    .....
    #[test]
    fn movement2() {
        let xs = 5;
        let ys = 7;

        let r = Robot {
            x: 3,
            y: 6,
            vx: -6,
            vy: 8,
        };

        let (nx, ny) = r.steps(1, xs, ys);
        assert_eq!(nx, 2);
        assert_eq!(ny, 0);
        let mut p = r.steps(2, xs, ys);
        assert_eq!((1, 1), p);
        p = r.steps(3, xs, ys);
        assert_eq!((0, 2), p);
        p = r.steps(4, xs, ys);
        assert_eq!((4, 3), p);
    }
}