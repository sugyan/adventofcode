use std::collections::HashMap;
use std::io::stdin;

struct IntCode {
    i: usize,
    rel: i64,
    codes: HashMap<usize, i64>,
    inputs: Vec<i64>,
    outputs: Vec<i64>,
}

impl IntCode {
    fn run(&mut self) {
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
        self.i = if self.get_val(pos1) == 1 {
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

    fn new(v: Vec<i64>) -> IntCode {
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
}

fn solve(codes: Vec<i64>, input: i64) -> HashMap<[i32; 2], bool> {
    let mut panels: HashMap<[i32; 2], bool> = HashMap::new();
    let mut pos: [i32; 2] = [0, 0];
    let mut dir: [i32; 2] = [0, 1];
    let mut computer = IntCode::new(codes);
    computer.inputs.push(input);
    while computer.codes[&computer.i] != 99 {
        computer.run();
        let out = computer.outputs.clone();
        computer.outputs.clear();
        panels.insert(pos, out[0] == 1);
        match out[1] {
            0 => dir = [-dir[1], dir[0]],
            1 => dir = [dir[1], -dir[0]],
            _ => {
                println!("invalid turn: {}", out[1]);
            }
        }
        pos[0] += dir[0];
        pos[1] += dir[1];
        let color = if let Some(white) = panels.get(&pos) {
            if *white {
                1
            } else {
                0
            }
        } else {
            0
        };
        computer.inputs.push(color);
    }
    return panels;
}

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();
    let codes: Vec<i64> = buf.split(",").map(|s| s.trim().parse().unwrap()).collect();

    let panels = solve(codes.clone(), 0);
    println!("{}", panels.len());
    let panels = solve(codes.clone(), 1);
    let (mut xmin, mut ymin) = (std::i32::MAX, std::i32::MAX);
    let (mut xmax, mut ymax) = (std::i32::MIN, std::i32::MIN);
    for (pos, white) in panels.iter() {
        if *white {
            xmin = std::cmp::min(xmin, pos[0]);
            ymin = std::cmp::min(ymin, pos[1]);
            xmax = std::cmp::max(xmax, pos[0]);
            ymax = std::cmp::max(ymax, pos[1]);
        }
    }
    let mut hull: Vec<Vec<bool>> =
        vec![vec![false; (xmax - xmin + 1) as usize]; (ymax - ymin + 1) as usize];
    for (pos, white) in panels.iter() {
        if *white {
            let x = (pos[0] - xmin) as usize;
            let y = (ymax - ymin) as usize - (pos[1] - ymin) as usize;
            hull[y][x] = true;
        }
    }
    for row in hull {
        println!(
            "{}",
            row.iter()
                .map(|b| if *b { '*' } else { ' ' })
                .collect::<String>()
        );
    }
}
