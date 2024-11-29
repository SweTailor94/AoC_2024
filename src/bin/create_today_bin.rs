use std::fs;
use std::fs::{create_dir_all, OpenOptions};
use std::io::{ Write};
use reqwest::header::COOKIE;
use advent_of_code_2024::input::get_private_session;
use anyhow::{bail, Result};
use std::path::{Path, PathBuf};
use chrono::{ Datelike};

fn main()  -> Result<(),Box<dyn std::error::Error>>{

    let now = chrono::offset::Local::now().date_naive();
    let year = now.year();
    let day = now.day();
    let month = now.month();
    if month != 12 {
        println!("Not December yet, come back later.");
    }else {
        println!("Try to create start for year {} day {}", year, day);
        match create_module_bin_and_get_input_for(year as u32, day) {
            Ok(_) => println!("Today is ready to go."),
            Err(e) => println!("Got error {}", e),
        }
    }
    Ok(())
}

///
fn create_module_bin_and_get_input_for(year: u32, day: u32 ) -> Result<()> {

    let  source_root = Path::new("src");
    let folder = source_root.join(format!("day{:00}", day));
    create_dir_all(&folder)?;
    let input_file_path = folder.join("input.txt");
    if input_file_path.exists() {bail!( "Already created");}

    // OK crete files for today!
    create_input_file(year, day, input_file_path)?;
    let model_file = format!("day{}_model",day);
    create_module_and_model_file(day,&folder, &model_file )?;
    create_main_file_stub( day,source_root.join("bin").join(format!("day{:00}_solver.rs", day)))?;
    add_module_to_lib_rs(day)?;

    Ok(())
}

fn add_module_to_lib_rs(day: u32) -> Result<()> {
    let mut lib_file = OpenOptions::new().append(true).open("./src/lib.rs")?;
    lib_file.write_all(format!("pub mod day{:00};\n",day).as_bytes())?;
    Ok(())
}

fn create_module_and_model_file(day: u32, folder: &PathBuf, model_file: &String) -> Result<()>{

    let mut mod_rs = String::new();

    mod_rs += format!("pub mod {};", model_file).as_str();
    fs::write(folder.join(format!("{}.rs",model_file)), format!("// model types for Day{} ",day))?;
    fs::write(folder.join("mod.rs"),mod_rs)?;
    Ok(())
}

fn create_main_file_stub(day: u32, main_filename: PathBuf) -> Result<()> {

    let mut main_output = String::new();

    main_output += &"use advent_of_code_2023::input::get_vector_from_file;\n\n";
    main_output += &"fn main() ->Result<(),Box<dyn std::error::Error>> {\n";
    main_output += format!("    let input = get_vector_from_file(\"src/day{:00}/input.txt\", parse_input_line);\n",day).as_str();
    main_output += format!("    println!(\"Day {} part 1 \");\n",day).as_str();
    main_output += format!("    println!(\"Day {} part 2 \");\n",day).as_str();
    main_output += &"      Ok(())\n}\n\n";

    main_output +=&"fn parse_input_line(line:&str) -> usize{
    line.len()
}\n";

    fs::write(main_filename, main_output)?;
    Ok(())
}

fn create_input_file(year: u32, day: u32, input_file_path: PathBuf) -> Result<()> {
    let session = get_private_session()?;
    let client = reqwest::blocking::Client::new();
    let body = client
        .get(format!("https://adventofcode.com/{}/day/{}/input", year, day)) // /2021/day/1/input
        .header(COOKIE, format!("session={}", session))
        .send()?
        .text()?;

    std::fs::write(input_file_path, body)?;
    Ok(())
}

