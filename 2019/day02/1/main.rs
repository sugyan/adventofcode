use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();
    let mut codes: Vec<usize> = buf.split(",").map(|s| s.parse().unwrap()).collect();
    codes[1] = 12;
    codes[2] = 02;
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
    println!("{}", codes[0]);
}
