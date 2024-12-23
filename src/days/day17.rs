use std::ops::{BitAnd, BitXor, BitXorAssign};
use colored::{ColoredString, Colorize}; 


#[derive(Clone)]
struct AlgebraPart{
    operand: usize,
    register: Option<Box<AlgebraPart>>,
    second_register: Option<Box<AlgebraPart>>,
    operation: Operations,  // If this is set to A0, that means this represents A(0), and all other fields should be ignored
}

#[derive(Clone)]
struct Registers{
    a: usize,
    b: usize,
    c: usize,
}

#[derive(PartialEq,Clone,Debug)]
enum Operations{
    Dv = 0,
    Bxl = 1,
    Bst = 2,
    Bxc = 4,
    Out = 5,
    Num = 8,
    A0 = 9,
}

pub(crate) 
fn run(input: String) -> (usize, usize){ 
    println!("{}","Day 17".bright_green().bold()); 
    let (registers, instructions) = parse_input(&input);
    let output = execute_program(registers.clone(), &instructions);
    println!("Part1: "); print!("\t");
    print_number_arr(&output);
    //* Part2
    // Change the initial value of register A so that the program outputs its own instructions
    let mut output0 = create_map(registers.clone(), &instructions);
    println!("Found map");
    // for i in 0..output0.len(){
    //     println!("{}",output0[i].debug());
    //     output0[i].simplify();
    //     println!("{}",output0[i].debug());
    // }
    // One unknown (A0) one known (Output(0) = instructions(0)). Unfortunately % is a not invertible operation, so we can't just solve it.
    // for i in 0..10000000000000{
    //     let output = output0[0].solve(i);
    //     if output == instructions[0] as usize{
    //         return (1, i)
    //     }
    //     if i % 100000000 == 0 {println!("Trying {}",i)}
    // }

    // let mut new_registers = registers.clone();
    // for i in 0..1000000000{
    //     new_registers.a = i;
    //     let output = execute_program(new_registers.clone(), &instructions);
    //     if output == instructions{
    //         return (1,i)
    //     }
    // }
    
    // Try a few values of A
    let mut nr = registers.clone();
    nr.a = 68;//64+4;
    println!("A:{}, output {:?}",nr.a, execute_program(nr.clone(), &instructions));
    println!("LHS: {}",((nr.a % 8).bitxor(4)));
    println!("RHS: {}",nr.a/exp2((nr.a % 8).bitxor(1) as u8));
    return (1, 0); 
} 

fn execute_program(registers: Registers, instructions: &Vec<u8>) -> Vec<u8>{
    let mut output = Vec::new();
    let mut registers = registers;
    let mut instr_ptr = 0;  // The instruction pointer
    println!("Inst_ptr: {}",instr_ptr);
    while instr_ptr < instructions.len()-1{
        let operand = instructions[instr_ptr+1];
        match instructions[instr_ptr] {
            0 => adv(operand, &mut registers),
            1 => bxl(operand, &mut registers),
            2 => bst(operand, &mut registers),
            3 => {
                if let Some(res) = jnz(operand, &registers){
                    instr_ptr = res as usize;
                    continue;
                }},
            4 => bxc(&mut registers),
            5 => output.push(out(operand, &registers)),
            6 => bdv(operand, &mut registers),
            7 => cdv(operand, &mut registers),
            val => {println!("Opcode is too large: {val}. Should not exceed 7.")},
        }
        instr_ptr += 2;
        if output.len() > instructions.len() {return vec![]}
    }
    return output;
}

