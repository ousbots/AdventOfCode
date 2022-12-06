use std::fs;
use std::io;

struct Computer {
    pc: usize,
    relative: i64,
    program: Vec<i64>,
    memory: Vec<i64>,
    input: Vec<i64>,
    output: Vec<i64>,
    halt: bool,
}

fn computer(program: Vec<i64>, memory: Vec<i64>) -> Computer {
    return Computer{
        pc: 0,
        relative: 0,
        program: program,
        memory: memory,
        input: vec![],
        output: vec![],
        halt: false,
    }
}

enum Mode {
    POSITION,
    IMMEDIATE,
    RELATIVE,
}

impl Computer {

    // Parses the given memory location for the opcode, parameters, and modes of a given length.
    fn parse_opcode(&self, pos: usize, len: usize) -> (i64, Vec<i64>, Vec<Mode>) {
        const INSTR_MOD: i64 = 100;
        const MODE_MOD: i64 = 10;

        let mut opcode: i64 = self.program[pos];
        let instr: i64 = opcode % INSTR_MOD;
        opcode = (opcode - instr) / INSTR_MOD;

        let mut modes: Vec<Mode> = Vec::new();
        let mut params: Vec<i64> = Vec::new();

        for offset in 1 .. len+1 {
            params.push(self.program[pos + offset]);

            match opcode % MODE_MOD {
                0 => modes.push(Mode::POSITION),
                1 => modes.push(Mode::IMMEDIATE),
                2 => modes.push(Mode::RELATIVE),
                _ => panic!("wrong parameter mode"),
            }

            opcode = (opcode - (opcode % MODE_MOD)) / MODE_MOD;
        }

        return (instr, params, modes);
    }

