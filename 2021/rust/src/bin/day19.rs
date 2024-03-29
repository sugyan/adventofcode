use aoc2021::Solve;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{BufRead, BufReader, Read};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Position {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
    fn distance(&self, p: &Position) -> i32 {
        (self.x - p.x).abs() + (self.y - p.y).abs() + (self.z - p.z).abs()
    }
}

#[derive(Clone, Copy)]
enum Axis {
    X,
    Y,
    Z,
}

#[derive(Clone, Copy)]
enum Sign {
    Positive,
    Negative,
}

#[derive(Clone, Copy)]
struct Direction {
    axis: Axis,
    sign: Sign,
}

#[derive(Clone, Copy)]
struct Orientation {
    facing: Direction,
    up: Direction,
}

impl Orientation {
    fn all() -> Vec<Orientation> {
        [Axis::X, Axis::Y, Axis::Z]
            .iter()
            .permutations(2)
            .flat_map(|a| {
                itertools::iproduct!(
                    [Sign::Positive, Sign::Negative],
                    [Sign::Positive, Sign::Negative]
                )
                .map(move |s| Orientation {
                    facing: Direction {
                        axis: *a[0],
                        sign: s.0,
                    },
                    up: Direction {
                        axis: *a[1],
                        sign: s.1,
                    },
                })
            })
            .collect()
    }
    fn calculate_position(&self, t: &Position, p: &Position) -> Position {
        use Axis::*;
        use Sign::*;
        match (
            (&self.facing.axis, &self.facing.sign),
            (&self.up.axis, &self.up.sign),
        ) {
            ((X, Positive), (Y, Positive)) => Position::new(t.x - p.x, t.y - p.y, t.z - p.z),
            ((X, Positive), (Y, Negative)) => Position::new(t.x - p.x, t.y + p.y, t.z + p.z),
            ((X, Positive), (Z, Positive)) => Position::new(t.x - p.x, t.y - p.z, t.z + p.y),
            ((X, Positive), (Z, Negative)) => Position::new(t.x - p.x, t.y + p.z, t.z - p.y),
            ((X, Negative), (Y, Positive)) => Position::new(t.x + p.x, t.y - p.y, t.z + p.z),
            ((X, Negative), (Y, Negative)) => Position::new(t.x + p.x, t.y + p.y, t.z - p.z),
            ((X, Negative), (Z, Positive)) => Position::new(t.x + p.x, t.y - p.z, t.z - p.y),
            ((X, Negative), (Z, Negative)) => Position::new(t.x + p.x, t.y + p.z, t.z + p.y),
            ((Y, Positive), (X, Positive)) => Position::new(t.x - p.y, t.y - p.x, t.z + p.z),
            ((Y, Positive), (X, Negative)) => Position::new(t.x - p.y, t.y + p.x, t.z - p.z),
            ((Y, Positive), (Z, Positive)) => Position::new(t.x - p.y, t.y - p.z, t.z - p.x),
            ((Y, Positive), (Z, Negative)) => Position::new(t.x - p.y, t.y + p.z, t.z + p.x),
            ((Y, Negative), (X, Positive)) => Position::new(t.x + p.y, t.y - p.x, t.z - p.z),
            ((Y, Negative), (X, Negative)) => Position::new(t.x + p.y, t.y + p.x, t.z + p.z),
            ((Y, Negative), (Z, Positive)) => Position::new(t.x + p.y, t.y - p.z, t.z + p.x),
            ((Y, Negative), (Z, Negative)) => Position::new(t.x + p.y, t.y + p.z, t.z - p.x),
            ((Z, Positive), (X, Positive)) => Position::new(t.x - p.z, t.y - p.x, t.z - p.y),
            ((Z, Positive), (X, Negative)) => Position::new(t.x - p.z, t.y + p.x, t.z + p.y),
            ((Z, Positive), (Y, Positive)) => Position::new(t.x - p.z, t.y - p.y, t.z + p.x),
            ((Z, Positive), (Y, Negative)) => Position::new(t.x - p.z, t.y + p.y, t.z - p.x),
            ((Z, Negative), (X, Positive)) => Position::new(t.x + p.z, t.y - p.x, t.z + p.y),
            ((Z, Negative), (X, Negative)) => Position::new(t.x + p.z, t.y + p.x, t.z - p.y),
            ((Z, Negative), (Y, Positive)) => Position::new(t.x + p.z, t.y - p.y, t.z - p.x),
            ((Z, Negative), (Y, Negative)) => Position::new(t.x + p.z, t.y + p.y, t.z + p.x),
            _ => unreachable!(),
        }
    }
    fn translate(&self, p: &Position, o: &Position) -> Position {
        use Axis::*;
        use Sign::*;
        match (
            (&self.facing.axis, &self.facing.sign),
            (&self.up.axis, &self.up.sign),
        ) {
            ((X, Positive), (Y, Positive)) => Position::new(o.x + p.x, o.y + p.y, o.z + p.z),
            ((X, Positive), (Y, Negative)) => Position::new(o.x + p.x, o.y - p.y, o.z - p.z),
            ((X, Positive), (Z, Positive)) => Position::new(o.x + p.x, o.y + p.z, o.z - p.y),
            ((X, Positive), (Z, Negative)) => Position::new(o.x + p.x, o.y - p.z, o.z + p.y),
            ((X, Negative), (Y, Positive)) => Position::new(o.x - p.x, o.y + p.y, o.z - p.z),
            ((X, Negative), (Y, Negative)) => Position::new(o.x - p.x, o.y - p.y, o.z + p.z),
            ((X, Negative), (Z, Positive)) => Position::new(o.x - p.x, o.y + p.z, o.z + p.y),
            ((X, Negative), (Z, Negative)) => Position::new(o.x - p.x, o.y - p.z, o.z - p.y),
            ((Y, Positive), (X, Positive)) => Position::new(o.x + p.y, o.y + p.x, o.z - p.z),
            ((Y, Positive), (X, Negative)) => Position::new(o.x + p.y, o.y - p.x, o.z + p.z),
            ((Y, Positive), (Z, Positive)) => Position::new(o.x + p.y, o.y + p.z, o.z + p.x),
            ((Y, Positive), (Z, Negative)) => Position::new(o.x + p.y, o.y - p.z, o.z - p.x),
            ((Y, Negative), (X, Positive)) => Position::new(o.x - p.y, o.y + p.x, o.z + p.z),
            ((Y, Negative), (X, Negative)) => Position::new(o.x - p.y, o.y - p.x, o.z - p.z),
            ((Y, Negative), (Z, Positive)) => Position::new(o.x - p.y, o.y + p.z, o.z - p.x),
            ((Y, Negative), (Z, Negative)) => Position::new(o.x - p.y, o.y - p.z, o.z + p.x),
            ((Z, Positive), (X, Positive)) => Position::new(o.x + p.z, o.y + p.x, o.z + p.y),
            ((Z, Positive), (X, Negative)) => Position::new(o.x + p.z, o.y - p.x, o.z - p.y),
            ((Z, Positive), (Y, Positive)) => Position::new(o.x + p.z, o.y + p.y, o.z - p.x),
            ((Z, Positive), (Y, Negative)) => Position::new(o.x + p.z, o.y - p.y, o.z + p.x),
            ((Z, Negative), (X, Positive)) => Position::new(o.x - p.z, o.y + p.x, o.z - p.y),
            ((Z, Negative), (X, Negative)) => Position::new(o.x - p.z, o.y - p.x, o.z + p.y),
            ((Z, Negative), (Y, Positive)) => Position::new(o.x - p.z, o.y + p.y, o.z + p.x),
            ((Z, Negative), (Y, Negative)) => Position::new(o.x - p.z, o.y - p.y, o.z - p.x),
            _ => unreachable!(),
        }
    }
}

