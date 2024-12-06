// model types for Day5

use std::collections::HashSet;
use crate::input::InputParser;

pub struct PrintQueue{
    rules: Vec<(u8,u8)>,
    prints: Vec<u8>,
    all_pages: HashSet<u8>,
    reading_prints: bool,
}

impl PrintQueue {
    pub fn new() -> Self {
        Self{
            rules: Vec::new(),
            prints: Vec::new(),
            reading_prints: false,
        }
    }
}

impl InputParser for PrintQueue {
    fn parse_line(&mut self, line: &String) -> anyhow::Result<()> {
        if line.is_empty() {
            self.reading_prints = true;
            return Ok(());
        }

        Ok(())
    }
}