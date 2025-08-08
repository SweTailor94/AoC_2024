// model types for Day15

use std::collections::HashMap;
use crate::input::InputParser;

#[derive(Copy, Clone, Debug)]
pub enum Object{
    Wall,
    Box,
}

pub struct World{
    rows: u32,
    cols: u32,
    stuff: HashMap<(u32,u32),Object>,
    robot: (u32,u32),
}
const WALL: u8 = 0x23; // '#'
const VOID: u8 = 0x2E; // '.'
const BOX: u8 = 0x4F; // 'O'
const ROBOT: u8 = 0x40; // '@'
impl World{
    pub fn new() -> Self{
        Self{
            rows:0,
            cols:0,
            stuff: HashMap::new(),
            robot: (0,0),
        }
    }
    pub fn print_robot(&self) {
        println!("Robot @ {:?}", self.robot);
    }
    pub fn parse_line(&mut self, line:&str){
        if line.is_empty() {return;}


        let line_as_u8 = line.as_bytes();
        for (index, c) in line_as_u8.iter().enumerate() {
            match *c {
                WALL => {
                    self.stuff.insert((self.rows, index as u32), Object::Wall);
                }
                BOX => {
                    self.stuff.insert((self.rows, index as u32), Object::Box);
                }
                ROBOT => {
                    self.robot = (self.rows, index as u32);
                }
                VOID => {}
                _ => panic!("Unknown character"),
            };
        }
        self.rows += 1;
        self.cols = line_as_u8.len() as u32;
    }

    pub fn move_robot(&mut self, dir: Direction){
        let (row,col) = self.robot;
        match dir {
            Direction::Up => {
                if self.push_up(row,col) {
                    self.robot = (row - 1,col);
                }
            }
            Direction::Down => {
                if self.push_down(row,col) {
                    self.robot = (row + 1,col);
                }
            }
            Direction::Left => {
                if self.push_left(row,col) {
                    self.robot = (row,col - 1);
                }

            }
            Direction::Right => {if self.push_right(row, col) {
                self.robot = (row,col + 1);
            }
            }
        }
    }

    fn push_up(&mut self, mut row:u32, col:u32) -> bool{
        let mut shall_move: Vec<(u32,u32)> = Vec::new();

        loop {
            row -= 1;
            if let Some(o) = self.stuff.get(&(row,col)) {
                match o {
                    Object::Wall => {return false;}
                    Object::Box => { shall_move.push((row,col));}
                }
            } else {
                // Space in the end found
                break;
            }
        }
        for k in shall_move.iter() {
            if matches!(self.stuff.remove(&k), None) {panic!("Just found these boxes ???") }
        }
        for (row, col) in shall_move {
            let _ = self.stuff.insert((row -1, col), Object::Box);
        }
        true
    }
    fn push_down(&mut self, mut row:u32, col:u32) -> bool{
        let mut shall_move: Vec<(u32,u32)> = Vec::new();

        loop {
            row += 1;
            if let Some(o) = self.stuff.get(&(row,col)) {
                match o {
                    Object::Wall => {return false;}
                    Object::Box => { shall_move.push((row,col));}
                }
            } else {
                // Space in the end found
                break;
            }
        }
        for k in shall_move.iter() {
            if matches!(self.stuff.remove(&k), None) {panic!("Just found these boxes ???") }
        }
        for (row, col) in shall_move {
            let _ = self.stuff.insert((row +1, col), Object::Box);
        }
        true
    }
    fn push_left(&mut self, row:u32, mut col:u32) -> bool{
        let mut shall_move: Vec<(u32,u32)> = Vec::new();

        loop {
            col -= 1;
            if let Some(o) = self.stuff.get(&(row,col)) {
                match o {
                    Object::Wall => {return false;}
                    Object::Box => { shall_move.push((row,col));}
                }
            } else {
                // Space in the end found
                break;
            }
        }
        for k in shall_move.iter() {
            if matches!(self.stuff.remove(&k), None) {panic!("Just found these boxes ???") }
        }
        for (row, col) in shall_move {
            let _ = self.stuff.insert((row, col - 1), Object::Box);
        }
        true
    }
    fn push_right(&mut self, row:u32, mut col:u32) -> bool{
        let mut shall_move: Vec<(u32,u32)> = Vec::new();

        loop {
            col += 1;
            if let Some(o) = self.stuff.get(&(row,col)) {
                match o {
                    Object::Wall => {return false;}
                    Object::Box => { shall_move.push((row,col)); }
                }
            } else {
                // Space in the end found
                break;
            }
        }
        for k in shall_move.iter() {
            if matches!(self.stuff.remove(&k), None) {panic!("Just found these boxes ???") }
        }
        for (row, col) in shall_move {
            let _ = self.stuff.insert((row, col + 1), Object::Box);
        }
        true
    }
    pub fn calculate_gps_sum(& self) -> u32{
        self.stuff.iter().filter_map(|((r,c),val)|
            match val {
                Object::Box => Some(100*r+c),
                _ => None,
        }).sum()
    }

}

#[derive(Debug,Copy, Clone)]
pub enum Direction{
    Up,
    Down,
    Left,
    Right,
}
// 5E, 76, 3C, 3E
// ^ , v , < , >
const UP: u8 = 0x5E;
const DOWN: u8 = 0x76;
const LEFT: u8 = 0x3C;
const RIGHT: u8 = 0x3E;
pub struct Warehouse{
    pub world: World,
    pub moves: Vec<Direction>,
    parsing_world: bool,
}

impl Warehouse {
    pub fn new() -> Self {
        Self{
            world: World::new(),
            moves: Vec::new(),
            parsing_world: true,
        }
    }

    pub fn part1(&mut self) -> u32{
        for m in &self.moves {
            self.world.move_robot(*m);
        }

        self.world.calculate_gps_sum()
    }

}

impl InputParser for Warehouse {
    fn parse_line(&mut self, line: &String) -> anyhow::Result<()> {
        if line.is_empty() {
            self.parsing_world = false;
            return Ok(());
        };
        if self.parsing_world {
            self.world.parse_line(line.as_str());
        } else {
            for c in line.as_bytes() {
                match *c {
                    UP => self.moves.push(Direction::Up),
                    DOWN => self.moves.push(Direction::Down),
                    LEFT => self.moves.push(Direction::Left),
                    RIGHT => self.moves.push(Direction::Right),
                    _ => panic!("Unknown direction"),
                }
            }
        }
        Ok(())
    }
}