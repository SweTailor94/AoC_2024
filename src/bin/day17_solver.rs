use advent_of_code_2024::day17::day17_model::Computer;

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let mut computer : Computer = Computer::new();
    computer.mnemonics();
    //return Ok(());
    println!("Day 17 part 1 ");
    let output = computer.execute();
    println!("{:?}",output);
    println!("{:?}", output.iter().map( |x| x.to_string()).collect::<Vec<String>>().join(","));
    println!("Day 17 part 2 ");
    let mut comp2 = Computer::new();
    comp2.set_start(35184372088832,0,0);
    println!("{}", comp2.part2());
    
    // println!("{:}",comp2.get_a());
    // loop {
    //     let mut input: String = String::new();
    //     std::io::stdin().read_line(&mut input)?;
    //     if input == "q" || input == "quit" {
    //         break;
    //     }
    //     let a:i64 = match input.trim().parse() {
    //         Ok(num) => num,
    //         Err(error) => {
    //             println!("try parse |{}| Error: {}", input.trim(), error);                    
    //             continue;
    //         }
    //     };
    // 
    //     comp2.is_program(a);
    //     println!("{:?}",comp2.mem);
    // }
    Ok(())
}

