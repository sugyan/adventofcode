#[derive(Default, Debug)]
pub struct Intcode {
    pub program: Vec<i32>,
    i: usize,
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
    pub fn run(&mut self, input: Option<i32>) -> Option<i32> {
        let mut output = None;
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
                    self.set_value(input.expect("input value"));
                }
                4 => {
                    output = Some(self.get_value(modes[0]));
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
        output
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
