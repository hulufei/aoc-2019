// https://github.com/michaelmelanson/advent-of-code-2019/blob/master/src/intcode/mod.rs
pub type IntcodeProgram = Vec<isize>;
type IntcodeMemory = Vec<isize>;

pub fn intcode_parser(input: &str) -> IntcodeProgram {
    input
        .split(',')
        .map(|s| s.parse::<isize>().expect("Invalid intcode"))
        .collect()
}

#[derive(Debug)]
pub enum Parameter {
    Position(usize),
    Immediate(isize),
    Relative(isize),
}

impl Parameter {
    pub fn new(mode: isize, value: isize) -> Parameter {
        match mode {
            0 => Parameter::Position(value as usize),
            1 => Parameter::Immediate(value),
            2 => Parameter::Relative(value),
            _ => unimplemented!(),
        }
    }

    pub fn resolve(&self, memory: &IntcodeMemory, relative_base: isize) -> isize {
        match self {
            Parameter::Immediate(value) => *value,
            Parameter::Position(position) => memory[*position],
            Parameter::Relative(value) => memory[(*value + relative_base) as usize],
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Add(Parameter, Parameter, Parameter),
    Multiply(Parameter, Parameter, Parameter),
    Input(Parameter),
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, Parameter),
    Equals(Parameter, Parameter, Parameter),
    SetRelativeBase(Parameter),
    Halt,
}

#[derive(Debug, PartialEq)]
pub enum Action {
    Output(isize),
    Halt,
}

pub struct Machine {
    memory: IntcodeMemory,
    ip: usize,
    inputs: Vec<isize>,
    relative_base: isize,
}

impl Machine {
    pub fn new(memory: &IntcodeMemory) -> Self {
        Machine {
            memory: memory.clone(),
            ip: 0,
            inputs: vec![],
            relative_base: 0,
        }
    }

    pub fn with_capacity(memory: &IntcodeMemory, capacity: usize) -> Self {
        let mut memory = memory.clone();
        memory.resize(capacity, 0);
        Machine {
            memory,
            ip: 0,
            inputs: vec![],
            relative_base: 0,
        }
    }

    fn read(&mut self) -> isize {
        let value = self.memory[self.ip];
        self.ip += 1;
        value
    }

    fn jump(&mut self, address: usize) {
        self.ip = address;
    }

    fn read_input(&mut self) -> isize {
        self.inputs.remove(0)
    }

    pub fn push_input(&mut self, input: isize) {
        self.inputs.push(input);
    }

    fn write(&mut self, value: isize, parameter: &Parameter) {
        // Parameters that an instruction writes to will never be in immediate mode.
        match parameter {
            Parameter::Position(position) => {
                self.memory[*position] = value;
            }
            Parameter::Relative(position) => {
                let position = *position + self.relative_base;
                self.memory[position as usize] = value;
            }
            Parameter::Immediate(_) => unimplemented!(),
        }
    }

    fn next_instruction(&mut self) -> Instruction {
        let instruction_value = self.read();
        let opcode = instruction_value % 100;
        let first_mode = instruction_value / 100 % 10;
        let second_mode = instruction_value / 1000 % 10;
        let third_mode = instruction_value / 10000 % 10;

        match opcode {
            1 => Instruction::Add(
                Parameter::new(first_mode, self.read()),
                Parameter::new(second_mode, self.read()),
                Parameter::new(third_mode, self.read()),
            ),
            2 => Instruction::Multiply(
                Parameter::new(first_mode, self.read()),
                Parameter::new(second_mode, self.read()),
                Parameter::new(third_mode, self.read()),
            ),
            3 => Instruction::Input(Parameter::new(first_mode, self.read())),
            4 => Instruction::Output(Parameter::new(first_mode, self.read())),
            5 => Instruction::JumpIfTrue(
                Parameter::new(first_mode, self.read()),
                Parameter::new(second_mode, self.read()),
            ),
            6 => Instruction::JumpIfFalse(
                Parameter::new(first_mode, self.read()),
                Parameter::new(second_mode, self.read()),
            ),
            7 => Instruction::LessThan(
                Parameter::new(first_mode, self.read()),
                Parameter::new(second_mode, self.read()),
                Parameter::new(third_mode, self.read()),
            ),
            8 => Instruction::Equals(
                Parameter::new(first_mode, self.read()),
                Parameter::new(second_mode, self.read()),
                Parameter::new(third_mode, self.read()),
            ),
            9 => Instruction::SetRelativeBase(Parameter::new(first_mode, self.read())),
            99 => Instruction::Halt,
            _ => unimplemented!(),
        }
    }

