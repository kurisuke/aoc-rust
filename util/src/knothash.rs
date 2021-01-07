use std::fmt;

pub struct KnotHash {
    v: [u8; 16],
}

pub fn knot(v: &mut Vec<u8>, lengths: &[usize], mut cur: usize, mut skip: usize) -> (usize, usize) {
    let vlen = v.len();
    for length in lengths {
        if *length > 1 {
            let mut i = 0;
            let mut j = length - 1;
            while i < j {
                v.swap((cur + i) % vlen, (cur + j) % vlen);
                i += 1;
                j -= 1;
            }
        }
        cur = (cur + length + skip) % vlen;
        skip += 1;
    }
    (cur, skip)
}

impl KnotHash {
    pub fn from(s: &str) -> KnotHash {
        let mut lengths: Vec<_> = s.chars().map(|c| c as u8 as usize).collect();
        let add = vec![17, 31, 73, 47, 23];
        lengths.extend(add);
        let mut v: Vec<_> = (0..=255).into_iter().collect();

        let mut cur = 0;
        let mut skip = 0;
        for _ in 0..64 {
            let res = knot(&mut v, &lengths, cur, skip);
            cur = res.0;
            skip = res.1;
        }
        let dense: Vec<u8> = (0..16)
            .map(|i| {
                let bas = 16 * i;
                (1..16).fold(v[bas], |acc, x| acc ^ v[bas + x])
            })
            .collect();
        KnotHash {
            v: [
                dense[0], dense[1], dense[2], dense[3], dense[4], dense[5], dense[6], dense[7],
                dense[8], dense[9], dense[10], dense[11], dense[12], dense[13], dense[14],
                dense[15],
            ],
        }
    }

    pub fn bytes(&self) -> impl Iterator<Item = &u8> {
        vec![
            &self.v[0],
            &self.v[1],
            &self.v[2],
            &self.v[3],
            &self.v[4],
            &self.v[5],
            &self.v[6],
            &self.v[7],
            &self.v[8],
            &self.v[9],
            &self.v[10],
            &self.v[11],
            &self.v[12],
            &self.v[13],
            &self.v[14],
            &self.v[15],
        ]
        .into_iter()
    }
}

impl fmt::Display for KnotHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out: String = self.v.iter().map(|x| format!("{:02x}", x)).collect();
        write!(f, "{}", out)
    }
}
