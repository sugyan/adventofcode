use std::usize;

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
                    output = Some(self.get_value(Mode::Position));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day02_example_1() {
        {
            let mut computer = Intcode::new(&[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
            computer.run(None);
            assert_eq!(
                vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
                computer.program
            );
        }
        {
            let mut computer = Intcode::new(&[1, 0, 0, 0, 99]);
            computer.run(None);
            assert_eq!(vec![2, 0, 0, 0, 99], computer.program);
        }
        {
            let mut computer = Intcode::new(&[2, 3, 0, 3, 99]);
            computer.run(None);
            assert_eq!(vec![2, 3, 0, 6, 99], computer.program);
        }
        {
            let mut computer = Intcode::new(&[2, 4, 4, 5, 99, 0]);
            computer.run(None);
            assert_eq!(vec![2, 4, 4, 5, 99, 9801], computer.program);
        }
        {
            let mut computer = Intcode::new(&[1, 1, 1, 4, 99, 5, 6, 0, 99]);
            computer.run(None);
            assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], computer.program);
        }
    }

    #[test]
    fn day05_example_1() {
        {
            let mut computer = Intcode::new(&[3, 0, 4, 0, 99]);
            let output = computer.run(Some(42));
            assert_eq!(Some(42), output);
        }
        {
            let mut computer = Intcode::new(&[1002, 4, 3, 4, 33]);
            computer.run(None);
            assert_eq!(vec![1002, 4, 3, 4, 99], computer.program);
        }
        {
            let mut computer = Intcode::new(&[1101, 100, -1, 4, 0]);
            computer.run(None);
            assert_eq!(vec![1101, 100, -1, 4, 99], computer.program);
        }
    }
}