    pub fn step(&mut self) -> Option<Action> {
        let mut action = None;
        let instruction = self.next_instruction();
        match instruction {
            Instruction::Add(lhs, rhs, output) => {
                let lhs = lhs.resolve(&self.memory, self.relative_base);
                let rhs = rhs.resolve(&self.memory, self.relative_base);
                self.write(lhs + rhs, &output);
            }
            Instruction::Multiply(lhs, rhs, output) => {
                let lhs = lhs.resolve(&self.memory, self.relative_base);
                let rhs = rhs.resolve(&self.memory, self.relative_base);
                self.write(lhs * rhs, &output);
            }
            Instruction::Input(output) => {
                let value = self.read_input();
                self.write(value, &output);
            }
            Instruction::Output(value) => {
                action = Some(Action::Output(
                    value.resolve(&self.memory, self.relative_base),
                ));
            }
            Instruction::JumpIfTrue(value, target) => {
                let value = value.resolve(&self.memory, self.relative_base);
                if value != 0 {
                    let target = target.resolve(&self.memory, self.relative_base) as usize;
                    self.jump(target);
                }
            }
            Instruction::JumpIfFalse(value, target) => {
                let value = value.resolve(&self.memory, self.relative_base);
                if value == 0 {
                    let target = target.resolve(&self.memory, self.relative_base) as usize;
                    self.jump(target);
                }
            }
            Instruction::LessThan(lhs, rhs, output) => {
                let lhs = lhs.resolve(&self.memory, self.relative_base);
                let rhs = rhs.resolve(&self.memory, self.relative_base);
                if lhs < rhs {
                    self.write(1, &output);
                } else {
                    self.write(0, &output);
                }
            }
            Instruction::Equals(lhs, rhs, output) => {
                let lhs = lhs.resolve(&self.memory, self.relative_base);
                let rhs = rhs.resolve(&self.memory, self.relative_base);
                if lhs == rhs {
                    self.write(1, &output);
                } else {
                    self.write(0, &output);
                }
            }
            Instruction::SetRelativeBase(value) => {
                let value = value.resolve(&self.memory, self.relative_base);
                self.relative_base += value;
            }
            Instruction::Halt => {
                action = Some(Action::Halt);
            }
        }
        action
    }

    pub fn run(&mut self) -> Action {
        loop {
            let action = self.step();
            if let Some(action) = action {
                return action;
            }
        }
    }
}

#[test]
fn test_relative_mode() {
    let mut program = vec![
        109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    ];
    let mut machine = Machine::new(&program);
    assert_eq!(machine.run(), Action::Output(109));

    program = vec![1102, 34_915_192, 34_915_192, 7, 4, 7, 99, 0];
    machine = Machine::new(&program);
    assert_eq!(machine.run(), Action::Output(1_219_070_632_396_864));

    program = vec![104, 1_125_899_906_842_624, 99];
    machine = Machine::new(&program);
    assert_eq!(machine.run(), Action::Output(1_125_899_906_842_624));
}

#[test]
fn test_init_with_capacity() {
    let program = vec![1, 2, 3];
    let machine = Machine::with_capacity(&program, 5);
    assert_eq!(machine.memory, vec![1, 2, 3, 0, 0]);
}
