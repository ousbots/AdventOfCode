use std::fs;
use std::io;


// Parses the given memory location for the opcode, parameters, and modes of a given length.
fn parse_opcode(memory: &Vec<i64>, pos: usize, len: usize) -> (i64, Vec<i64>, Vec<i64>) {
    const INSTR_MOD: i64 = 100;
    const MODE_MOD: i64 = 10;

    let mut opcode: i64 = memory[pos];
    let instr: i64 = opcode % INSTR_MOD;
    opcode = (opcode - instr) / INSTR_MOD;

    let mut modes: Vec<i64> = Vec::new();
    let mut params: Vec<i64> = Vec::new();

    for offset in 1 .. len+1 {
        params.push(memory[pos + offset]);

        modes.push(opcode % MODE_MOD);
        opcode = (opcode - (opcode % MODE_MOD)) / MODE_MOD;
    }

    return (instr, params, modes);
}


//  Runs the IntCode machine on the given memory space.
//
//  An opcode is broken down into: the least 2 digits are the instruction, and then the remaining
//  digits represent the parameter's mode in increasing signifigance.
//
//  A parameter can be given in two different modes: in immediate mode, the value of the parameter
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
//      99 [0] => halt
fn run(mut memory: Vec<i64>) {
    const POSITION_MODE: i64 = 0;
    let mut count: i64 = 0;

    println!("START");

    loop {
        let (instr, _, _) = parse_opcode(&memory, count as usize, 0);

        match instr {
            1 => {
                let (_, params, modes) = parse_opcode(&memory, count as usize, 3);
                let (op1, op2): (i64, i64);

                if modes[0] == POSITION_MODE {
                    op1 = memory[params[0] as usize];
                }

                else {
                    op1 = params[0];
                }

                if modes[1] == POSITION_MODE {
                    op2 = memory[params[1] as usize];
                }

                else {
                    op2 = params[1];
                }

                memory[params[2] as usize] = op1 + op2;
                count += 4;
            },

            2 => {
                let (_, params, modes) = parse_opcode(&memory, count as usize, 3);
                let (op1, op2): (i64, i64);

                if modes[0] == POSITION_MODE {
                    op1 = memory[params[0] as usize];
                }

                else {
                    op1 = params[0];
                }

                if modes[1] == POSITION_MODE {
                    op2 = memory[params[1] as usize];
                }

                else {
                    op2 = params[1];
                }

                memory[params[2] as usize] = op1 * op2;
                count += 4;
            },

            3 => {
                let (_, params, _) = parse_opcode(&memory, count as usize, 1);

                let mut raw_input = String::new();
                let input: i64;

                println!("INPUT INT:");
                if let Ok(_) = io::stdin().read_line(&mut raw_input) {
                    raw_input.pop();

                    if let Ok(num) = raw_input.parse::<i64>() {
                        input = num;
                    }

                    else {
                        println!("NOT AN INT, CRASH!");
                        panic!("bad input");
                    }
                }

                else {
                    println!("COULD NOT READ STDIN, CRASH!");
                    panic!("bad stdin");
                }

                memory[params[0] as usize] = input;
                count += 2;
            },

            4 => {
                let (_, params, modes) = parse_opcode(&memory, count as usize, 1);
                let op1: i64;

                if modes[0] == POSITION_MODE {
                    op1 = memory[params[0] as usize];
                }

                else {
                    op1 = params[0];
                }

                println!("PRINT {}", op1);
                count += 2;
            },

            5 => {
                let (_, params, modes) = parse_opcode(&memory, count as usize, 2);
                let (op1, op2): (i64, i64);

                if modes[0] == POSITION_MODE {
                    op1 = memory[params[0] as usize];
                }

                else {
                    op1 = params[0];
                }

                if modes[1] == POSITION_MODE {
                    op2 = memory[params[1] as usize];
                }

                else {
                    op2 = params[1];
                }

                if op1 != 0 {
                    count = op2;
                }

                else {
                    count += 3;
                }
            },

            6 => {
                let (_, params, modes) = parse_opcode(&memory, count as usize, 2);
                let (op1, op2): (i64, i64);

                if modes[0] == POSITION_MODE {
                    op1 = memory[params[0] as usize];
                }

                else {
                    op1 = params[0];
                }

                if modes[1] == POSITION_MODE {
                    op2 = memory[params[1] as usize];
                }

                else {
                    op2 = params[1];
                }

                if op1 == 0 {
                    count = op2;
                }

                else {
                    count += 3;
                }
            },

            7 => {
                let (_, params, modes) = parse_opcode(&memory, count as usize, 3);
                let (op1, op2): (i64, i64);

                if modes[0] == POSITION_MODE {
                    op1 = memory[params[0] as usize];
                }

                else {
                    op1 = params[0];
                }

                if modes[1] == POSITION_MODE {
                    op2 = memory[params[1] as usize];
                }

                else {
                    op2 = params[1];
                }

                let result: i64;
                if op1 < op2 {
                    result = 1;
                }

                else {
                    result = 0;
                }

                memory[params[2] as usize] = result;
                count += 4;
            },

            8 => {
                let (_, params, modes) = parse_opcode(&memory, count as usize, 3);
                let (op1, op2): (i64, i64);

                if modes[0] == POSITION_MODE {
                    op1 = memory[params[0] as usize];
                }

                else {
                    op1 = params[0];
                }

                if modes[1] == POSITION_MODE {
                    op2 = memory[params[1] as usize];
                }

                else {
                    op2 = params[1];
                }

                let result: i64;
                if op1 == op2 {
                    result = 1;
                }

                else {
                    result = 0;
                }

                memory[params[2] as usize] = result;
                count += 4;
            },

            99 => {
                println!("HALT");
                break;
            },

            _ => {
                println!("BAD OPCODE: {}", instr);
                panic!("bad machine state");
            },
        }
    }
}

fn main() -> io::Result<()> {
    // Parse the input file to a memory vector
    let mut input: String = fs::read_to_string("input")?;
    input.pop();

    let split = input.split(",");
    let mut memory: Vec<i64> = vec![];

    for element in split {
        if let Ok(num) = element.parse::<i64>() {
            memory.push(num);
        }

        else {
            panic!("{} is not valid IntCode", element);
        }
    }

    run(memory);

    Ok(())
}
