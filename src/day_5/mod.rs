use std::fs;
use std::io::stdin;

pub fn run(input: &str) {
    let inputs = load_inputs(input);
    part_1(&inputs);
}

fn load_inputs(file_name: &str) -> Vec<i32> {
    let content = fs::read_to_string(file_name).expect("Could not read file");
    return content
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
}

fn part_1(inputs: &Vec<i32>) {
    let mut memory = inputs.clone();
    let return_code = execute(&mut memory);
    println!("Return code: {}", return_code);
}

fn execute(memory: &mut Vec<i32>) -> i8 {
    let mut p = 0;

    loop {
        let (opcode, am, bm, rm) = parse_instruction(memory[p]);
        println!(
            "IN {} -> op[{}] a[{}] b[{}] c[{}]",
            memory[p], opcode, am, bm, rm
        );
        match opcode {
            // Addition
            1 => {
                let a = get_value(memory, am, memory[p + 1]);
                let b = get_value(memory, bm, memory[p + 2]);
                let r = memory[p + 3] as usize;
                memory[r] = a + b;
                p += 4;
            }
            // Multiplication
            2 => {
                let a = get_value(memory, am, memory[p + 1]);
                let b = get_value(memory, bm, memory[p + 2]);
                let r = memory[p + 3] as usize;
                memory[r] = a * b;
                p += 4;
            }
            // Get input
            3 => {
                let r = memory[p + 1] as usize;
                let v = get_input();
                memory[r] = v;
                println!(" {} = {}", r, v);
                p += 2;
            }
            // Print
            4 => {
                let a = get_value(memory, am, memory[p + 1]);
                println!("INFO: {}", a);
                p += 2;
            }
            // Jump if true
            5 => {
                let a = get_value(memory, am, memory[p + 1]);
                let b = get_value(memory, bm, memory[p + 2]) as usize;
                if a != 0 {
                    p = b;
                } else {
                    p += 3;
                }
            }
            // Jump if false
            6 => {
                let a = get_value(memory, am, memory[p + 1]);
                let b = get_value(memory, bm, memory[p + 2]) as usize;
                if a == 0 {
                    p = b;
                } else {
                    p += 3;
                }
            }
            // Less than
            7 => {
                let a = get_value(memory, am, memory[p + 1]);
                let b = get_value(memory, bm, memory[p + 2]);
                let r = memory[p + 3] as usize;
                memory[r] = if a < b { 1 } else { 0 };
                p += 4;
            }
            // Equals
            8 => {
                let a = get_value(memory, am, memory[p + 1]);
                let b = get_value(memory, bm, memory[p + 2]);
                let r = memory[p + 3] as usize;
                memory[r] = if a == b { 1 } else { 0 };
                p += 4;
            }
            99 => {
                return 0;
            }
            _ => {
                return -1;
            }
        }
    }
}

fn parse_instruction(mut instruction: i32) -> (i32, i32, i32, i32) {
    let opcode = instruction % 100;
    instruction /= 100;
    let am = instruction % 10;
    instruction /= 10;
    let bm = instruction % 10;
    instruction /= 10;
    let rm = instruction % 10;
    return (opcode, am, bm, rm);
}

fn get_value(memory: &Vec<i32>, mode: i32, parameter: i32) -> i32 {
    let v: i32;
    if mode == 1 {
        v = parameter;
    } else {
        v = memory[parameter as usize];
    }
    println!("  get_value(memory, {}, {}) -> {}", mode, parameter, v);
    return v;
}

fn get_input() -> i32 {
    print!("INPUT: ");
    let mut buffer = String::new();
    stdin()
        .read_line(&mut buffer)
        .expect("Input not a correct string.");
    if let Some('\n') = buffer.chars().next_back() {
        buffer.pop();
    }
    if let Some('\r') = buffer.chars().next_back() {
        buffer.pop();
    }
    return buffer.parse().unwrap();
}
