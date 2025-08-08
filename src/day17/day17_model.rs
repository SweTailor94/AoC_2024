// model types for Day17

use std::fmt::Display;

#[derive(Debug)]
pub struct Computer{
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    pub mem: Vec<u8>,
}
impl Display for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A: {:02X}, B: {:02X}, C: {:02X}, Mem: {:?}", self.reg_a, self.reg_b, self.reg_c, self.mem )
    }
}
impl Computer {
    pub fn new() -> Self {
        // my personal input:
        //Register A: 66171486
        // Register B: 0
        // Register C: 0
        //
        // Program: 2,4,1,6,7,5,4,6,1,4,5,5,0,3,3,0
        Self{
            reg_a: 66171486,
            reg_b: 0,
            reg_c: 0,
            mem: vec![2,4,1,6,7,5,4,6,1,4,5,5,0,3,3,0],
        }
    }


    pub fn new_test() -> Self {
        //Register A: 729
        // Register B: 0
        // Register C: 0
        //
        // Program: 0,1,5,4,3,0
        Self{
            reg_a:729,
            reg_b:0,
            reg_c:0,
            mem: vec![0,1,5,4,3,0],
        }
    }

    pub fn set_start(&mut self, reg_a: i64, reg_b: i64, reg_c: i64) {
        self.reg_a = reg_a;
        self.reg_b = reg_b;
        self.reg_c = reg_c;
    }
    pub fn get_a(&self) -> i64 {
        self.reg_a
    }

    pub fn mnemonics(&self) {
        let mut p = 0;
        fn operand(o: u8) ->  &'static str {
            match o {
                0 => "0",
                1 => "1",
                2 => "2",
                3 => "3",
                4 => "A",
                5 => "B",
                6 => "C",
                _ => panic!("Illegal operand")
            }
        }
        
