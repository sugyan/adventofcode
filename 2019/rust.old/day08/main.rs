use std::env;
use std::io::stdin;

fn solve1(layers: Vec<&[usize]>) {
    let mut answer = 0;
    let mut zeros = std::u32::MAX;
    for layer in layers {
        let mut count = vec![0; 3];
        for n in layer {
            count[*n] += 1;
        }
        if count[0] < zeros {
            zeros = count[0];
            answer = count[1] * count[2];
        }
    }
    println!("{}", answer);
}

fn solve2(layers: Vec<&[usize]>) {
    let mut answer = [[2; 25]; 6];
    for layer in layers {
        for i in 0..6 {
            for j in 0..25 {
                if answer[i][j] == 2 {
                    answer[i][j] = layer[i * 25 + j];
                }
            }
        }
    }
    for row in answer.iter() {
        println!(
            "{}",
            row.iter()
                .map(|e| if *e == 1 { '*' } else { ' ' })
                .collect::<String>()
        );
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();

    let layer_size = 25 * 6;
    let data: Vec<usize> = buf.chars().map(|c| c as usize - '0' as usize).collect();
    let layers: Vec<&[usize]> = data.chunks(layer_size).collect();

    if args.len() < 2 || &args[1] != "2" {
        solve1(layers);
    } else {
        solve2(layers);
    }
}
