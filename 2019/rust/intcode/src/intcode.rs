#[derive(Clone, Default)]
pub struct Intcode {
    pub program: Vec<i32>,
    i: usize,
}

#[derive(Debug, PartialEq)]
pub enum Result {
    WaitInput,
    Output(i32),
    Halted,
}

#[derive(Clone, Copy)]
enum Mode {
    Position = 0,
    Immediate = 1,
}

impl Intcode {
    #[must_use]
    pub fn new(program: &[i32]) -> Self {
        Self {
            program: program.to_owned(),
            ..Intcode::default()
        }
    }
    /// Run program
    ///
    /// # Panics
    ///
    /// Panics if opecode is unknown
    pub fn run(&mut self, inputs: Vec<i32>) -> Result {
        let mut inputs = inputs.into_iter();
        loop {
            let modes = match self.program[self.i] / 100 {
                0 => [Mode::Position, Mode::Position],
                1 => [Mode::Immediate, Mode::Position],
                10 => [Mode::Position, Mode::Immediate],
                11 => [Mode::Immediate, Mode::Immediate],
                _ => unimplemented!(),
            };
            let opcode = self.program[self.i] % 100;
            match opcode {
                1 => {
                    let v1 = self.get_value(modes[0]);
                    let v2 = self.get_value(modes[1]);
                    self.set_value(v1 + v2);
                }
                2 => {
                    let v1 = self.get_value(modes[0]);
                    let v2 = self.get_value(modes[1]);
                    self.set_value(v1 * v2);
                }
                3 => {
                    if let Some(input) = inputs.next() {
                        self.set_value(input);
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
                        self.i = v2 as usize;
                        continue;
                    }
                }
                6 => {
                    let v1 = self.get_value(modes[0]);
                    let v2 = self.get_value(modes[1]);
                    if v1 == 0 {
                        self.i = v2 as usize;
                        continue;
                    }
                }
                7 => {
                    let v1 = self.get_value(modes[0]);
                    let v2 = self.get_value(modes[1]);
                    self.set_value(if v1 < v2 { 1 } else { 0 });
                }
                8 => {
                    let v1 = self.get_value(modes[0]);
                    let v2 = self.get_value(modes[1]);
                    self.set_value(if v1 == v2 { 1 } else { 0 });
                }
                99 => break,
                _ => unimplemented!(),
            }
            self.i += 1;
        }
        Result::Halted
    }
    fn get_value(&mut self, mode: Mode) -> i32 {
        self.i += 1;
        match mode {
            Mode::Position => self.program[self.program[self.i] as usize],
            Mode::Immediate => self.program[self.i],
        }
    }
    fn set_value(&mut self, val: i32) {
        self.i += 1;
        let pos = self.program[self.i] as usize;
        self.program[pos] = val;
    }
}

mod tests;
