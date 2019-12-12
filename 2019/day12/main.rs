use std::io::stdin;

const AXIS: usize = 3;
const MOONS_NUM: usize = 4;

fn main() {
    let mut pos: [[i32; AXIS]; MOONS_NUM] = [[0; AXIS]; MOONS_NUM];
    let mut vel: [[i32; AXIS]; MOONS_NUM] = [[0; AXIS]; MOONS_NUM];
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
    for _ in 0..1000 {
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
    let pot = pos.iter().map(|e| e.iter().map(|v| v.abs()).sum::<i32>());
    let kin = vel.iter().map(|e| e.iter().map(|v| v.abs()).sum::<i32>());
    let answer = pot.zip(kin).map(|e| e.0 * e.1).sum::<i32>();
    println!("{}", answer);
}
