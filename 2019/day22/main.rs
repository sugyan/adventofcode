use std::io::stdin;

#[derive(Debug)]
enum Shuffle {
    DealIntoNewStack(),
    CutCards(i32),
    DealWithIncrement(i32),
}

fn apply(deck: &mut Vec<i32>, shuffles: Vec<Shuffle>) {
    for shuffle in shuffles.iter() {
        match shuffle {
            Shuffle::DealIntoNewStack() => {
                deck.reverse();
            }
            Shuffle::CutCards(n) => {
                let v = deck.clone();
                let n = ((v.len() as i32 + *n) as usize) % v.len();
                unsafe {
                    std::ptr::copy(v.as_ptr(), deck.as_mut_ptr().add(v.len() - n), n);
                    std::ptr::copy(v.as_ptr().add(n), deck.as_mut_ptr(), v.len() - n);
                }
            }
            Shuffle::DealWithIncrement(n) => {
                let size = deck.len();
                let v = deck.clone();
                for (i, e) in (0..).zip(v) {
                    deck[(i * *n as usize) % size] = e;
                }
            }
        }
    }
}

struct Solution {}

impl Solution {
    fn solve1(&self, inputs: Vec<String>) -> i32 {
        let shuffles: Vec<Shuffle> = inputs
            .iter()
            .map(|input| {
                if input.as_str() == "deal into new stack" {
                    Shuffle::DealIntoNewStack()
                } else {
                    let n: i32 = input.split(' ').last().unwrap().parse().unwrap();
                    if input.starts_with("deal with increment") {
                        Shuffle::DealWithIncrement(n)
                    } else {
                        Shuffle::CutCards(n)
                    }
                }
            })
            .collect();
        let mut deck: Vec<i32> = (0..10007).collect();
        apply(&mut deck, shuffles);
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
    let solution = Solution {};
    println!("{}", solution.solve1(inputs));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut deck: Vec<i32> = (0..10).collect();
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
        let mut deck: Vec<i32> = (0..10).collect();
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
        let mut deck: Vec<i32> = (0..10).collect();
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
        let mut deck: Vec<i32> = (0..10).collect();
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
