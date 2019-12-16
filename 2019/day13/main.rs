use std::cmp::Ordering;
use std::io::stdin;
use std::thread::sleep;
use std::time::Duration;
use utils::IntCode;

struct Game {
    score: i64,
    screen: Vec<Vec<u8>>,
    paddle: usize,
    ball: usize,
    debug: bool,
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
            debug: false, // for debug animation
        };
        game.update(out);
        return game;
    }
    fn update(&mut self, out: &Vec<i64>) {
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
    fn render(&self) {
        print!("\x1bc");
        println!("Score: {}", self.score);
        // frame += 1;
        for row in self.screen.iter() {
            let mut s = String::new();
            for v in row.iter() {
                s.push_str(match *v {
                    0 => "  ",
                    1 => "XX",
                    2 => "[]",
                    3 => "--",
                    4 => "()",
                    _ => "  ",
                });
            }
            println!("{}", s);
        }
    }
}

fn solve1(codes: Vec<i64>) -> i32 {
    let mut computer = IntCode::new(codes);
    let outputs = computer.run(vec![]);
    let mut answer = 0;
    for i in 0..outputs.len() / 3 {
        if outputs[i * 3 + 2] == 2 {
            answer += 1;
        }
    }
    return answer;
}

fn solve2(codes: Vec<i64>) -> i64 {
    let mut codes = codes;
    codes[0] = 2;
    let mut computer = IntCode::new(codes);
    let outputs = computer.run(vec![]);
    let mut game = Game::new(&outputs);
    if game.debug {
        game.render();
    }
    while !computer.is_halted() {
        if game.debug {
            sleep(Duration::from_millis(10));
            game.render();
        }
        let outputs = computer.run(vec![match game.paddle.cmp(&game.ball) {
            Ordering::Equal => 0,
            Ordering::Less => 1,
            Ordering::Greater => -1,
        }]);
        game.update(&outputs);
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
