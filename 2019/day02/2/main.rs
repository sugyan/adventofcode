use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();
    let codes: Vec<usize> = buf.split(",").map(|s| s.parse().unwrap()).collect();
    for noun in 0..100 {
        for verb in 0..100 {
            let mut codes = codes.clone();
            codes[1] = noun;
            codes[2] = verb;
            let mut i = 0;
            while codes[i] != 99 {
                let lhs = codes[codes[i + 1]];
                let rhs = codes[codes[i + 2]];
                let pos = codes[i + 3];
                match codes[i] {
                    1 => codes[pos] = lhs + rhs,
                    2 => codes[pos] = lhs * rhs,
                    _ => break,
                }
                i += 4;
            }
            if codes[0] == 19690720 {
                println!("{}", noun * 100 + verb);
                return;
            }
        }
    }
}