    //  Runs the IntCode machine on the given memory space.
    //
    //  An opcode is broken down into: the least 2 digits are the instruction, and then the remaining
    //  digits represent the parameter's mode in increasing signifigance.
    //
    // TODO:: FIXME
    //  A parameter can be given in three different modes: in immediate mode, the value of the parameter
    //  is used directly, in position mode, the value of the parameter is the memory location from
    //  which to retrieve the value.  The mode is given by digits preceding the instruction in the
    //  opcode.
    //
    //  opcodes: opcode [number of params]: description
    //      1  [3] => add the values given by params 1 and 2 and store at param 3.
    //      2  [3] => multiply the values given by params 1 and 2 and store at param 3.
    //      3  [1] => takes an i64 from user input and stores it at param 1.
    //      4  [1] => prints the value given by param 1.
    //      5  [2] => if param 1 is not zero it sets the instruction pointer to param 2.
    //      6  [2] => if param 1 is zero it sets the instruction pointer to param 2.
    //      7  [3] => if param 1 is less than param 2 it stores 1 at param 3, otherwise 0 is stored.
    //      8  [3] => if param 1 is equal to param 2 it stores 1 at param 3, otherwise 0 is stored.
    //      9  [1] => adjust the relative memory base address by the parameter amount.
    //      99 [0] => halt
    fn run(&mut self) {
        loop {
            let (instr, _, _) = self.parse_opcode(self.pc, 0);

            match instr {
                1 => {
                    let (_, params, modes) = self.parse_opcode(self.pc, 3);
                    let (op1, op2): (i64, i64);

                    match modes[0] {
                        Mode::POSITION => op1 = self.memory[params[0] as usize],
                        Mode::IMMEDIATE => op1 = params[0],
                        Mode::RELATIVE => op1 = self.memory[(self.relative + params[0]) as usize],
                    }

                    match modes[1] {
                        Mode::POSITION => op2 = self.memory[params[1] as usize],
                        Mode::IMMEDIATE => op2 = params[1],
                        Mode::RELATIVE => op2 = self.memory[(self.relative + params[1]) as usize],
                    }

                    let addr: usize;
                    match modes[2] {
                        Mode::POSITION => addr = params[2] as usize,
                        Mode::RELATIVE => addr = (self.relative + params[2]) as usize,
                        _ => panic!("bad instruction"),
                    }

                    self.memory[addr] = op1 + op2;
                    self.pc += 4;
                },

                2 => {
                    let (_, params, modes) = self.parse_opcode(self.pc, 3);
                    let (op1, op2): (i64, i64);

                    match modes[0] {
                        Mode::POSITION => op1 = self.memory[params[0] as usize],
                        Mode::IMMEDIATE => op1 = params[0],
                        Mode::RELATIVE => op1 = self.memory[(self.relative + params[0]) as usize],
                    }

                    match modes[1] {
                        Mode::POSITION => op2 = self.memory[params[1] as usize],
                        Mode::IMMEDIATE => op2 = params[1],
                        Mode::RELATIVE => op2 = self.memory[(self.relative + params[1]) as usize],
                    }

                    let addr: usize;
                    match modes[2] {
                        Mode::POSITION => addr = params[2] as usize,
                        Mode::RELATIVE => addr = (self.relative + params[2]) as usize,
                        _ => panic!("bad instruction"),
                    }

                    self.memory[addr] = op1 * op2;
                    self.pc += 4;
                },

                3 => {
                    let (_, params, modes) = self.parse_opcode(self.pc, 1);

                    if self.input.len() == 0 {
                        println!("missing input");
                        return;
                    }

                    let value: i64 = self.input[0];
                    self.input.remove(0);

                    let addr: usize;
                    match modes[0] {
                        Mode::POSITION => addr = params[0] as usize,
                        Mode::RELATIVE => addr = (self.relative + params[0]) as usize,
                        _ => panic!("bad instruction"),
                    }

                    self.memory[addr] = value;
                    self.pc += 2;
                },

                4 => {
                    let (_, params, modes) = self.parse_opcode(self.pc, 1);
                    let op1: i64;

                    match modes[0] {
                        Mode::POSITION => op1 = self.memory[params[0] as usize],
                        Mode::IMMEDIATE => op1 = params[0],
                        Mode::RELATIVE => op1 = self.memory[(self.relative + params[0]) as usize],
                    }

                    self.output.push(op1);
                    self.pc += 2;
                },

                5 => {
                    let (_, params, modes) = self.parse_opcode(self.pc, 2);
                    let (op1, op2): (i64, i64);

                    match modes[0] {
                        Mode::POSITION => op1 = self.memory[params[0] as usize],
                        Mode::IMMEDIATE => op1 = params[0],
                        Mode::RELATIVE => op1 = self.memory[(self.relative + params[0]) as usize],
                    }

                    match modes[1] {
                        Mode::POSITION => op2 = self.memory[params[1] as usize],
                        Mode::IMMEDIATE => op2 = params[1],
                        Mode::RELATIVE => op2 = self.memory[(self.relative + params[1]) as usize],
                    }

                    if op1 != 0 {
                        self.pc = op2 as usize;
                    }

                    else {
                        self.pc += 3;
                    }
                },

                6 => {
                    let (_, params, modes) = self.parse_opcode(self.pc, 2);
                    let (op1, op2): (i64, i64);

                    match modes[0] {
                        Mode::POSITION => op1 = self.memory[params[0] as usize],
                        Mode::IMMEDIATE => op1 = params[0],
                        Mode::RELATIVE => op1 = self.memory[(self.relative + params[0]) as usize],
                    }

                    match modes[1] {
                        Mode::POSITION => op2 = self.memory[params[1] as usize],
                        Mode::IMMEDIATE => op2 = params[1],
                        Mode::RELATIVE => op2 = self.memory[(self.relative + params[1]) as usize],
                    }

                    if op1 == 0 {
                        self.pc = op2 as usize;
                    }

                    else {
                        self.pc += 3;
                    }
                },

                7 => {
                    let (_, params, modes) = self.parse_opcode(self.pc, 3);
                    let (op1, op2): (i64, i64);

                    match modes[0] {
                        Mode::POSITION => op1 = self.memory[params[0] as usize],
                        Mode::IMMEDIATE => op1 = params[0],
                        Mode::RELATIVE => op1 = self.memory[(self.relative + params[0]) as usize],
                    }

                    match modes[1] {
                        Mode::POSITION => op2 = self.memory[params[1] as usize],
                        Mode::IMMEDIATE => op2 = params[1],
                        Mode::RELATIVE => op2 = self.memory[(self.relative + params[1]) as usize],
                    }

                    let result: i64;
                    if op1 < op2 {
                        result = 1;
                    }

                    else {
                        result = 0;
                    }

                    let addr: usize;
                    match modes[2] {
                        Mode::POSITION => addr = params[2] as usize,
                        Mode::RELATIVE => addr = (self.relative + params[2]) as usize,
                        _ => panic!("bad instruction"),
                    }

                    self.memory[addr] = result;
                    self.pc += 4;
                },

                8 => {
                    let (_, params, modes) = self.parse_opcode(self.pc, 3);
                    let (op1, op2): (i64, i64);

                    match modes[0] {
                        Mode::POSITION => op1 = self.memory[params[0] as usize],
                        Mode::IMMEDIATE => op1 = params[0],
                        Mode::RELATIVE => op1 = self.memory[(self.relative + params[0]) as usize],
                    }

                    match modes[1] {
                        Mode::POSITION => op2 = self.memory[params[1] as usize],
                        Mode::IMMEDIATE => op2 = params[1],
                        Mode::RELATIVE => op2 = self.memory[(self.relative + params[1]) as usize],
                    }

                    let result: i64;
                    if op1 == op2 {
                        result = 1;
                    }

                    else {
                        result = 0;
                    }

                    let addr: usize;
                    match modes[2] {
                        Mode::POSITION => addr = params[2] as usize,
                        Mode::RELATIVE => addr = (self.relative + params[2]) as usize,
                        _ => panic!("bad instruction"),
                    }

                    self.memory[addr] = result;
                    self.pc += 4;
                },

                9 => {
                    let (_, params, modes) = self.parse_opcode(self.pc, 1);
                    let op1: i64;

                    match modes[0] {
                        Mode::POSITION => op1 = self.memory[params[0] as usize],
                        Mode::IMMEDIATE => op1 = params[0],
                        Mode::RELATIVE => op1 = self.memory[(self.relative + params[0]) as usize],
                    }

                    self.relative += op1;
                    self.pc += 2;
                },

                99 => {
                    self.halt = true;
                    break;
                },

                _ => {
                    println!("BAD OPCODE: {}", instr);
                    panic!("bad machine state");
                },
            }
        }
    }

}

fn main() -> io::Result<()> {
    // Parse the input file to a memory vector
    let mut input: String = fs::read_to_string("input")?;
    input.pop();

    let split = input.split(",");
    let mut program: Vec<i64> = vec![];
    let mut memory: Vec<i64> = vec![];

    for element in split {
        if let Ok(num) = element.parse::<i64>() {
            program.push(num);
        }

        else {
            panic!("{} is not valid IntCode", element);
        }
    }

    // Add 10k memory
    for _ in 0 .. 10_000 {
        memory.push(0);
    }

    let mut part1: Computer = computer(program.clone(), memory.clone());
    part1.input.push(1);
    part1.run();
    println!("PART1: {:?}", part1.output);

    let mut part2: Computer = computer(program.clone(), memory.clone());
    part2.input.push(2);
    part2.run();
    println!("PART2: {:?}", part2.output);

    Ok(())
}
