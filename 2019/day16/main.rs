use std::io::stdin;

fn fft1(v: Vec<i32>, phases: usize) -> Vec<i32> {
    let base = vec![0, 1, 0, -1];
    let pattern = |i: usize, j: usize| -> i32 {
        return base[((j + 1) % (4 * (i + 1))) / (i + 1)];
    };
    let mut v = v;
    for _ in 0..phases {
        let mut ret = vec![0; v.len()];
        for i in 0..v.len() {
            let mut sum = 0;
            for j in i..v.len() {
                sum += v[j] * pattern(i, j);
            }
            ret[i] = sum.abs() % 10;
        }
        v = ret;
    }
    return v;
}

fn fft2(v: Vec<i32>, offset: usize) -> Vec<i32> {
    let mut signal = vec![0; v.len() * 10_000];
    for i in offset..v.len() * 10_000 {
        signal[i] = v[i % v.len()];
    }
    for _ in 0..100 {
        if offset >= signal.len() / 2 {
            let mut sum = 0;
            for i in (offset..v.len() * 10_000).rev() {
                sum += signal[i];
                signal[i] = sum % 10;
            }
        }
    }
    return Vec::from(&signal[offset..]);
}

fn solve1(input: String) -> String {
    let v: Vec<i32> = input
        .chars()
        .map(|c| (c as u8 - '0' as u8) as i32)
        .collect();
    let ret = fft1(v, 100);
    return (&ret[0..8])
        .iter()
        .map(|i| ('0' as u8 + *i as u8) as char)
        .collect();
}

fn solve2(input: String) -> String {
    let v: Vec<i32> = input
        .chars()
        .map(|c| (c as u8 - '0' as u8) as i32)
        .collect();
    let offset = (&v[0..7]).iter().fold(0, |acc, x| acc * 10 + x);
    let ret = fft2(v, offset as usize);
    return (&ret[0..8])
        .iter()
        .map(|i| ('0' as u8 + *i as u8) as char)
        .collect();
}

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();

    println!("{}", solve1(buf.clone()));
    println!("{}", solve2(buf.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            vec![4, 8, 2, 2, 6, 1, 5, 8],
            fft1(vec![1, 2, 3, 4, 5, 6, 7, 8], 1)
        );
        assert_eq!(
            vec![3, 4, 0, 4, 0, 4, 3, 8],
            fft1(vec![4, 8, 2, 2, 6, 1, 5, 8], 1)
        );
        assert_eq!(
            vec![0, 3, 4, 1, 5, 5, 1, 8],
            fft1(vec![3, 4, 0, 4, 0, 4, 3, 8], 1)
        );
        assert_eq!(
            vec![0, 1, 0, 2, 9, 4, 9, 8],
            fft1(vec![0, 3, 4, 1, 5, 5, 1, 8], 1)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![2, 4, 1, 7, 6, 1, 7, 6],
            &fft1(
                "80871224585914546619083218645595"
                    .chars()
                    .map(|c| (c as u8 - '0' as u8) as i32)
                    .collect(),
                100
            )[0..8]
        );
        assert_eq!(
            vec![7, 3, 7, 4, 5, 4, 1, 8],
            &fft1(
                "19617804207202209144916044189917"
                    .chars()
                    .map(|c| (c as u8 - '0' as u8) as i32)
                    .collect(),
                100
            )[0..8]
        );
        assert_eq!(
            vec![5, 2, 4, 3, 2, 1, 3, 3],
            &fft1(
                "69317163492948606335995924319873"
                    .chars()
                    .map(|c| (c as u8 - '0' as u8) as i32)
                    .collect(),
                100
            )[0..8]
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            vec![8, 4, 4, 6, 2, 0, 2, 6],
            &fft2(
                "03036732577212944063491565474664"
                    .chars()
                    .map(|c| (c as u8 - '0' as u8) as i32)
                    .collect(),
                303673
            )[0..8]
        );
        assert_eq!(
            vec![7, 8, 7, 2, 5, 2, 7, 0],
            &fft2(
                "02935109699940807407585447034323"
                    .chars()
                    .map(|c| (c as u8 - '0' as u8) as i32)
                    .collect(),
                293510
            )[0..8]
        );
        assert_eq!(
            vec![5, 3, 5, 5, 3, 7, 3, 1],
            &fft2(
                "03081770884921959731165446850517"
                    .chars()
                    .map(|c| (c as u8 - '0' as u8) as i32)
                    .collect(),
                308177
            )[0..8]
        );
    }
}
