use std::fs;
use std::io;

struct Amplifier {
    pc: usize,
    memory: Vec<i64>,
    input: Vec<i64>,
    output: Vec<i64>,
    halt: bool,
}

fn amplifier(memory: Vec<i64>) -> Amplifier {
    return Amplifier{
        pc: 0,
        memory: memory,
        input: vec![],
        output: vec![],
        halt: false,
    }
}


impl Amplifier {

    // Parses the given memory location for the opcode, parameters, and modes of a given length.
    fn parse_opcode(&self, pos: usize, len: usize) -> (i64, Vec<i64>, Vec<i64>) {
        const INSTR_MOD: i64 = 100;
        const MODE_MOD: i64 = 10;

        let mut opcode: i64 = self.memory[pos];
        let instr: i64 = opcode % INSTR_MOD;
        opcode = (opcode - instr) / INSTR_MOD;

        let mut modes: Vec<i64> = Vec::new();
        let mut params: Vec<i64> = Vec::new();

        for offset in 1 .. len+1 {
            params.push(self.memory[pos + offset]);

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
    fn run(&mut self) {
        const POSITION_MODE: i64 = 0;

        loop {
            let (instr, _, _) = self.parse_opcode(self.pc, 0);

            match instr {
                1 => {
                    let (_, params, modes) = self.parse_opcode(self.pc, 3);
                    let (op1, op2): (i64, i64);

                    if modes[0] == POSITION_MODE {
                        op1 = self.memory[params[0] as usize];
                    }

                    else {
                        op1 = params[0];
                    }

                    if modes[1] == POSITION_MODE {
                        op2 = self.memory[params[1] as usize];
                    }

                    else {
                        op2 = params[1];
                    }

                    self.memory[params[2] as usize] = op1 + op2;
                    self.pc += 4;
                },

                2 => {
                    let (_, params, modes) = self.parse_opcode(self.pc, 3);
                    let (op1, op2): (i64, i64);

                    if modes[0] == POSITION_MODE {
                        op1 = self.memory[params[0] as usize];
                    }

                    else {
                        op1 = params[0];
                    }

                    if modes[1] == POSITION_MODE {
                        op2 = self.memory[params[1] as usize];
                    }

                    else {
                        op2 = params[1];
                    }

                    self.memory[params[2] as usize] = op1 * op2;
                    self.pc += 4;
                },

                3 => {
                    let (_, params, _) = self.parse_opcode(self.pc, 1);

                    if self.input.len() == 0 {
                        return;
                    }

                    let value: i64 = self.input[0];
                    self.input.remove(0);

                    self.memory[params[0] as usize] = value;
                    self.pc += 2;
                },

                4 => {
                    let (_, params, modes) = self.parse_opcode(self.pc, 1);
                    let op1: i64;

                    if modes[0] == POSITION_MODE {
                        op1 = self.memory[params[0] as usize];
                    }

                    else {
                        op1 = params[0];
                    }

                    self.output.push(op1);
                    self.pc += 2;
                },

                5 => {
                    let (_, params, modes) = self.parse_opcode(self.pc, 2);
                    let (op1, op2): (i64, i64);

                    if modes[0] == POSITION_MODE {
                        op1 = self.memory[params[0] as usize];
                    }

                    else {
                        op1 = params[0];
                    }

                    if modes[1] == POSITION_MODE {
                        op2 = self.memory[params[1] as usize];
                    }

                    else {
                        op2 = params[1];
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

                    if modes[0] == POSITION_MODE {
                        op1 = self.memory[params[0] as usize];
                    }

                    else {
                        op1 = params[0];
                    }

                    if modes[1] == POSITION_MODE {
                        op2 = self.memory[params[1] as usize];
                    }

                    else {
                        op2 = params[1];
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

                    if modes[0] == POSITION_MODE {
                        op1 = self.memory[params[0] as usize];
                    }

                    else {
                        op1 = params[0];
                    }

                    if modes[1] == POSITION_MODE {
                        op2 = self.memory[params[1] as usize];
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

                    self.memory[params[2] as usize] = result;
                    self.pc += 4;
                },

                8 => {
                    let (_, params, modes) = self.parse_opcode(self.pc, 3);
                    let (op1, op2): (i64, i64);

                    if modes[0] == POSITION_MODE {
                        op1 = self.memory[params[0] as usize];
                    }

                    else {
                        op1 = params[0];
                    }

                    if modes[1] == POSITION_MODE {
                        op2 = self.memory[params[1] as usize];
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

                    self.memory[params[2] as usize] = result;
                    self.pc += 4;
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
    let mut memory: Vec<i64> = vec![];

    for element in split {
        if let Ok(num) = element.parse::<i64>() {
            memory.push(num);
        }

        else {
            panic!("{} is not valid IntCode", element);
        }
    }

    //##########
    //# PART 1 #
    //##########
    println!("\nPART 1\n");

    // Build all possible inputs
    let mut inputs: Vec<Vec<i64>> = Vec::new();
    for phasea in 0 .. 5 {
        for phaseb in 0 .. 5 {
            for phasec in 0 .. 5 {
                for phased in 0 .. 5 {
                    for phasee in 0 .. 5 {
                        if phasea == phaseb || phasea == phasec || phasea == phased || phasea == phasee {
                            continue;
                        }

                        if phaseb == phasec || phaseb == phased || phaseb == phasee {
                            continue;
                        }

                        if phasec == phased || phasec == phasee {
                            continue;
                        }

                        if phased == phasee {
                            continue;
                        }

                        inputs.push(vec![phasea, phaseb, phasec, phased, phasee]);
                    }
                }
            }
        }
    }

    let mut max_output: i64 = 0;
    let mut max_input: Vec<i64> = vec![];

    for input in inputs {
        let mut amp_a: Amplifier = amplifier(memory.clone());
        let mut amp_b: Amplifier = amplifier(memory.clone());
        let mut amp_c: Amplifier = amplifier(memory.clone());
        let mut amp_d: Amplifier = amplifier(memory.clone());
        let mut amp_e: Amplifier = amplifier(memory.clone());

        amp_a.input.append(&mut vec![input[0], 0]);
        amp_a.run();

        amp_b.input.append(&mut vec![input[1], amp_a.output[0]]);
        amp_b.run();

        amp_c.input.append(&mut vec![input[2], amp_b.output[0]]);
        amp_c.run();

        amp_d.input.append(&mut vec![input[3], amp_c.output[0]]);
        amp_d.run();

        amp_e.input.append(&mut vec![input[4], amp_d.output[0]]);
        amp_e.run();

        if amp_e.output[0] > max_output {
            max_output = amp_e.output[0];
            max_input = input.clone();
        }
    }

    println!("max thrust {}", max_output);
    println!("max input {:?}", max_input);

    //##########
    //# PART 2 #
    //##########
    println!("\nPART 2\n");

    // Build all possible inputs
    let mut inputs: Vec<Vec<i64>> = Vec::new();
    for phasea in 4 .. 10 {
        for phaseb in 4 .. 10 {
            for phasec in 4 .. 10 {
                for phased in 4 .. 10 {
                    for phasee in 4 .. 10 {
                        if phasea == phaseb || phasea == phasec || phasea == phased || phasea == phasee {
                            continue;
                        }

                        if phaseb == phasec || phaseb == phased || phaseb == phasee {
                            continue;
                        }

                        if phasec == phased || phasec == phasee {
                            continue;
                        }

                        if phased == phasee {
                            continue;
                        }

                        inputs.push(vec![phasea, phaseb, phasec, phased, phasee]);
                    }
                }
            }
        }
    }

    let mut max_output: i64 = 0;
    let mut max_input: Vec<i64> = vec![];

    for input in inputs {
        let mut amp_a: Amplifier = amplifier(memory.clone());
        let mut amp_b: Amplifier = amplifier(memory.clone());
        let mut amp_c: Amplifier = amplifier(memory.clone());
        let mut amp_d: Amplifier = amplifier(memory.clone());
        let mut amp_e: Amplifier = amplifier(memory.clone());
        amp_a.input.push(input[0]);
        amp_b.input.push(input[1]);
        amp_c.input.push(input[2]);
        amp_d.input.push(input[3]);
        amp_e.input.push(input[4]);
        amp_e.output.push(0);

        while amp_e.halt == false {
            amp_a.input.push(amp_e.output.pop().unwrap());
            amp_a.run();

            amp_b.input.push(amp_a.output.pop().unwrap());
            amp_b.run();

            amp_c.input.push(amp_b.output.pop().unwrap());
            amp_c.run();

            amp_d.input.push(amp_c.output.pop().unwrap());
            amp_d.run();

            amp_e.input.push(amp_d.output.pop().unwrap());
            amp_e.run();

            if amp_a.halt || amp_b.halt || amp_c.halt || amp_d.halt || amp_e.halt {
                break;
            }
        }

        if amp_e.output[0] > max_output {
            max_output = amp_e.output[0];
            max_input = input.clone();
        }
    }

    println!("max thrust {}", max_output);
    println!("max input {:?}", max_input);

    Ok(())
}