struct Solution {
    scanners: Vec<Position>,
    beacons: Vec<HashSet<Position>>,
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = i32;

    fn new(r: impl Read) -> Self {
        let reports = BufReader::new(r)
            .lines()
            .filter_map(Result::ok)
            .collect::<Vec<_>>()
            .split(String::is_empty)
            .map(|lines| {
                lines
                    .iter()
                    .skip(1)
                    .map(|line| {
                        let (x, y, z) = line
                            .split(',')
                            .map(|s| s.parse().unwrap())
                            .collect_tuple()
                            .unwrap();
                        Position { x, y, z }
                    })
                    .collect()
            })
            .collect::<Vec<_>>();
        let find_overlap =
            |r0: &HashSet<Position>, r1: &HashSet<Position>| -> Option<(Orientation, Position)> {
                for orientation in Orientation::all() {
                    for p0 in r0 {
                        for p1 in r1 {
                            let o = orientation.calculate_position(p0, p1);
                            if r1
                                .iter()
                                .filter(|&p| r0.contains(&orientation.translate(p, &o)))
                                .count()
                                >= 12
                            {
                                return Some((orientation, o));
                            }
                        }
                    }
                }
                None
            };
        let dists = reports
            .iter()
            .map(|r: &HashSet<Position>| {
                let mut hm = HashMap::new();
                for (p0, p1) in r.iter().tuple_combinations() {
                    *hm.entry(p0.distance(p1)).or_insert(0) += 1;
                }
                hm
            })
            .collect::<Vec<_>>();
        let mut graph = vec![Vec::new(); reports.len()];
        for (i, j) in (0..reports.len()).tuple_combinations() {
            if dists[i]
                .iter()
                .filter_map(|(k, vi)| dists[j].get(k).map(|vj| vi.min(vj)))
                .sum::<u32>()
                < 78
            {
                continue;
            }
            if find_overlap(&reports[i], &reports[j]).is_some() {
                graph[i].push(j);
                graph[j].push(i);
            }
        }
        let mut scanners = vec![None; reports.len()];
        let mut beacons = vec![None; reports.len()];
        scanners[0] = Some(Position::new(0, 0, 0));
        beacons[0] = Some(reports[0].clone());
        let mut vd = VecDeque::new();
        vd.push_back(0);
        while let Some(i) = vd.pop_front() {
            for &j in &graph[i] {
                if scanners[j].is_none() {
                    let (orientation, o) =
                        find_overlap(beacons[i].as_ref().unwrap(), &reports[j]).unwrap();
                    scanners[j] = Some(o);
                    beacons[j] = Some(
                        reports[j]
                            .iter()
                            .map(|p| orientation.translate(p, &o))
                            .collect(),
                    );
                    vd.push_back(j);
                }
            }
        }
        Self {
            scanners: scanners.iter().filter_map(|&o| o).collect(),
            beacons: beacons.iter().filter_map(|o| o.as_ref().cloned()).collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.beacons
            .iter()
            .flat_map(|hs| hs.iter())
            .collect::<HashSet<_>>()
            .len()
    }
    fn part2(&self) -> Self::Answer2 {
        self.scanners
            .iter()
            .tuple_combinations()
            .map(|(p0, p1)| p0.distance(p1))
            .max()
            .unwrap()
    }
}

fn main() {
    let solution = Solution::new(std::io::stdin().lock());
    println!("Part 1: {}", solution.part1());
    println!("Part 2: {}", solution.part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static [u8] {
        r"
--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(79, Solution::new(example_input()).part1());
    }

    #[test]
    fn part2() {
        assert_eq!(3621, Solution::new(example_input()).part2());
    }
}