/// ### Creates a map from A0 to the output (more like a tree)
/// Stops after 1 output, as it is enough to calculate a0, No its not......
fn create_map(registers: Registers, instructions: &Vec<u8>) -> Vec<AlgebraPart>{
    let a0 = AlgebraPart{operand: 0, register: None, operation: Operations::A0, second_register: None};
    let mut output = Vec::new();
    let mut registers: [AlgebraPart;3] = [a0, AlgebraPart::new(registers.b),AlgebraPart::new(registers.c)];
    let mut instr_ptr = 0;  // The instruction pointer
    let jumps = [true,true,true];  // What to do at the jumps   // A is A0/8 at the first (and potentially last) jump, so A0/8 != 0, A0 > 8, second: A0/64 -> A0 > 64, 3:A0>512
    let mut jump_nr = 0;
    while instr_ptr < instructions.len()-1{
        let operand = instructions[instr_ptr+1];
        
        match instructions[instr_ptr] {
            0 => adv_algebra(operand, &mut registers),
            1 => bxl_algebra(operand, &mut registers),
            2 => bst_algebra(operand, &mut registers),
            3 => {
                if output.len() < instructions.len() {  //? Only applicable for my data
                    // Jump,
                    let mut a = registers[0].clone();
                    a.simplify();
                    println!("Requirement: {}!=0",a.debug());
                    instr_ptr = operand as usize;
                } else{
                    // output.push(registers[0].clone()); return output; // Can't handle jump statements
                    jump_nr += 1;
                }
                
                },
            4 => bxc_algebra(&mut registers),
            5 => output.push(out_algebra(operand, &registers)),
            6 => bdv_algebra(operand, &mut registers),
            7 => cdv_algebra(operand, &mut registers),
            val => {println!("Opcode is too large: {val}. Should not exceed 7.")},
        }
        instr_ptr += 2;
        // if output.len() == 1 {return output}
    }
    return output;
}

/// ### Division C <br>
/// register A divided by (2^combo operand) <br>
/// Result is stored in register C
fn cdv(operand: u8, registers: &mut Registers){
    let val = get_combo_operand(operand, registers);
    registers.c = registers.a / exp2(val as u8);
}

/// ### Division B <br>
/// register A divided by (2^combo operand) <br>
/// Result is stored in register B
fn bdv(operand: u8, registers: &mut Registers){
    let val = get_combo_operand(operand, registers);
    registers.b = registers.a / exp2(val as u8);
}

/// ### Output
/// Outputs the combo value of the operand
fn out(operand: u8, registers: &Registers) -> u8{
    return get_combo_operand(operand, registers).bitand(7) as u8;
}

/// ### Bitwise XOR of B and C
/// Calculate bitwise XOR with register B and register C, stores result in register B
fn bxc(registers: &mut Registers){
    registers.b.bitxor_assign(registers.c);
}

/// ### Jump not zero <br>
/// Jumps if register A is not zero
fn jnz(operand: u8, registers: &Registers) -> Option<u8>{
    if registers.a == 0 {return None;}
    return Some(operand);
}

/// ### Combo, mod 8
/// Calculates the combo value of operand, takes the lowest 3 bits, and writes it to register B
fn bst(operand: u8, registers: &mut Registers){
    let val = get_combo_operand(operand, registers);
    registers.b = val.bitand(7);
}

/// ### Bitwise XOR of B and operand <br>
/// Calculate bitwise XOR with register B and literal operand, stores result in register B
fn bxl(operand: u8, registers: &mut Registers){
    registers.b.bitxor_assign(operand as usize);
}

/// ### Division A <br>
/// register A divided by (2^combo operand) <br>
/// Result is stored in register A
fn adv(operand: u8, registers: &mut Registers){
    let val = get_combo_operand(operand, registers);
    registers.a /= exp2(val as u8);
}

fn get_combo_operand(operand: u8, registers: &Registers) -> usize {
    match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => registers.a,
        5 => registers.b,
        6 => registers.c,
        num => {println!("Combo requested on {num}, should not have happened"); 0},
    }
}

/// Calculates 2^pow <br>
/// pow should be smaller than 64
fn exp2(pow: u8) -> usize{
    if pow >= 64{
        panic!("[exp2] 2^{pow} would result in overflow")
    }
    1 << pow
}

