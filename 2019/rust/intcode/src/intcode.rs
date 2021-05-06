use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(Clone, Default)]
pub struct Intcode {
    pub memory: HashMap<usize, i64>,
    i: usize,
    base: i64,
}

#[derive(Debug, PartialEq)]
pub enum Result {
    WaitInput,
    Output(i64),
    Halted,
}

#[derive(Clone, Copy)]
enum Mode {
    Position = 0,
    Immediate = 1,
    Relative = 2,
}

impl Intcode {
    #[must_use]
    pub fn new(program: &[i64]) -> Self {
        let mut memory = HashMap::new();
        for (i, &value) in program.iter().enumerate() {
            memory.insert(i, value);
        }
        Self {
            memory,
            ..Intcode::default()
        }
    }
    /// Run program
    ///
    /// # Panics
    ///
    /// Panics if opecode is unknown
    pub fn run(&mut self, inputs: Vec<i64>) -> Result {
        let mut inputs = inputs.into_iter();
        loop {
            let digits = *self.memory.get(&self.i).unwrap_or(&0);
            let opcode = digits % 100;
            let modes = (0..3)
                .scan(digits / 100, |state, _| {
                    let mode = match *state % 10 {
                        0 => Mode::Position,
                        1 => Mode::Immediate,
                        2 => Mode::Relative,
                        _ => unimplemented!(),
                    };
                    *state /= 10;
                    Some(mode)
                })
                .collect::<Vec<_>>();
            match opcode {
                1 => {
                    let v1 = self.get_value(modes[0]);
                    let v2 = self.get_value(modes[1]);
                    self.set_value(v1 + v2, modes[2]);
                }
                2 => {
                    let v1 = self.get_value(modes[0]);
                    let v2 = self.get_value(modes[1]);
                    self.set_value(v1 * v2, modes[2]);
                }
                3 => {
                    if let Some(input) = inputs.next() {
                        self.set_value(input, modes[0]);
                    } else {
                        return Result::WaitInput;
                    }
                }
                4 => {
                    let out = self.get_value(modes[0]);
                    self.i += 1;
                    return Result::Output(out);
                }
                5 => {
                    let v1 = self.get_value(modes[0]);
                    let v2 = self.get_value(modes[1]);
                    if v1 != 0 {
                        if let Ok(i) = usize::try_from(v2) {
                            self.i = i;
                        } else {
                            // TODO
                        }
                        continue;
                    }
                }
                6 => {
                    let v1 = self.get_value(modes[0]);
                    let v2 = self.get_value(modes[1]);
                    if v1 == 0 {
                        if let Ok(i) = usize::try_from(v2) {
                            self.i = i;
                        } else {
                            // TODO
                        }
                        continue;
                    }
                }
                7 => {
                    let v1 = self.get_value(modes[0]);
                    let v2 = self.get_value(modes[1]);
                    self.set_value(if v1 < v2 { 1 } else { 0 }, modes[2]);
                }
                8 => {
                    let v1 = self.get_value(modes[0]);
                    let v2 = self.get_value(modes[1]);
                    self.set_value(if v1 == v2 { 1 } else { 0 }, modes[2]);
                }
                9 => {
                    self.base += self.get_value(modes[0]);
                }
                99 => break,
                _ => unimplemented!(),
            }
            self.i += 1;
        }
        Result::Halted
    }
    fn get_value(&mut self, mode: Mode) -> i64 {
        self.i += 1;
        match mode {
            Mode::Position => {
                if let Some(&value) = self.memory.get(&self.i) {
                    if let Ok(pos) = usize::try_from(value) {
                        *self.memory.get(&pos).unwrap_or(&0)
                    } else {
                        unreachable!() // TODO
                    }
                } else {
                    unreachable!() // TODO
                }
            }
            Mode::Immediate => *self.memory.get(&self.i).unwrap(),
            Mode::Relative => {
                if let Some(&value) = self.memory.get(&self.i) {
                    if let Ok(pos) = usize::try_from(value + self.base) {
                        *self.memory.get(&pos).unwrap()
                    } else {
                        unreachable!() // TODO
                    }
                } else {
                    unreachable!() // TODO
                }
            }
        }
    }
    fn set_value(&mut self, val: i64, mode: Mode) {
        self.i += 1;
        match mode {
            Mode::Position => {
                if let Some(&value) = self.memory.get(&self.i) {
                    if let Ok(pos) = usize::try_from(value) {
                        self.memory.insert(pos, val);
                    } else {
                        // TODO
                    }
                }
            }
            Mode::Immediate => {
                // TODO
            }
            Mode::Relative => {
                if let Some(&value) = self.memory.get(&self.i) {
                    if let Ok(pos) = usize::try_from(value + self.base) {
                        self.memory.insert(pos, val);
                    } else {
                        // TODO
                    }
                }
            }
        }
    }
}

mod tests;
