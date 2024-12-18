use std::ops::{BitAnd, BitXorAssign};
use colored::Colorize; 


#[derive(Clone)]
struct AlgebraPart{
    operand: usize,
    register: Option<Box<AlgebraPart>>,
    second_register: Option<Box<AlgebraPart>>,
    operation: Operations,
    a0: bool,  // If this is set to true, that means this represents A(0), and all other fields should be ignored
}

#[derive(Clone)]
struct Registers{
    a: usize,
    b: usize,
    c: usize,
}

#[derive(PartialEq,Clone)]
enum Operations{
    Dv = 0,
    Bxl = 1,
    Bst = 2,
    Bxc = 4,
    Out = 5,
    Num = 8,
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
    let output0 = create_map(registers.clone(), &instructions);
    println!("Found map");
    // One unknown (A0) one known (Output(0) = instructions(0)). Unfortunately % is a not invertible operation, so we can't just solve it.
    for i in 0..10000000000000{
        let output = output0[0].solve(i);
        if output == instructions[0] as usize{
            return (1, i)
        }
        if i % 100000000 == 0 {println!("Trying {}",i)}
    }

    // let mut new_registers = registers.clone();
    // for i in 0..1000000000{
    //     new_registers.a = i;
    //     let output = execute_program(new_registers.clone(), &instructions);
    //     if output == instructions{
    //         return (1,i)
    //     }
    // }
    return (1, 0); 
} 

fn execute_program(registers: Registers, instructions: &Vec<u8>) -> Vec<u8>{
    let mut output = Vec::new();
    let mut registers = registers;
    let mut instr_ptr = 0;  // The instruction pointer
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
    let a0 = AlgebraPart{a0: true,operand: 0, register: None, operation: Operations::Num, second_register: None};
    let mut output = Vec::new();
    let mut registers: [AlgebraPart;3] = [a0, AlgebraPart::new(registers.b),AlgebraPart::new(registers.c)];
    let mut instr_ptr = 0;  // The instruction pointer
    while instr_ptr < instructions.len()-1{
        let operand = instructions[instr_ptr+1];
        match instructions[instr_ptr] {
            0 => adv_algebra(operand, &mut registers),
            1 => bxl_algebra(operand, &mut registers),
            2 => bst_algebra(operand, &mut registers),
            3 => {
                    return output; // Can't handle jump statements
                },
            4 => bxc_algebra(&mut registers),
            5 => output.push(out_algebra(operand, &registers)),
            6 => bdv_algebra(operand, &mut registers),
            7 => cdv_algebra(operand, &mut registers),
            val => {println!("Opcode is too large: {val}. Should not exceed 7.")},
        }
        instr_ptr += 2;
        if output.len() == 1 {return output}
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
    registers[2] = AlgebraPart{a0: false, operand: operand as usize, register: Some(Box::new(registers[0].clone())), operation: Operations::Dv, second_register: None}
}

/// ### Division B <br>
/// register A divided by (2^combo operand) <br>
/// Result is stored in register B
fn bdv_algebra(operand: u8, registers: &mut [AlgebraPart;3]){
    registers[1] = AlgebraPart{a0: false, operand: operand as usize, register: Some(Box::new(registers[0].clone())), operation: Operations::Dv, second_register: None}
}

/// ### Output
/// Outputs the combo value of the operand
fn out_algebra(operand: u8, registers: &[AlgebraPart;3]) -> AlgebraPart{
    let val = get_combo_operand_algebra(operand, registers);
    if val.operation == Operations::Num { return AlgebraPart{a0: false, operand: val.operand, register: None, operation: Operations::Out, second_register: None} }

    return AlgebraPart{a0: false, operand: 0, register: Some(Box::new(val)), operation: Operations::Out, second_register: None};
}

/// ### Bitwise XOR of B and C
/// Calculate bitwise XOR with register B and register C, stores result in register B
fn bxc_algebra(registers: &mut [AlgebraPart;3]){
    registers[1] = AlgebraPart{a0: false, operand: 0, register: Some(Box::new(registers[1].clone())), operation: Operations::Bxc, second_register: Some(Box::new(registers[2].clone()))}
}

/// ### Combo, mod 8
/// Calculates the combo value of operand, takes the lowest 3 bits, and writes it to register B
fn bst_algebra(operand: u8, registers: &mut [AlgebraPart;3]){
    registers[1] = out_algebra(operand, registers);
}

/// ### Bitwise XOR of B and operand <br>
/// Calculate bitwise XOR with register B and literal operand, stores result in register B
fn bxl_algebra(operand: u8, registers: &mut [AlgebraPart;3]){
    registers[1] = AlgebraPart{a0: false, operand: operand as usize, register: Some(Box::new(registers[1].clone())), operation: Operations::Bxl, second_register: None}
}

/// ### Division A <br>
/// register A divided by (2^combo operand) <br>
/// Result is stored in register A
fn adv_algebra(operand: u8, registers: &mut [AlgebraPart;3]){
    registers[1] = AlgebraPart{a0: false, operand: operand as usize, register: Some(Box::new(registers[0].clone())), operation: Operations::Dv, second_register: None}
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
        AlgebraPart{a0: false, operand: val, register: None, operation: Operations::Num, second_register: None}
    }

    // Solve for some a1
    fn solve(&self, a1: usize) -> usize {
        match self.operation {
            Operations::Num => self.operand,
            Operations::Dv => self.register.as_ref().unwrap().solve(a1) / exp2(self.operand as u8),
            Operations::Bxl => self.register.as_ref().unwrap().solve(a1) ^ self.operand,
            Operations::Bxc => self.register.as_ref().unwrap().solve(a1) ^ self.second_register.as_ref().unwrap().solve(a1),
            Operations::Out => {
                if let Some(register) =  self.register.as_ref(){
                    register.solve(a1).bitand(7)
                }else{
                    self.operand.bitand(7)
                }
            },
            Operations::Bst => self.register.as_ref().unwrap().solve(a1).bitand(7),
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

*/