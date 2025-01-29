use std::fmt::Display;

use bevy::prelude::{Component, Resource};

use crate::loader::RawInput;

#[derive(Debug, Clone, Resource, Component)]
pub struct Input {
    pub registers: Registers,
    pub program: Vec<Instruction>,
    pub raw_program: Vec<u8>,
}

impl Input {
    pub fn parse(input: &RawInput) -> Self {
        let mut input = input.split(|c| *c == b'\n');

        let a = input
            .next()
            .map(|line| {
                line.split(|c| *c == b':')
                    .nth(1)
                    .map(|val| String::from_utf8_lossy(val.trim_ascii()).parse().unwrap())
                    .unwrap()
            })
            .unwrap();
        let b = input
            .next()
            .map(|line| {
                line.split(|c| *c == b':')
                    .nth(1)
                    .map(|val| String::from_utf8_lossy(val.trim_ascii()).parse().unwrap())
                    .unwrap()
            })
            .unwrap();
        let c = input
            .next()
            .map(|line| {
                line.split(|c| *c == b':')
                    .nth(1)
                    .map(|val| String::from_utf8_lossy(val.trim_ascii()).parse().unwrap())
                    .unwrap()
            })
            .unwrap();
        _ = input.next();

        let registers = Registers { pc: 0, a, b, c };

        let raw_program = input
            .next()
            .map(|line| {
                line.split(|c| *c == b':')
                    .nth(1)
                    .unwrap()
                    .split(|c| *c == b',')
                    .map(|byte| String::from_utf8_lossy(byte.trim_ascii()).parse::<u8>())
                    .collect::<Result<Vec<u8>, _>>()
                    .unwrap()
            })
            .unwrap();
        let program = raw_program
            .chunks(2)
            .map(|chunk| match chunk[0] {
                0 => Instruction::Adv(chunk[1].into()),
                1 => Instruction::Bxl(chunk[1]),
                2 => Instruction::Bst(chunk[1].into()),
                3 => Instruction::Jnz(chunk[1]),
                4 => Instruction::Bxc(chunk[1]),
                5 => Instruction::Out(chunk[1].into()),
                6 => Instruction::Bdv(chunk[1].into()),
                7 => Instruction::Cdv(chunk[1].into()),
                _ => unreachable!("AoC inputs are well formed"),
            })
            .collect::<Vec<_>>();

        Self {
            registers,
            program,
            raw_program,
        }
    }

    pub fn execute(&mut self) -> Vec<u8> {
        let mut output = vec![];
        while self.registers.pc < self.program.len() {
            let instruction = self.program[self.registers.pc];
            if let Some(out) = instruction.execute(&mut self.registers) {
                output.push(out);
            }
        }
        output
    }

    pub fn debug(&self) -> Debugger {
        Debugger {
            vm: self.clone(),
            queue: vec![0],
            buffer: vec![],
            i: self.program.len() * 2,
            a: 0,
            j: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Registers {
    pub pc: usize,
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Adv(ComboOperand),
    Bxl(u8),
    Bst(ComboOperand),
    Jnz(u8),
    #[allow(dead_code)]
    Bxc(u8),
    Out(ComboOperand),
    Bdv(ComboOperand),
    Cdv(ComboOperand),
}

impl Instruction {
    pub fn execute(&self, registers: &mut Registers) -> Option<u8> {
        registers.pc += 1;
        let mut res = None;
        match self {
            Instruction::Adv(combo) => {
                registers.a /= 2usize.pow(u32::try_from(combo.get_value(registers)).unwrap())
            }
            Instruction::Bxl(literal) => registers.b ^= usize::from(*literal),
            Instruction::Bst(combo) => registers.b = combo.get_value(registers) % 8,
            Instruction::Jnz(literal) => {
                if registers.a != 0 {
                    registers.pc = usize::from(*literal);
                }
            }
            Instruction::Bxc(_) => registers.b ^= registers.c,
            Instruction::Out(combo) => {
                res.replace(combo.get_value(registers) % 8);
            }
            Instruction::Bdv(combo) => {
                registers.b =
                    registers.a / 2usize.pow(u32::try_from(combo.get_value(registers)).unwrap())
            }
            Instruction::Cdv(combo) => {
                registers.c =
                    registers.a / 2usize.pow(u32::try_from(combo.get_value(registers)).unwrap())
            }
        }
        res.map(|val| u8::try_from(val).unwrap())
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Adv(combo) => write!(f, "adv {}", combo),
            Self::Bxl(literal) => write!(f, "blx {}", literal),
            Self::Bst(combo) => write!(f, "bst {}", combo),
            Self::Jnz(literal) => write!(f, "jnz {}", literal),
            Self::Bxc(_) => write!(f, "bxc"),
            Self::Out(combo) => write!(f, "out {}", combo),
            Self::Bdv(combo) => write!(f, "bdv {}", combo),
            Self::Cdv(combo) => write!(f, "cdv {}", combo),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ComboOperand {
    Literal(u8),
    RegisterA,
    RegisterB,
    RegisterC,
    Reserved,
}

impl ComboOperand {
    pub fn get_value(&self, registers: &Registers) -> usize {
        match self {
            ComboOperand::Literal(val) => usize::from(*val),
            ComboOperand::RegisterA => registers.a,
            ComboOperand::RegisterB => registers.b,
            ComboOperand::RegisterC => registers.c,
            ComboOperand::Reserved => unreachable!("Reserved combo operand is not usable"),
        }
    }
}

impl From<u8> for ComboOperand {
    fn from(value: u8) -> Self {
        match value {
            val @ 0..=3 => Self::Literal(val),
            4 => Self::RegisterA,
            5 => Self::RegisterB,
            6 => Self::RegisterC,
            7 => Self::Reserved,
            _ => unreachable!("AoC inputs are well formed"),
        }
    }
}

impl Display for ComboOperand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(literal) => write!(f, "{:?}", literal),
            Self::RegisterA => write!(f, "$A"),
            Self::RegisterB => write!(f, "$B"),
            Self::RegisterC => write!(f, "$C"),
            Self::Reserved => write!(f, "$X"),
        }
    }
}

#[derive(Debug, Component)]
pub struct Debugger {
    pub vm: Input,
    pub queue: Vec<usize>,
    pub buffer: Vec<usize>,
    pub i: usize,
    pub j: usize,
    pub a: usize,
}

impl Debugger {
    pub fn step(&mut self) {
        if self.i != 0 {
            let i = self.i - 1;
            let a = self.a;
            let j = self.j;

            let mut vm = self.vm.clone();

            let test = j << (i * 3);
            vm.registers.a = a | test;

            let out = vm.execute();

            if let Some(digit) = out.get(i).copied() {
                if u64::from(digit) == u64::from(self.vm.raw_program[i]) {
                    self.buffer.push(a | test);
                }
            }

            self.j += 1;
            if j == 8 {
                self.j = 0;
                if let Some(a) = self.queue.pop() {
                    self.a = a;
                } else {
                    self.buffer.reverse();
                    std::mem::swap(&mut self.buffer, &mut self.queue);
                    self.a = self.queue.pop().unwrap();
                    self.i -= 1;
                }
            }
        }
    }
}
