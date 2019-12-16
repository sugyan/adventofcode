use std::collections::HashMap;
use std::io::stdin;
use utils::IntCode;

// fn render(hm: &HashMap<(i32, i32), char>) {
//     let (mut xmin, mut ymin) = (std::i32::MAX, std::i32::MAX);
//     let (mut xmax, mut ymax) = (std::i32::MIN, std::i32::MIN);
//     for pos in hm.keys() {
//         xmin = std::cmp::min(xmin, pos.0);
//         ymin = std::cmp::min(ymin, pos.1);
//         xmax = std::cmp::max(xmax, pos.0);
//         ymax = std::cmp::max(ymax, pos.1);
//     }
//     for i in 0..(ymax - ymin + 1) {
//         let y = ymax - i;
//         let row: String = (0..(xmax - xmin + 1))
//             .map(|j| {
//                 if let Some(c) = hm.get(&(xmin + j, y)) {
//                     *c
//                 } else {
//                     '!'
//                 }
//             })
//             .collect();
//         println!("{}", row);
//     }
// }

fn search(hm: &HashMap<(i32, i32), char>) -> i32 {
    let (mut xmin, mut ymin) = (std::i32::MAX, std::i32::MAX);
    let (mut xmax, mut ymax) = (std::i32::MIN, std::i32::MIN);
    for pos in hm.keys() {
        xmin = std::cmp::min(xmin, pos.0);
        ymin = std::cmp::min(ymin, pos.1);
        xmax = std::cmp::max(xmax, pos.0);
        ymax = std::cmp::max(ymax, pos.1);
    }
    let xsize = (xmax - xmin + 1) as usize;
    let ysize = (ymax - ymin + 1) as usize;
    let mut field = vec![vec![' '; xsize]; ysize];
    for i in 0..(ymax - ymin + 1) {
        for j in 0..(xmax - xmin + 1) {
            field[i as usize][j as usize] = if let Some(c) = hm.get(&(xmin + j, ymax - i)) {
                *c
            } else {
                '!'
            };
        }
    }
    loop {
        let mut finish = true;
        'check: for i in 0..ysize {
            for j in 0..xsize {
                if field[i][j] == '.' {
                    let mut count = 0;
                    if i > 0 && (field[i - 1][j] == '.' || field[i - 1][j] == '!') {
                        count += 1;
                    }
                    if j > 0 && (field[i][j - 1] == '.' || field[i][j - 1] == '!') {
                        count += 1;
                    }
                    if i < xsize - 1 && (field[i + 1][j] == '.' || field[i + 1][j] == '!') {
                        count += 1;
                    }
                    if j < xsize - 1 && (field[i][j + 1] == '.' || field[i][j + 1] == '!') {
                        count += 1;
                    }
                    if count < 2 {
                        field[i][j] = 'x';
                        finish = false;
                        break 'check;
                    }
                }
            }
        }
        if finish {
            break;
        }
    }
    let mut answer = 1;
    for i in 0..ysize {
        for j in 0..xsize {
            if field[i][j] == '.' {
                answer += 1;
            }
        }
    }
    return answer;
}

fn solve1(inputs: String) -> i32 {
    let mut hm: HashMap<(i32, i32), char> = HashMap::new();
    let mut droid = IntCode::from(inputs);
    let mut pos = (0, 0);
    hm.insert(pos, '!');
    let directions = vec![1, 4, 2, 3];
    let mut didx = 0;
    for _ in 0..3000 {
        let outputs = droid.run(vec![directions[didx]]);
        if outputs.len() != 1 {
            println!("invalid outputs: {:?}", outputs);
            break;
        }
        match outputs[0] {
            0 => {
                match didx {
                    0 => {
                        hm.insert((pos.0, pos.1 + 1), '#');
                    }
                    1 => {
                        hm.insert((pos.0 + 1, pos.1), '#');
                    }
                    2 => {
                        hm.insert((pos.0, pos.1 - 1), '#');
                    }
                    3 => {
                        hm.insert((pos.0 - 1, pos.1), '#');
                    }
                    _ => {}
                };
                didx = (didx + 3) % 4;
            }
            1 => {
                match didx {
                    0 => pos.1 += 1,
                    1 => pos.0 += 1,
                    2 => pos.1 -= 1,
                    3 => pos.0 -= 1,
                    _ => {}
                };
                hm.insert(pos, '.');
                didx = (didx + 1) % 4;
            }
            2 => {
                match didx {
                    0 => pos.1 += 1,
                    1 => pos.0 += 1,
                    2 => pos.1 -= 1,
                    3 => pos.0 -= 1,
                    _ => {}
                };
                hm.insert(pos, '!');
                break;
            }
            _ => println!("invalid output: {}", outputs[0]),
        }
    }
    return search(&hm);
}

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();

    println!("{}", solve1(buf));
}
