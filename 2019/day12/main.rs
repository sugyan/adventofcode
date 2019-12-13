use std::io::stdin;

const AXIS: usize = 3;
const MOONS_NUM: usize = 4;

fn solve1(pos: [[i32; AXIS]; MOONS_NUM]) -> i32 {
    let mut pos = pos;
    let mut vel = [[0; AXIS]; MOONS_NUM];
    for _ in 0..1000 {
        simulate(&mut pos, &mut vel);
    }
    let pot = pos.iter().map(|e| e.iter().map(|v| v.abs()).sum::<i32>());
    let kin = vel.iter().map(|e| e.iter().map(|v| v.abs()).sum::<i32>());
    return pot.zip(kin).map(|e| e.0 * e.1).sum::<i32>();
}

fn simulate(pos: &mut [[i32; AXIS]; MOONS_NUM], vel: &mut [[i32; AXIS]; MOONS_NUM]) {
    for k in 0..AXIS {
        let mut d = [0; MOONS_NUM];
        for i in 0..MOONS_NUM {
            for j in 0..MOONS_NUM {
                if i == j {
                    continue;
                }
                d[i] += match pos[i][k].cmp(&pos[j][k]) {
                    std::cmp::Ordering::Greater => -1,
                    std::cmp::Ordering::Equal => 0,
                    std::cmp::Ordering::Less => 1,
                };
            }
        }
        for i in 0..MOONS_NUM {
            vel[i][k] += d[i];
            pos[i][k] += vel[i][k];
        }
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    return if a == 0 { b } else { gcd(b % a, a) };
}

fn lcm(a: i64, b: i64) -> i64 {
    let g = gcd(a, b);
    return (a / g) * b;
}

fn solve2(pos: [[i32; AXIS]; MOONS_NUM]) -> i64 {
    let init_pos = pos.clone();
    let mut pos = pos;
    let mut vel = [[0; AXIS]; MOONS_NUM];
    let mut periods = [0; AXIS];
    for i in 1.. {
        simulate(&mut pos, &mut vel);
        for j in 0..AXIS {
            if (0..MOONS_NUM).all(|k| pos[k][j] == init_pos[k][j] && vel[k][j] == 0) {
                periods[j] = i;
            }
        }
        if let Some(min) = periods.iter().min() {
            if *min > 0 {
                break;
            }
        }
    }
    return periods.iter().fold(1, |acc, x| lcm(acc, *x));
}

fn main() {
    let mut pos: [[i32; AXIS]; MOONS_NUM] = [[0; AXIS]; MOONS_NUM];
    for i in 0..MOONS_NUM {
        let mut buf = String::new();
        stdin().read_line(&mut buf).ok();
        let v: Vec<i32> = buf
            .trim()
            .trim_start_matches('<')
            .trim_end_matches('>')
            .split(", ")
            .map(|s| s.split('=').nth(1).unwrap())
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i32>>();
        pos[i].copy_from_slice(v.as_slice());
    }
    let answer1 = solve1(pos);
    println!("{}", answer1);
    let answer2 = solve2(pos);
    println!("{}", answer2);
}
