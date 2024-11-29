
use std::fs::File;
use std::io::{BufReader, BufRead};
use anyhow::{Result,Ok};

pub trait InputParser {
    fn parse_line(&mut self, line:&String) -> Result<()>;
}

pub fn parse_input_file<P>(file_name: &str, parser: &mut P) -> Result<()> where
    P: InputParser{
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let mut count = 0;
    for line in reader.lines() {
        parser.parse_line(&(line.unwrap()))?;
        count += 1;
    };
    let _ = parser.parse_line(&"".to_string());
    println!("Lines in input {}",count);
    Ok(())
}
pub fn parse_input_string<P>(input: &String, parser: &mut P) -> Result<()> where
    P: InputParser{
    for line in input.lines() {
        parser.parse_line(&(line.to_string()))?;
    };
    Ok(())
}

pub fn get_vector_from_file<T, F>(file_name: &str, mut transform: F) -> Vec<T> where
    F: FnMut(&str) -> T {
    let mut all_of_them: Vec<T> = Vec::new();
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let val = transform(line.unwrap().as_str());
        all_of_them.push(val);
    }
    return all_of_them;
}



pub fn get_private_session() -> Result<String>{
    let file = File::open("my_secret.txt")?;
    let mut reader = BufReader::new(file);
    let mut secret :String = String::new();
    let _size = reader.read_line(&mut secret)?;
    Ok(secret)
}