use std::env;
use std::io::stdin;

fn permutation(input: &mut Vec<i32>, output: &mut Vec<Vec<i32>>, v: &mut Vec<i32>) {
    if input.is_empty() {
        output.push(v.clone());
        return;
    }
    for i in 0..input.len() {
        let n = input[i];
        v.push(n);
        input.remove(i);
        permutation(input, output, v);
        input.insert(i, n);
        v.pop();
    }
}

struct Amplifier {
    codes: Vec<i32>,
    inputs: Vec<i32>,
    i: usize,
    output: i32,
}

impl Amplifier {
    fn intcode(&mut self, input: i32) -> bool {
        self.inputs.push(input);
        loop {
            let get_param = |pos: usize| -> i32 {
                return if (self.codes[self.i] / 10i32.pow(pos as u32 + 1)) % 10 == 0 {
                    self.codes[self.codes[self.i + pos] as usize]
                } else {
                    self.codes[self.i + pos]
                };
            };
            match self.codes[self.i] % 100 {
                1 => {
                    let pos = self.codes[self.i + 3] as usize;
                    self.codes[pos] = get_param(1) + get_param(2);
                    self.i += 4;
                }
                2 => {
                    let pos = self.codes[self.i + 3] as usize;
                    self.codes[pos] = get_param(1) * get_param(2);
                    self.i += 4;
                }
                3 => {
                    if let Some(first) = self.inputs.first() {
                        let pos = self.codes[self.i + 1] as usize;
                        self.codes[pos] = *first;
                        self.inputs.remove(0);
                        self.i += 2;
                    }
                }
                4 => {
                    self.output = get_param(1);
                    self.i += 2;
                    break;
                }
                5 => {
                    if get_param(1) != 0 {
                        self.i = get_param(2) as usize;
                    } else {
                        self.i += 3;
                    }
                }
                6 => {
                    if get_param(1) == 0 {
                        self.i = get_param(2) as usize;
                    } else {
                        self.i += 3;
                    }
                }
                7 => {
                    let pos = self.codes[self.i + 3] as usize;
                    self.codes[pos] = if get_param(1) < get_param(2) { 1 } else { 0 };
                    self.i += 4;
                }
                8 => {
                    let pos = self.codes[self.i + 3] as usize;
                    self.codes[pos] = if get_param(1) == get_param(2) { 1 } else { 0 };
                    self.i += 4;
                }
                99 => break,
                _ => {}
            }
        }
        return self.codes[self.i] == 99;
    }
}

fn solve1(codes: Vec<i32>) -> i32 {
    let (permutations, phases, v) = (&mut vec![], &mut vec![0, 1, 2, 3, 4], &mut vec![]);
    permutation(phases, permutations, v);

    let mut answer = std::i32::MIN;
    for permutation in permutations {
        let mut amplifiers: Vec<Amplifier> = permutation
            .iter()
            .map(|p| Amplifier {
                codes: codes.clone(),
                inputs: vec![*p],
                i: 0,
                output: 0,
            })
            .collect();
        let mut input = 0;
        for amp in amplifiers.iter_mut() {
            amp.intcode(input);
            input = amp.output;
        }
        answer = std::cmp::max(answer, input);
    }
    return answer;
}

fn solve2(codes: Vec<i32>) -> i32 {
    let (permutations, phases, v) = (&mut vec![], &mut vec![5, 6, 7, 8, 9], &mut vec![]);
    permutation(phases, permutations, v);

    let mut answer = std::i32::MIN;
    for permutation in permutations {
        let mut amplifiers: Vec<Amplifier> = permutation
            .iter()
            .map(|p| Amplifier {
                codes: codes.clone(),
                inputs: vec![*p],
                i: 0,
                output: 0,
            })
            .collect();
        let mut input = 0;
        let mut feedback = true;
        while feedback {
            for amp in amplifiers.iter_mut() {
                if amp.intcode(input) {
                    feedback = false;
                }
                input = amp.output;
            }
        }
        answer = std::cmp::max(answer, input);
    }
    return answer;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();
    let codes: Vec<i32> = buf.split(",").map(|s| s.trim().parse().unwrap()).collect();

    let answer = if args.len() < 2 || &args[1] != "2" {
        solve1(codes)
    } else {
        solve2(codes)
    };
    println!("{}", answer);
}
