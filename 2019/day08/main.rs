use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();

    let layer_size = 25 * 6;

    let mut answer = 0;
    let mut zeros = std::u32::MAX;
    for layer in buf
        .chars()
        .map(|c| c as usize - '0' as usize)
        .collect::<Vec<usize>>()
        .chunks(layer_size)
    {
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
