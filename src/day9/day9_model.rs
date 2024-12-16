// model types for Day9

use std::collections::VecDeque;
use crate::input::InputParser;

pub struct Haddrive{
    pub blocks: VecDeque<Block>,
    pub empty_spaces: VecDeque<Empty>,    
    pub used_blocks: VecDeque<IdBlock>,
}
pub enum Block {
    Empty,
    Id(u32),
}
#[derive(Debug)]
pub struct Empty {
    pub size: usize,
    pub start_index: usize,
}
pub struct IdBlock{
    pub id: u64,
    pub size: usize,
    pub start_index: usize,
}
impl Haddrive {
    pub fn compact2(&mut self) -> u64 {     
        for cur in self.used_blocks.iter_mut().rev() { 
            // try find a place for this block
            let needed_space = cur.size;
            if let Some(space)= self.empty_spaces.iter_mut().find(|x|x.size >= needed_space) {
               if space.start_index < cur.start_index {
                   cur.start_index = space.start_index;
                   space.size = space.size - needed_space;
                   space.start_index += needed_space;
               }
            }
        }
        self.used_blocks.iter().fold(0, |acc, cur|
            acc + ( (cur.start_index .. cur.start_index + cur.size)
                .fold(0, | loc, i | loc + cur.id * (i as u64) ) ) 
        )
    }
}

impl Haddrive {
    pub fn new() -> Haddrive {
        Haddrive{
            blocks: VecDeque::new(),
            empty_spaces: VecDeque::new(),
            used_blocks: VecDeque::new(),
        }
    }
    
    pub fn compact(&mut self)  -> u64 {
        let mut compacted : Vec::<u32> = Vec::new();
        let blocks = self.blocks.iter().filter(|x| !matches!(**x,  Block::Empty) ).count();
       'outer: loop  {
            match self.blocks.pop_front()    {
                Some(Block::Empty) => {
                    loop {
                        match self.blocks.pop_back() {
                            Some(Block::Id(v) )=> {
                                compacted.push(v);
                                break;
                            }
                            Some(Block::Empty) => {}
                            None => break 'outer,
                        }
                    }
                }
                Some(Block::Id(v))     => compacted.push(v),
                None => break 'outer,
            }
        }
       
        if blocks != compacted.len() {
            panic!("wrong number of blocks {} compacted {}", blocks,  compacted.len());
        }
        let ans = compacted.iter().enumerate().fold(0, |acc, (i, v)| {acc + (i as u64 * (*v as u64))});
        self.blocks = compacted.iter().map(|x|Block::Id(*x)).collect();
        ans
    }
    
}

impl InputParser for Haddrive {
    fn parse_line(&mut self, line: &String) -> anyhow::Result<()> {
        let numbers = line.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>();
        let mut id = 0;
        let mut index = 0;
        loop {
            if id*2 >= numbers.len() { break;}
            self.used_blocks.push_back(IdBlock{ id: id as u64, size: numbers[id*2] as usize, start_index: index}); // part 2
            let count = numbers[id * 2];
            index += count as usize;
            for _ in 0..count {
                self.blocks.push_back(Block::Id(id as u32));
            }
            if id*2+1 >= numbers.len() {break;}
            let count = numbers[id*2+1];
            self.empty_spaces.push_back(Empty{size: count as usize, start_index: index, }); // part2 
            index += count as usize;
            for _ in 0..count as usize {
                self.blocks.push_back(Block::Empty);
            }
            id+=1;
        }
        Ok(())
    }
}