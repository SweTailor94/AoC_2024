use advent_of_code_2024::input::get_vector_from_file;

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let input = get_vector_from_file("src/day4/input.txt", parse_input_line);
    println!("Day 4 part 1 ");
    println!("XMAS count {}",find_xmas(&input));
    println!("Day 4 part 2 ");
    println!("XMAS count {}",find_xmas_2(&input));
      Ok(())
}

fn parse_input_line(line:&str) -> Vec<u8>{
    line.as_bytes().to_vec()
}

fn find_xmas(i:&Vec<Vec<u8>>) -> i32 {
    let xmas = "XMAS".as_bytes();
    let samx = "SAMX".as_bytes();
    let cols = i[0].len();
    let rows = i.len();
    let mut tmp: Vec<u8> = vec![0,0,0,0];
    let mut xmas_count = 0;
    for row in 0..rows {
        for col in 0 .. cols {
            // across
            if col < cols-3 {
                let check = &i[row][col..col + 4];
                if check == xmas || check == samx {
                    xmas_count += 1;
                }
            }
            // down
            if row < rows-3{
                tmp[0] = i[row][col];
                tmp[1] = i[row+1][col];
                tmp[2] = i[row+2][col];
                tmp[3] = i[row+3][col];
                if tmp.as_slice() == xmas || tmp.as_slice() == samx {
                    xmas_count += 1;
                }
            }
            // diagonal right
            if col < cols-3 && row < rows-3 {
                tmp[0] = i[row][col];
                tmp[1] = i[row + 1][col + 1];
                tmp[2] = i[row + 2][col + 2];
                tmp[3] = i[row + 3][col + 3];
                if tmp.as_slice() == xmas || tmp.as_slice() == samx {
                    xmas_count += 1;
                }
            }
            // diagonal left
            if col >= 3 && row < rows-3 {
                tmp[0] = i[row][col];
                tmp[1] = i[row+1][col-1];
                tmp[2] = i[row+2][col-2];
                tmp[3] = i[row+3][col-3];
                if tmp.as_slice() == xmas || tmp.as_slice() == samx {
                    xmas_count += 1;
                }
            }
        }
    }
    xmas_count
}


fn find_xmas_2(i:&Vec<Vec<u8>>) -> i32 {
    let mas = "MAS".as_bytes();

    let cols = i[0].len();
    let rows = i.len();
    let mut xmas_count = 0;
    for row in 1..rows-1 {
        for col in 1..cols-1 {
            if i[row][col] == mas[1] &&
                (i[row-1][col-1] == mas[0] && i[row+1][col+1] == mas[2] || i[row-1][col-1] == mas[2] && i[row+1][col+1] == mas[0] ) &&
                (i[row+1][col-1] == mas[0] && i[row-1][col+1] == mas[2] || i[row+1][col-1] == mas[2] && i[row-1][col+1] == mas[0] ){
                xmas_count += 1;
            }
        }
    }
    xmas_count
}