        loop{
            match self.mem[p] {
                0 => {
                    // The adv instruction (opcode 0) performs division. The numerator is the value
                    // in the A register. The denominator is found by raising 2 to the power of the
                    // instruction's combo operand. (So, an operand of 2 would divide A by 4 (2^2);
                    // an operand of 5 would divide A by 2^B.) The result of the division operation
                    // is truncated to an integer and then written to the A register.
                    // adv reg_a = reg A / (2.pow(combo operand)
                    let i = p;
                    p += 1;
                    println!("{:02} ADV: A / 2.pow({}) -> A", i, operand(self.mem[p]));
                    p += 1;
                }
                1 => {
                    //The bxl instruction (opcode 1) calculates the bitwise XOR of register B and
                    // the instruction's literal operand, then stores the result in register B.
                    // bxl  b xor literal operand
                    let i = p;
                    p += 1;
                    println!("{:02} BXL: B xor {} -> B", i, self.mem[p]);
                    p += 1;
                   
                }
                2 => {
                    // The bst instruction (opcode 2) calculates the value of its combo operand
                    // modulo 8 (thereby keeping only its lowest 3 bits),
                    // then writes that value to the B register.
                    // bst reg_b = combo operand & 7 (111b)
                    let i = p;
                    p += 1;
                    println!("{:02} BST: {} & 7 -> B", i, operand(self.mem[p]));
                    p += 1;

                }
                3 => {
                    // The jnz instruction (opcode 3) does nothing if the A register is 0. However,
                    // if the A register is not zero, it jumps by setting the instruction pointer to
                    // the value of its literal operand; if this instruction jumps, the instruction
                    // pointer is not increased by 2 after this instruction.
                    // jnz if reg_a != 0 inst_p = literal operand
                    // else inst_p next op
                    let i = p;
                    p += 1;
                    println!("{:02} JNZ: if A != 0  jmp to {} else go on.", i, self.mem[p]);
                    p += 1;

                }
                4 => {
                    // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and
                    // register C, then stores the result in register B. (For legacy reasons,
                    // this instruction reads an operand but ignores it.)
                    // bxc reg_B = reg_b xor reg_c
                    // ignore operand
                    let i = p;
                    p += 1;
                    println!("{:02} BXC: B xor C -> B", i);
                    p += 1;
                   
                }
                5 => {
                    // The out instruction (opcode 5) calculates the value of its combo operand
                    // modulo 8, then outputs that value. (If a program outputs multiple values,
                    // they are separated by commas.)
                    // out combo operand & 7  add. output.push
                    let i = p;
                    p += 1;
                    println!("{:02} OUT: {} & 7", i, operand(self.mem[p]) );
                    p += 1;
                   
                }
                6 => {
                    // The bdv instruction (opcode 6) works exactly like the adv instruction except
                    // that the result is stored in the B register.
                    // (The numerator is still read from the A register.)
                    // bdv reg_b = reg A / (2.pow(combo operand)
                    let i = p;
                    p += 1;
                    println!("{:02} BDV: A / 2.pow({}) -> B", i, operand(self.mem[p]));
                    p += 1;
                }
                7 => {
                    // The cdv instruction (opcode 7) works exactly like the adv instruction
                    // except that the result is stored in the C register.
                    // (The numerator is still read from the A register.)
                    // cdv reg_c = reg A / (2.pow(combo operand)
                    let i = p;
                    p += 1;
                    println!("{:02} CDV: A / 2.pow({}) -> C", i, operand(self.mem[p]));
                    p += 1;
                }
                _ => {}
            }
            if p >= self.mem.len() { 
                break;
            }
        }
    }
    pub fn execute(&mut self) -> Vec<u8> {
        let mut inst_p: usize = 0;
        let mut output: Vec<u8> = Vec::new();
        while inst_p < self.mem.len() {
           // println!("-------");
            //println!("inst_p {}, Outp: {:?}",inst_p, output);
            //println!("State {}",self);
            match self.mem[inst_p] {
                0 => {
                    // The adv instruction (opcode 0) performs division. The numerator is the value
                    // in the A register. The denominator is found by raising 2 to the power of the
                    // instruction's combo operand. (So, an operand of 2 would divide A by 4 (2^2);
                    // an operand of 5 would divide A by 2^B.) The result of the division operation
                    // is truncated to an integer and then written to the A register.
                    // adv reg_a = reg A / (2.pow(combo operand)
                    inst_p += 1;

                    let op = self.mem[inst_p];
                    let combo = self.get_combo_operand(op) as u32;
                    let denom = 2i64.pow(combo);
                    let val = self.reg_a / denom;
                    //println!("adv:  reg_A {0:02X}, op {1}, combo {2:02X}, denom {3:02X} -> {0:02X} / {3:02X} = {4:02X} -> A ",self.reg_a, op, combo, denom, val );
                    self.reg_a = val;
                    inst_p += 1;
                }
                1 => {
                    //The bxl instruction (opcode 1) calculates the bitwise XOR of register B and
                    // the instruction's literal operand, then stores the result in register B.
                    // bxl  b xor literal operand
                    inst_p += 1;
                    let lit = self.mem[inst_p] as i64;
                    let val = self.reg_b ^ lit;
                    //println!("bxl: {:02X} xor {:02X} = {:02X}",self.reg_b ,lit , val);
                    self.reg_b = val;
                    inst_p += 1;
                }
                2 => {
                    // The bst instruction (opcode 2) calculates the value of its combo operand
                    // modulo 8 (thereby keeping only its lowest 3 bits),
                    // then writes that value to the B register.
                    // bst reg_b = combo operand & 7 (111b)
                    inst_p += 1;
                    let op = self.mem[inst_p];
                    let combo = self.get_combo_operand(op);
                    let val = combo & 7;
                    //println!("bst: op:{} combo:{:02X} & 7 = {} -> B", op, combo, val);
                    self.reg_b = val;
                    inst_p += 1;
                }
                3 => {
                    // The jnz instruction (opcode 3) does nothing if the A register is 0. However,
                    // if the A register is not zero, it jumps by setting the instruction pointer to
                    // the value of its literal operand; if this instruction jumps, the instruction
                    // pointer is not increased by 2 after this instruction.
                    // jnz if reg_a != 0 inst_p = literal operand
                    // else inst_p next op
                    if self.reg_a == 0 {
                        //println!("jnz: no jump (reg a == 0)");
                        inst_p += 2;
                    } else {
                        inst_p += 1;
                        inst_p = self.mem[inst_p] as usize;
                        //println!("jnz:  jump to {}",inst_p);
                    }
                }
                4 => {
                    // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and
                    // register C, then stores the result in register B. (For legacy reasons,
                    // this instruction reads an operand but ignores it.)
                    // bxc reg_B = reg_b xor reg_c
                    // ignore operand
                    inst_p += 1;
                    let val = self.reg_b ^ self.reg_c;
                    //println!("bxc {:02X} xor {:02X} = {:02X}",self.reg_b,self.reg_c,val);
                    self.reg_b = val;
                    inst_p += 1;
                }
                5 => {
                    // The out instruction (opcode 5) calculates the value of its combo operand
                    // modulo 8, then outputs that value. (If a program outputs multiple values,
                    // they are separated by commas.)
                    // out combo operand & 7  add. output.push
                    inst_p += 1;
                    let op = self.mem[inst_p];
                    let combo = self.get_combo_operand(op);
                    let out = combo & 7 ;
                    //println!("Out, op {}, combo {:02X}, out val {}",op,combo,out);
                    output.push(out as u8);
                    inst_p += 1;
                }
                6 => {
                    // The bdv instruction (opcode 6) works exactly like the adv instruction except
                    // that the result is stored in the B register.
                    // (The numerator is still read from the A register.)
                    // bdv reg_b = reg A / (2.pow(combo operand)
                    inst_p += 1;
                    let op = self.mem[inst_p];
                    let combo = self.get_combo_operand(op) as u32;
                    let denom = 2i64.pow(combo);
                    let val = self.reg_a / denom;
                    //println!("bdv:  reg_A {0:02X}, op {1}, combo {2:02X}, denom {3:02X} -> {0:02X} / {3:02X} = {4:02X} -> B ",self.reg_a, op, combo, denom, val );
                    self.reg_b = val;
                    inst_p += 1;
                }
                7 => {
                    // The cdv instruction (opcode 7) works exactly like the adv instruction
                    // except that the result is stored in the C register.
                    // (The numerator is still read from the A register.)
                    // cdv reg_c = reg A / (2.pow(combo operand)
                    inst_p += 1;
                    let op = self.mem[inst_p];
                    let combo = self.get_combo_operand(op) as u32;
                    let denom = 2i64.pow(combo);
                    let val = self.reg_a / denom;
                    //println!("cdv:  reg_A {0:02X}, op {1}, combo {2:02X}, denom {3:02X} -> {0:02X} / {3:02X} = {4:02X} -> C ",self.reg_a, op, combo, denom, val );
                    self.reg_c = val;
                    inst_p += 1;
                }
                _ => {}
            }
        }

        output
    }

    fn get_combo_operand(&mut self, combo: u8) -> i64 {
        match combo {
            0..=3 => combo as i64,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("illegal operand: {}", combo),
        }
    }

    pub fn part2(&mut self) -> i64 {
        
        let mut last_a = 0;
        let mut index = (self.mem.len()-1) as i64;
        let mut out:Vec<u8>;
        loop{
            let mut three_bit: i64 = 0;
            // println!("Index {} ", index);
            loop {
                let try_a = last_a |three_bit;
                // println!("  Try {:o} ", try_a);
                self.set_start(try_a,0,0);
                out = self.execute();
                // println!("  Output {:>60}", format!("{:?}",out));
                // println!("  Origin {:>60}", format!("{:?}",self.mem));

                if out[0] == self.mem[index as usize]{
                    last_a = try_a << 3;
                    index -= 1;
                    break;
                }
                three_bit += 1;
                while three_bit > 7{
                    last_a = last_a >> 3;
                    three_bit = (last_a & 7) + 1;
                    last_a = last_a & 0x7FFFFFFF_FFFFFFF8;
                    index += 1;
                    // println!("Index {} ", index);
                    if index >= self.mem.len() as i64 {panic!("Impossible!");}
                }
            }
            if index < 0 {
                last_a = last_a >> 3;
                break;
            }
        }
        println!("  Output {:?}", out);
        println!("  Origin {:?}", self.mem);

        last_a
    }

    pub fn is_program(&mut self, a: i64) -> ProgramResult {
        self.set_start(a, 0, 0);
        let p = self.execute();
        println!("{:?} when A {}",p,a);
        if self.mem.len() != p.len() { return ProgramResult::WrongLength};
        if self.mem == p {
            ProgramResult::Yes
        } else {
            ProgramResult::No
        }
    }
}

pub enum ProgramResult {
    No,
    Yes,
    WrongLength,
}