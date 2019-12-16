use std::io::stdin;

fn fft(v: Vec<i32>, phases: usize) -> Vec<i32> {
    let base = vec![0, 1, 0, -1];
    let pattern = |i: usize, j: usize| -> i32 {
        return base[((j + 1) % (4 * (i + 1))) / (i + 1)];
    };
    let mut v = v;
    for _ in 0..phases {
        let mut ret = vec![0; v.len()];
        for i in 0..v.len() {
            let mut sum = 0;
            for j in 0..v.len() {
                sum += v[j] * pattern(i, j);
            }
            ret[i] = sum.abs() % 10;
        }
        v = ret;
    }
    return v;
}

fn solve1(input: String) -> String {
    let v: Vec<i32> = input
        .chars()
        .map(|c| (c as u8 - '0' as u8) as i32)
        .collect();
    let ret = fft(v, 100);
    return (&ret[0..8])
        .iter()
        .map(|i| ('0' as u8 + *i as u8) as char)
        .collect();
}

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();

    println!("{}", solve1(buf));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            vec![4, 8, 2, 2, 6, 1, 5, 8],
            fft(vec![1, 2, 3, 4, 5, 6, 7, 8], 1)
        );
        assert_eq!(
            vec![3, 4, 0, 4, 0, 4, 3, 8],
            fft(vec![4, 8, 2, 2, 6, 1, 5, 8], 1)
        );
        assert_eq!(
            vec![0, 3, 4, 1, 5, 5, 1, 8],
            fft(vec![3, 4, 0, 4, 0, 4, 3, 8], 1)
        );
        assert_eq!(
            vec![0, 1, 0, 2, 9, 4, 9, 8],
            fft(vec![0, 3, 4, 1, 5, 5, 1, 8], 1)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![2, 4, 1, 7, 6, 1, 7, 6],
            &fft(
                "80871224585914546619083218645595"
                    .chars()
                    .map(|c| (c as u8 - '0' as u8) as i32)
                    .collect(),
                100
            )[0..8]
        );
        assert_eq!(
            vec![7, 3, 7, 4, 5, 4, 1, 8],
            &fft(
                "19617804207202209144916044189917"
                    .chars()
                    .map(|c| (c as u8 - '0' as u8) as i32)
                    .collect(),
                100
            )[0..8]
        );
        assert_eq!(
            vec![5, 2, 4, 3, 2, 1, 3, 3],
            &fft(
                "69317163492948606335995924319873"
                    .chars()
                    .map(|c| (c as u8 - '0' as u8) as i32)
                    .collect(),
                100
            )[0..8]
        );
    }
}
