use std::process::exit;

use crate::tinyvm::bytecode::Instruction;

static STACK_SIZE:usize = 1000;

pub struct VM {
    ip: usize,
    code: Vec<u8>,
    stack: Vec<u8>,
    is_running: bool,
}

impl VM {
    pub fn new(program: Vec<u8>) -> Self {
        VM {
            ip: 0,
            code: program,
            stack: Vec::with_capacity(STACK_SIZE),
            is_running: false,
        }
    }

    fn disassemble(&mut self, opcode: u8) -> Instruction {
        println!("opcode = {opcode}", opcode = opcode);
        match opcode {
            0 => Instruction::NOP,
            1 => Instruction::POP,
            2 => Instruction::PUSH,
            3 => Instruction::PRINT,
            4 => Instruction::INC,
            5 => Instruction::ADD,
            6 => Instruction::SUB,
            7 => Instruction::XOR,
            10 => Instruction::JMPF,
            99 => Instruction::HALT,
            _ => Instruction::PANIC,
        }
    }

    fn execute(&mut self, instruction: Instruction) -> bool {
        match instruction {
            Instruction::HALT => {
                self.is_running = false;
                true
            },
            Instruction::PANIC => {
                false
            }
            Instruction::NOP => {
                self.ip += 1;
                true
            }
            Instruction::INC => {
                let val = self.stack.pop();
                match val {
                    Some(mut value) => {
                        value += 1;
                        self.stack.push(value);
                        self.ip += 1;
                        true
                    },
                    None => false
                }
            },
            Instruction::PRINT => {
                let val = self.stack.pop();
                match val {
                    Some(value) => {
                        println!("PRINT: {:#04x}", value);
                        self.ip += 1;
                        true
                    },
                    None => false
                }
            },
            Instruction::PUSH => {
                self.ip += 1;
                let val = self.code[self.ip];
                self.stack.push(val);
                self.ip += 1;
                true
            },
            Instruction::ADD => {
                self.ip += 1;
                let oper1 = self.code[self.ip];
                self.ip += 1;
                let oper2 = self.code[self.ip];
                self.stack.push(oper1 + oper2);
                self.ip += 1;
                true
            },
            Instruction::SUB => {
                self.ip += 1;
                let oper1 = self.code[self.ip];
                self.ip += 1;
                let oper2 = self.code[self.ip];
                self.stack.push(oper1 - oper2);
                self.ip += 1;
                true
            },
            Instruction::XOR => {
                self.ip += 1;
                let oper1 = self.code[self.ip];
                self.ip += 1;
                let oper2 = self.code[self.ip];
                self.stack.push(oper1^oper2);
                self.ip += 1;
                true
            },
            Instruction::JMPF => {
                self.ip += 1;
                let addr = self.code[self.ip];
                self.ip = addr as usize;
                true
            }
            _ => {
                false
            }
        }
    }

    pub fn cpu(&mut self) {
        self.is_running = true;
        while self.ip < self.code.len() {
            let opcode = self.code[self.ip];
            let instruction = self.disassemble(opcode);
            let res: bool = self.execute(instruction);
            if !self.is_running {
                println!("HALT received. VM stopped.");
                exit(0);
            }
            if !res {
                println!("Panic! Execution stopped");
                exit(1);
            }
        }
    }
    
}