fn parse_input(input: &String) -> (Registers, Vec<u8>){
    let mut registers: Registers = Registers{a:0,b:0,c:0};
    let mut count = 0;
    let mut lines = input.lines();
    while let Some(line) = lines.next(){
        if line.is_empty() {break;}
        let mut num = 0;
        // There is only one number
        for c in line.chars(){
            if c.is_digit(10){
                num *= 10;
                num += c.to_digit(10).unwrap() as usize;
            }
        }
        match count {
            0 => {registers.a = num},
            1 => {registers.b = num},
            _ => {registers.c = num},
        }
        count += 1;
    }
    
    // Instructions  (they are [0,7])
    let instructions= lines.next().unwrap().chars().filter(|c| c.is_digit(10)).map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>();

    return (registers, instructions);
}


fn print_number_arr(output: &Vec<u8>){
    print!("{}",output[0]);
    for num in output.iter().skip(1){
        print!(",{num}")
    }
    println!("");
}

//* Part 2
/// ### Division C <br>
/// register A divided by (2^combo operand) <br>
/// Result is stored in register C
fn cdv_algebra(operand: u8, registers: &mut [AlgebraPart;3]){
    let val = get_combo_operand_algebra(operand, registers);
    if val.operation == Operations::Num {
        registers[2] = AlgebraPart{operand: val.operand, register: Some(Box::new(registers[0].clone())), operation: Operations::Dv, second_register: None};
        return;
    }
    registers[2] = AlgebraPart{operand: 0, register: Some(Box::new(registers[0].clone())), operation: Operations::Dv, second_register: Some(Box::new(val))};
}

/// ### Division B <br>
/// register A divided by (2^combo operand) <br>
/// Result is stored in register B
fn bdv_algebra(operand: u8, registers: &mut [AlgebraPart;3]){
    let val = get_combo_operand_algebra(operand, registers);
    if val.operation == Operations::Num {
        registers[1] = AlgebraPart{operand: val.operand, register: Some(Box::new(registers[0].clone())), operation: Operations::Dv, second_register: None};
        return;
    }
    registers[1] = AlgebraPart{operand: 0, register: Some(Box::new(registers[0].clone())), operation: Operations::Dv, second_register: Some(Box::new(val))}
}

/// ### Output
/// Outputs the combo value of the operand
fn out_algebra(operand: u8, registers: &[AlgebraPart;3]) -> AlgebraPart{
    let val = get_combo_operand_algebra(operand, registers);
    if val.operation == Operations::Num {
        // if val.a0 {return AlgebraPart{a0: true, operand: 0, register: None, operation: Operations::Out, second_register: None}}
        return AlgebraPart{operand: val.operand, register: None, operation: Operations::Out, second_register: None}
    }
    return AlgebraPart{operand: 0, register: Some(Box::new(val)), operation: Operations::Out, second_register: None};
}

/// ### Bitwise XOR of B and C
/// Calculate bitwise XOR with register B and register C, stores result in register B
fn bxc_algebra(registers: &mut [AlgebraPart;3]){
    registers[1] = AlgebraPart{ operand: 0, register: Some(Box::new(registers[1].clone())), operation: Operations::Bxc, second_register: Some(Box::new(registers[2].clone()))}
}

/// ### Combo, mod 8
/// Calculates the combo value of operand, takes the lowest 3 bits, and writes it to register B
fn bst_algebra(operand: u8, registers: &mut [AlgebraPart;3]){
    registers[1] = out_algebra(operand, registers);
    registers[1].operation = Operations::Bst;
}

/// ### Bitwise XOR of B and operand <br>
/// Calculate bitwise XOR with register B and literal operand, stores result in register B
fn bxl_algebra(operand: u8, registers: &mut [AlgebraPart;3]){
    registers[1] = AlgebraPart{operand: operand as usize, register: Some(Box::new(registers[1].clone())), operation: Operations::Bxl, second_register: None}
}

