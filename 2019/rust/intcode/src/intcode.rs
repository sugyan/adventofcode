#[derive(Default, Debug)]
pub struct Intcode {
    pub program: Vec<u32>,
    i: usize,
}

impl Intcode {
    #[must_use]
    pub fn new(program: &[u32]) -> Self {
        Self {
            program: program.to_owned(),
            ..Intcode::default()
        }
    }
    pub fn run(&mut self) {
        loop {
            match self.program.get(self.i) {
                Some(1) => {
                    let pos1 = self.program[self.i as usize + 1] as usize;
                    let pos2 = self.program[self.i as usize + 2] as usize;
                    let pos3 = self.program[self.i as usize + 3] as usize;
                    let v1 = self.program[pos1];
                    let v2 = self.program[pos2];
                    self.program[pos3] = v1 + v2;
                }
                Some(2) => {
                    let pos1 = self.program[self.i as usize + 1] as usize;
                    let pos2 = self.program[self.i as usize + 2] as usize;
                    let pos3 = self.program[self.i as usize + 3] as usize;
                    let v1 = self.program[pos1];
                    let v2 = self.program[pos2];
                    self.program[pos3] = v1 * v2;
                }
                Some(99) => break,
                _ => unimplemented!(),
            }
            self.i += 4;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day02_example_1() {
        {
            let mut computer = Intcode::new(&[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
            computer.run();
            assert_eq!(
                vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
                computer.program
            );
        }
        {
            let mut computer = Intcode::new(&[1, 0, 0, 0, 99]);
            computer.run();
            assert_eq!(vec![2, 0, 0, 0, 99], computer.program);
        }
        {
            let mut computer = Intcode::new(&[2, 3, 0, 3, 99]);
            computer.run();
            assert_eq!(vec![2, 3, 0, 6, 99], computer.program);
        }
        {
            let mut computer = Intcode::new(&[2, 4, 4, 5, 99, 0]);
            computer.run();
            assert_eq!(vec![2, 4, 4, 5, 99, 9801], computer.program);
        }
        {
            let mut computer = Intcode::new(&[1, 1, 1, 4, 99, 5, 6, 0, 99]);
            computer.run();
            assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], computer.program);
        }
    }
}
