// model types for Day17

pub struct Computer{
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    mem: Vec<u8>,
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

    pub fn execute(&mut self) -> Vec<i64> {
        let mut inst_p: usize = 0;
        let mut output: Vec<i64> = Vec::new();
        loop {
            match self.mem[inst_p] {
                0 => {
                    //
                }
                _ => {}
            }
        }

    }
}