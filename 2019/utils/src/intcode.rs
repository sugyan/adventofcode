use std::collections::HashMap;

pub struct IntCode {
    i: usize,
    rel: i64,
    codes: HashMap<usize, i64>,
    inputs: Vec<i64>,
    outputs: Vec<i64>,
}

impl IntCode {
    pub fn new(v: Vec<i64>) -> IntCode {
        let mut codes: HashMap<usize, i64> = HashMap::new();
        for (i, code) in (0..).zip(v) {
            codes.insert(i, code);
        }
        return IntCode {
            i: 0,
            rel: 0,
            codes,
            inputs: Vec::new(),
            outputs: Vec::new(),
        };
    }
    pub fn run(&mut self, inputs: Vec<i64>) -> Vec<i64> {
        self.inputs.extend(inputs);
        loop {
            let code = self.codes[&self.i];
            match code % 100 {
                1 => self.op1(),
                2 => self.op2(),
                3 => {
                    if self.inputs.is_empty() {
                        break;
                    }
                    self.op3();
                }
                4 => self.op4(),
                5 => self.op5(),
                6 => self.op6(),
                7 => self.op7(),
                8 => self.op8(),
                9 => self.op9(),
                99 => break,
                _ => {
                    println!("unknown code: {}", code);
                }
            }
        }
        let outputs = self.outputs.clone();
        self.outputs.clear();
        return outputs;
    }
    pub fn is_halted(&self) -> bool {
        return self.codes[&self.i] == 99;
    }
    fn op1(&mut self) {
        let pos1 = self.get_pos(1);
        let pos2 = self.get_pos(2);
        let pos3 = self.get_pos(3);
        self.codes
            .insert(pos3, self.get_val(pos1) + self.get_val(pos2));
        self.i += 4;
    }
    fn op2(&mut self) {
        let pos1 = self.get_pos(1);
        let pos2 = self.get_pos(2);
        let pos3 = self.get_pos(3);
        self.codes
            .insert(pos3, self.get_val(pos1) * self.get_val(pos2));
        self.i += 4;
    }
    fn op3(&mut self) {
        let pos = self.get_pos(1);
        self.codes.insert(pos, self.inputs[0]);
        self.inputs.remove(0);
        self.i += 2;
    }
    fn op4(&mut self) {
        let pos = self.get_pos(1);
        self.outputs.push(self.get_val(pos));
        self.i += 2;
    }
    fn op5(&mut self) {
        let pos1 = self.get_pos(1);
        let pos2 = self.get_pos(2);
        self.i = if self.get_val(pos1) != 0 {
            self.get_val(pos2) as usize
        } else {
            self.i + 3
        };
    }
    fn op6(&mut self) {
        let pos1 = self.get_pos(1);
        let pos2 = self.get_pos(2);
        self.i = if self.get_val(pos1) == 0 {
            self.get_val(pos2) as usize
        } else {
            self.i + 3
        };
    }
    fn op7(&mut self) {
        let pos1 = self.get_pos(1);
        let pos2 = self.get_pos(2);
        let pos3 = self.get_pos(3);
        self.codes.insert(
            pos3,
            if self.get_val(pos1) < self.get_val(pos2) {
                1
            } else {
                0
            },
        );
        self.i += 4;
    }
    fn op8(&mut self) {
        let pos1 = self.get_pos(1);
        let pos2 = self.get_pos(2);
        let pos3 = self.get_pos(3);
        self.codes.insert(
            pos3,
            if self.get_val(pos1) == self.get_val(pos2) {
                1
            } else {
                0
            },
        );
        self.i += 4;
    }
    fn op9(&mut self) {
        let pos = self.get_pos(1);
        self.rel += self.get_val(pos);
        self.i += 2;
    }
    fn get_pos(&self, pos: usize) -> usize {
        return match (self.codes[&self.i] / 10i64.pow(pos as u32 + 1)) % 10 {
            0 => self.codes[&(self.i + pos)] as usize,
            1 => self.i + pos,
            2 => (self.codes[&(self.i + pos)] + self.rel) as usize,
            _ => {
                println!("invalid mode: {}", self.codes[&self.i]);
                0
            }
        };
    }
    fn get_val(&self, pos: usize) -> i64 {
        return match self.codes.get(&pos) {
            Some(v) => *v,
            None => 0,
        };
    }
}
