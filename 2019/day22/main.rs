use std::io::stdin;

#[derive(Clone, Debug)]
enum Shuffle {
    DealIntoNewStack(),
    CutCards(i64),
    DealWithIncrement(i64),
}

fn apply(deck: &mut Vec<i64>, shuffles: Vec<Shuffle>) {
    let (off, inc) = calc_matrix(shuffles, deck.len() as i64);
    for i in 0..deck.len() {
        let mut n = (off + inc * i as i64) % deck.len() as i64;
        if n < 0 {
            n += deck.len() as i64;
        }
        deck[i] = n;
    }
}

fn inv(n: i64, size: i64) -> i64 {
    if size <= 10 {
        for i in 1.. {
            if (n * i) % size == 1 {
                return i;
            }
        }
    }
    let mut n = n;
    let mut m = size - 2;
    let mut ret = 1;
    while m > 0 {
        if m & 1 == 1 {
            ret = (ret * n) % size;
        }
        n = (n * n) % size;
        m >>= 1;
    }
    return ret;
}

fn calc_matrix(shuffles: Vec<Shuffle>, size: i64) -> (i64, i64) {
    let (mut off, mut inc) = (0, 1);
    for shuffle in shuffles.iter() {
        match shuffle {
            Shuffle::DealIntoNewStack() => {
                inc *= -1;
                off += inc;
            }
            Shuffle::CutCards(n) => off += inc * n,
            Shuffle::DealWithIncrement(n) => inc *= inv(*n, size),
        }
        off = off % size;
        inc = inc % size;
    }
    return (off, inc);
}

struct Solution {
    shuffles: Vec<Shuffle>,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Solution {
        let shuffles: Vec<Shuffle> = inputs
            .iter()
            .map(|input| {
                if input.as_str() == "deal into new stack" {
                    Shuffle::DealIntoNewStack()
                } else {
                    let n: i64 = input.split(' ').last().unwrap().parse().unwrap();
                    if input.starts_with("deal with increment") {
                        Shuffle::DealWithIncrement(n)
                    } else {
                        Shuffle::CutCards(n)
                    }
                }
            })
            .collect();
        return Solution { shuffles };
    }
    fn solve1(&self) -> i32 {
        let mut deck: Vec<i64> = (0..10007).collect();
        apply(&mut deck, self.shuffles.clone());
        return deck.iter().position(|card| *card == 2019).unwrap() as i32;
    }
}

fn main() {
    let mut inputs: Vec<String> = Vec::new();
    loop {
        let mut buf = String::new();
        stdin().read_line(&mut buf).ok();
        if buf.is_empty() {
            break;
        }
        inputs.push(buf.trim().to_string());
    }
    let solution = Solution::new(inputs);
    println!("{}", solution.solve1());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut deck: Vec<i64> = (0..10).collect();
        apply(
            &mut deck,
            vec![
                Shuffle::DealWithIncrement(7),
                Shuffle::DealIntoNewStack(),
                Shuffle::DealIntoNewStack(),
            ],
        );
        assert_eq!(vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7], deck);
    }

    #[test]
    fn example_2() {
        let mut deck: Vec<i64> = (0..10).collect();
        apply(
            &mut deck,
            vec![
                Shuffle::CutCards(6),
                Shuffle::DealWithIncrement(7),
                Shuffle::DealIntoNewStack(),
            ],
        );
        assert_eq!(vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6], deck);
    }

    #[test]
    fn example_3() {
        let mut deck: Vec<i64> = (0..10).collect();
        apply(
            &mut deck,
            vec![
                Shuffle::DealWithIncrement(7),
                Shuffle::DealWithIncrement(9),
                Shuffle::CutCards(-2),
            ],
        );
        assert_eq!(vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9], deck);
    }

    #[test]
    fn example_4() {
        let mut deck: Vec<i64> = (0..10).collect();
        apply(
            &mut deck,
            vec![
                Shuffle::DealIntoNewStack(),
                Shuffle::CutCards(-2),
                Shuffle::DealWithIncrement(7),
                Shuffle::CutCards(8),
                Shuffle::CutCards(-4),
                Shuffle::DealWithIncrement(7),
                Shuffle::CutCards(3),
                Shuffle::DealWithIncrement(9),
                Shuffle::DealWithIncrement(3),
                Shuffle::CutCards(-1),
            ],
        );
        assert_eq!(vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6], deck);
    }
}
