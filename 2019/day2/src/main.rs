use std::fs;
use std::io;

//  Runs the intcode machine on the given memory space.
//
//  Notes:
//  output is at memory[0]
//  inputs are memory[1] and memory[2]
//  opcodes: opcode, src1, src2, dst
//      1 => add the values at locations given by src1 and src2 and store at dst
//      2 => multiply the values at locations given by src1 and src2 and store at dst
//      99 => halt
fn run(mut memory: Vec<u64>) -> u64 {
    let mut count: u64 = 0;

    loop {
        let opcode = memory[count as usize];
        let src1 = memory[count as usize + 1];
        let src2 = memory[count as usize + 2];
        let dst = memory[count as usize + 3];

        match opcode {
            1 => {
                memory[dst as usize] = memory[src1 as usize] + memory[src2 as usize];
                count += 4;
            },

            2 => {
                memory[dst as usize] = memory[src1 as usize] * memory[src2 as usize];
                count += 4;
            },

            99 => {
                break;
            },

            _ => {
                continue;
            },
        }
    }

    return memory[0];
}


fn main() -> io::Result<()> {
    let mut input: String = fs::read_to_string("input")?;
    input.pop();

    let split = input.split(",");
    let mut memory: Vec<u64> = vec![];

    for element in split {
        memory.push(element.parse::<u64>().unwrap())
    }

    let desired_output = 19690720;

    for noun in 0..99 {
        for verb in 0 .. 99 {
            memory[1] = noun;
            memory[2] = verb;

            if run(memory.clone()) == desired_output {
                println!("answer: {}", 100 * noun + verb);
            }
        }
    }

    Ok(())
}