/// ### Division A <br>
/// register A divided by (2^combo operand) <br>
/// Result is stored in register A
fn adv_algebra(operand: u8, registers: &mut [AlgebraPart;3]){
    let val = get_combo_operand_algebra(operand, registers);
    if val.operation == Operations::Num {
        registers[0] = AlgebraPart{operand: val.operand, register: Some(Box::new(registers[0].clone())), operation: Operations::Dv, second_register: None};
        return;
    }
    registers[0] = AlgebraPart{operand: 0, register: Some(Box::new(registers[0].clone())), operation: Operations::Dv, second_register: Some(Box::new(val))}
}

fn get_combo_operand_algebra(operand: u8, registers: &[AlgebraPart;3]) -> AlgebraPart {
    match operand {
        0 => AlgebraPart::new(0),
        1 => AlgebraPart::new(1),
        2 => AlgebraPart::new(2),
        3 => AlgebraPart::new(3),
        4 => registers[0].clone(),
        5 => registers[1].clone(),
        6 => registers[2].clone(),
        num => {panic!("Combo requested on {num}, should not have happened");},
    }
}


impl AlgebraPart {
    fn new(val: usize) -> AlgebraPart {
        AlgebraPart{operand: val, register: None, operation: Operations::Num, second_register: None}
    }

    // Solve for some a0
    fn solve(&self, a0: usize) -> usize {
        match self.operation {
            Operations::Num => self.operand,
            Operations::Dv => 
                if let Some(reg2) = self.second_register.as_ref() {
                    self.register.as_ref().unwrap().solve(a0) / exp2(reg2.solve(a0) as u8)
                } else {
                    self.register.as_ref().unwrap().solve(a0) / exp2(self.operand as u8)
                },
            Operations::Bxl => self.register.as_ref().unwrap().solve(a0) ^ self.operand,
            Operations::Bxc => self.register.as_ref().unwrap().solve(a0) ^ self.second_register.as_ref().unwrap().solve(a0),
            Operations::Out => 
                if let Some(register) =  self.register.as_ref(){
                    register.solve(a0).bitand(7)
                }else{
                    self.operand.bitand(7)
                },
            Operations::Bst => 
                if let Some(register) =  self.register.as_ref(){
                    register.solve(a0).bitand(7)
                }else{
                    self.operand.bitand(7)
                },
            Operations::A0 => a0,
        }
    }

    /// Implement debug for struct
    fn debug(&self) -> ColoredString {
        match self.operation {
            Operations::Num => format!("{}",self.operand).truecolor(100, 100, 100),
            Operations::Dv => 
                if let Some(reg2) = self.second_register.as_ref() {
                    format!("({}/exp2({}))",self.register.as_ref().unwrap().debug(),reg2.debug()).truecolor(10, 130, 10)
                } else {
                    format!("({}/{})",self.register.as_ref().unwrap().debug(),exp2(self.operand as u8)).truecolor(10, 130, 10)
                },
            Operations::Bxl => format!("({} ^ {})",self.register.as_ref().unwrap().debug(),self.operand).truecolor(250, 130, 10),
            Operations::Bxc => format!("({} ^ {})",self.register.as_ref().unwrap().debug(),self.second_register.as_ref().unwrap().debug()).truecolor(250, 130, 10),
            Operations::Out => {
                if let Some(register) =  self.register.as_ref(){
                    format!("{} % 8",register.debug()).truecolor(140, 10, 30)
                }else{
                    format!("{} % 8",self.operand).truecolor(140, 10, 30)
                }
            },
            Operations::Bst => format!("({} % 8)",self.register.as_ref().unwrap().debug()).truecolor(140, 10, 10),
            Operations::A0 => format!("A(0)").truecolor(140, 0, 130),
        }
    }

