use common::day::Day;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

pub struct Day19 {}

#[derive(Eq, Debug, Copy, Clone)]
struct Vec3D {
    x: i64,
    y: i64,
    z: i64,
}

// Seems that the fxhash is slower than this:
impl Hash for Vec3D {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i64(self.x ^ self.y ^ self.z);
    }
}

impl PartialEq for Vec3D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Vec3D {
    fn add(&self, other: &Vec3D) -> Vec3D {
        Vec3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    fn diff(&self, other: &Vec3D) -> Vec3D {
        Vec3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn rot(&self, i: usize) -> Vec3D {
        let (x, y, z) = (self.x, self.y, self.z);
        match i {
            0 => Vec3D { x, y, z },
            1 => Vec3D { x: -x, y, z: -z },
            2 => Vec3D { x, y: -y, z: -z },
            3 => Vec3D { x: -x, y: -y, z },
            4 => Vec3D { x: y, y: z, z: x },
            5 => Vec3D { x: -y, y: z, z: -x },
            6 => Vec3D { x: y, y: -z, z: -x },
            7 => Vec3D { x: -y, y: -z, z: x },
            8 => Vec3D { x: z, y: x, z: y },
            9 => Vec3D { x: -z, y: x, z: -y },
            10 => Vec3D { x: z, y: -x, z: -y },
            11 => Vec3D { x: -z, y: -x, z: y },
            12 => Vec3D { x: -x, y: z, z: y },
            13 => Vec3D { x, y: -z, z: y },
            14 => Vec3D { x, y: z, z: -y },
            15 => Vec3D {
                x: -x,
                y: -z,
                z: -y,
            },
            16 => Vec3D { x: -y, y: x, z },
            17 => Vec3D { x: y, y: -x, z },
            18 => Vec3D { x: y, y: x, z: -z },
            19 => Vec3D {
                x: -y,
                y: -x,
                z: -z,
            },
            20 => Vec3D { x: -z, y, z: x },
            21 => Vec3D { x: z, y: -y, z: x },
            22 => Vec3D { x: z, y, z: -x },
            23 => Vec3D {
                x: -z,
                y: -y,
                z: -x,
            },
            _ => unreachable!(),
        }
    }

    fn manhattan(&self, other: &Vec3D) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

#[derive(Clone)]
struct Scan {
    beacons: HashSet<Vec3D>,
}

impl Scan {
    fn rot(&self, i: usize) -> Scan {
        Scan {
            beacons: self.beacons.iter().map(|b| b.rot(i)).collect(),
        }
    }

    fn merge(&mut self, other: &Scan) {
        self.beacons = self.beacons.union(&other.beacons).cloned().collect()
    }

    fn add(&self, v: &Vec3D) -> Scan {
        Scan {
            beacons: self.beacons.iter().map(|b| b.add(v)).collect(),
        }
    }
}

fn parse_input(input: &str) -> HashMap<usize, Scan> {
    input
        .split("\n\n")
        .enumerate()
        .map(|(i, s)| {
            (
                i,
                Scan {
                    beacons: s
                        .lines()
                        .skip(1)
                        .map(|l| {
                            let v: Vec<_> = l.split(',').collect();
                            Vec3D {
                                x: v[0].parse().unwrap(),
                                y: v[1].parse().unwrap(),
                                z: v[2].parse().unwrap(),
                            }
                        })
                        .collect(),
                },
            )
        })
        .collect()
}

fn get_dist(scan1: &Scan, scan2: &Scan) -> (Vec3D, usize) {
    let mut dist_map = HashMap::new();
    for b1 in scan1.beacons.iter() {
        for b2 in scan2.beacons.iter() {
            let d_vec = b1.diff(b2);
            *dist_map.entry(d_vec).or_insert(0) += 1;
        }
    }

    dist_map
        .into_iter()
        .max_by(|(_, n1), (_, n2)| n1.cmp(n2))
        .unwrap()
}

fn try_merge(scan1: &Scan, scan2: &Scan) -> Option<(Vec3D, Scan)> {
    for i in 0..24 {
        let scan_rot = scan2.rot(i);
        let (d, num_overlap) = get_dist(scan1, &scan_rot);
        if num_overlap >= 12 {
            return Some((d, scan_rot));
        }
    }
    None
}

fn merge_scans(mut scans: HashMap<usize, Scan>) -> (HashMap<usize, Vec3D>, Scan) {
    let orig_len = scans.len();
    let mut merged = scans.remove(&0).unwrap();
    let mut scanners = HashMap::new();
    while !scans.is_empty() {
        for i in 1..orig_len {
            if let Some(scan) = scans.get(&i) {
                if let Some((d_vec, scan_rot)) = try_merge(&merged, scan) {
                    scanners.insert(i, d_vec);
                    let scan_trans = scan_rot.add(&d_vec);
                    merged.merge(&scan_trans);
                    scans.remove(&i);
                }
            }
        }
    }
    (scanners, merged)
}

impl Day for Day19 {
    fn star1(&self, input: &str) -> String {
        let scans = parse_input(input);
        let (_, merged) = merge_scans(scans);
        format!("{}", merged.beacons.len())
    }

    fn star2(&self, input: &str) -> String {
        let scans = parse_input(input);
        let (scanners, _) = merge_scans(scans);
        let mut max_dist = 0;
        for scanner1 in scanners.values() {
            for scanner2 in scanners.values() {
                max_dist = scanner1.manhattan(scanner2).max(max_dist);
            }
        }
        format!("{}", max_dist)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"--- scanner 0 ---
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
30,-46,-14"#;

        let d = Day19 {};
        assert_eq!(d.star1(input), "79");
        assert_eq!(d.star2(input), "3621");
    }
}
