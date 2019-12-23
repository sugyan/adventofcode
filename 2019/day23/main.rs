use std::collections::VecDeque;
use std::io::stdin;
use utils::IntCode;

fn solve1(codes: Vec<i64>) -> i64 {
    let mut computers: Vec<IntCode> = Vec::with_capacity(50);
    let mut inq: Vec<VecDeque<i64>> = vec![VecDeque::new(); 50];
    let mut outq: Vec<VecDeque<i64>> = vec![VecDeque::new(); 50];
    for i in 0..50 {
        let mut computer = IntCode::new(codes.clone());
        computer.run(vec![i as i64]);
        computers.push(computer);
    }
    loop {
        for (i, c) in (0..).zip(computers.iter_mut()) {
            let mut inputs = vec![];
            if inq[i].is_empty() {
                inputs.push(-1);
            } else {
                let x = inq[i].pop_front().unwrap();
                let y = inq[i].pop_front().unwrap();
                inputs.push(x);
                inputs.push(y);
            }
            let outputs = c.run(inputs);
            for v in outputs {
                outq[i].push_back(v);
            }
        }
        for i in 0..50 {
            if !outq[i].is_empty() {
                let address = outq[i].pop_front().unwrap();
                let x = outq[i].pop_front().unwrap();
                let y = outq[i].pop_front().unwrap();
                if address == 255 {
                    return y;
                }
                inq[address as usize].push_back(x);
                inq[address as usize].push_back(y);
            }
        }
    }
}

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();
    let codes: Vec<i64> = buf.split(",").map(|s| s.trim().parse().unwrap()).collect();
    println!("{}", solve1(codes));
}