    fn simplify(&mut self){
        match self.operation {
            Operations::Num => {},  // Nothing to do here
            Operations::Dv => {  // The only thing we can do is simplify if they are both numbers
                if let Some(reg2) = self.second_register.as_mut() {
                    self.register.as_mut().unwrap().simplify();
                    reg2.simplify();
                    if reg2.operation == Operations::Num {
                        self.operand = reg2.operand;
                        self.second_register = None;
                        // If the reg1 could also be simplified, than convert to num block
                        let reg1 = self.register.as_ref().unwrap();
                        if reg1.operation == Operations::Num {
                            self.operation = Operations::Num;
                            self.operand = reg1.operand / exp2(self.operand as u8);
                            self.register = None;
                        }
                    }
                } else {
                    // reg1/exp2(operand) 
                    self.register.as_mut().unwrap().simplify();
                    // If the reg1 could be simplified, than convert to num block
                    let reg1 = self.register.as_ref().unwrap();
                    if reg1.operation == Operations::Num {
                        self.operation = Operations::Num;
                        self.operand = reg1.operand / exp2(self.operand as u8);
                        self.register = None;
                    }
                }
                // Merge with the Dv below it (note it is A/exp2(num)/exp2(num) = A/exp2(num+num)
                while self.register.as_mut().unwrap().second_register.is_none() && self.register.as_mut().unwrap().operation == Operations::Dv && self.register.as_ref().unwrap().second_register.is_none() {
                        // They are both numbers
                        self.operand += self.register.as_ref().unwrap().operand;
                        self.register = self.register.as_mut().unwrap().register.take();
                }
            },
            Operations::Bxl => {
                self.register.as_mut().unwrap().simplify();
                // If it is a number
                if self.register.as_ref().unwrap().operation == Operations::Num {
                    self.operation = Operations::Num;
                    self.operand = self.register.as_ref().unwrap().operand.bitxor(self.operand);
                    self.register = None;
                }else{
                    // See if there is a bitxor after, then we can simplify   (4 ^ 2 = 6)
                    if self.register.as_mut().unwrap().operation == Operations::Bxl {
                        self.operand = self.register.as_ref().unwrap().operand.bitxor(self.operand);
                        // Remove the intermediate AlgebraPart
                        self.register = self.register.as_mut().unwrap().register.take();
                    }
                }

            },
            Operations::Bxc => {
                // If the second or first register is a number, we can simplify to a Bxl operation
                self.register.as_mut().unwrap().simplify();
                if let Some(reg2) = self.second_register.as_mut() {
                    reg2.simplify();
                    if reg2.operation == Operations::Num {
                        self.operand = reg2.operand;
                        self.second_register = None;
                        self.operation = Operations::Bxl;
                    } else {
                        if self.register.as_ref().unwrap().operation == Operations::Num {
                            self.operand = self.register.as_ref().unwrap().operand;
                            self.register = self.second_register.take();
                            self.operation = Operations::Bxl;
                        }
                    }
                }
                // Now it is converted to bxl if any part was a number
                // Convert to number if possible
                if self.second_register.is_none() && self.register.as_ref().unwrap().operation == Operations::Num {
                    self.operation = Operations::Num;
                    self.operand = self.register.as_ref().unwrap().operand.bitxor(self.operand);
                    self.register = None;
                }
            },
            Operations::Out | Operations::Bst => {
                // If the register is a number, we can simplify to a number
                self.register.as_mut().unwrap().simplify();
                if self.register.as_ref().unwrap().operation == Operations::Num {
                    self.operation = Operations::Num;
                    self.operand = self.register.as_ref().unwrap().operand.bitand(7);
                    self.register = None;
                }
            },                
            Operations::A0 => {},  // Nothing to do here
        }
        
    }
}

/*
Variables: A,B,C
B(0) & C(0) are known
Only A(0) is unknown

Todo:
Setup A(1) = A(0)/B(0) --> A(0)/number 
keep track in the loop what B and C is, and at the algebraic stuff to the equation if necessary


Operators:  Let an operator of 8 mean copy first field;

A(k+1) = A(k)/2^num
B(k+1) = B(k) bit-xor num
B(k+1) = combo(k) % 8
// jump
B(k+1) = C(i) bit-xor B(k)
out = Combo(k) % 8
B = A/2^num
C = A/2^num

Simplifications:
    num % 8 % 8 = num % 8
    num ^ num = 0
    num ^ 4 ^ 2 = num ^ 6
    num ^ 5 ^ 1 = num ^ 4

*/