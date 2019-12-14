use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::stdin;
// use std::thread::sleep;
// use std::time::Duration;

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

struct Game {
    score: i64,
    screen: Vec<Vec<u8>>,
    paddle: usize,
    ball: usize,
}

impl Game {
    fn new(out: &Vec<i64>) -> Self {
        let mut xsize = 0;
        let mut ysize = 0;
        for i in 0..out.len() / 3 {
            if out[i * 3] >= 0 && out[i * 3 + 1] >= 0 {
                xsize = std::cmp::max(xsize, out[i * 3 + 0] as usize + 1);
                ysize = std::cmp::max(ysize, out[i * 3 + 1] as usize + 1);
            }
        }
        let mut game = Game {
            score: 0,
            screen: vec![vec![0; xsize]; ysize],
            paddle: 0,
            ball: 0,
        };
        game.update(out);
        return game;
    }
    fn update(&mut self, out: &Vec<i64>) {
        // println!("{:?}", out);
        for i in 0..out.len() / 3 {
            if out[i * 3] == -1 && out[i * 3 + 1] == 0 {
                self.score = out[i * 3 + 2];
                continue;
            }
            let x = out[i * 3 + 0] as usize;
            let y = out[i * 3 + 1] as usize;
            self.screen[y][x] = out[i * 3 + 2] as u8;
            match out[i * 3 + 2] {
                3 => self.paddle = x,
                4 => self.ball = x,
                _ => {}
            }
        }
    }
    // fn render(&self) {
    //     print!("\x1bc");
    //     println!("Score: {}", self.score);
    //     // frame += 1;
    //     for row in self.screen.iter() {
    //         let mut s = String::new();
    //         for v in row.iter() {
    //             s.push_str(match *v {
    //                 0 => "  ",
    //                 1 => "XX",
    //                 2 => "[]",
    //                 3 => "--",
    //                 4 => "()",
    //                 _ => "  ",
    //             });
    //         }
    //         println!("{}", s);
    //     }
    // }
}

fn solve1(codes: Vec<i64>) -> i32 {
    let mut computer = IntCode::new(codes);
    computer.run();
    let mut answer = 0;
    for i in 0..computer.outputs.len() / 3 {
        if computer.outputs[i * 3 + 2] == 2 {
            answer += 1;
        }
    }
    return answer;
}

fn solve2(codes: Vec<i64>) -> i64 {
    let mut codes = codes;
    codes[0] = 2;
    let mut computer = IntCode::new(codes);
    computer.run();
    let mut game = Game::new(&computer.outputs);
    computer.outputs.clear();
    // game.render();
    while computer.codes[&computer.i] != 99 {
        // sleep(Duration::from_millis(60));
        // game.render();
        computer.inputs.push(match game.paddle.cmp(&game.ball) {
            Ordering::Equal => 0,
            Ordering::Less => 1,
            Ordering::Greater => -1,
        });
        computer.run();
        game.update(&computer.outputs);
        computer.outputs.clear();
    }
    return game.score;
}

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();
    let codes: Vec<i64> = buf.split(",").map(|s| s.trim().parse().unwrap()).collect();

    let answer1 = solve1(codes.clone());
    println!("{}", answer1);
    let answer2 = solve2(codes.clone());
    println!("{}", answer2);
}
