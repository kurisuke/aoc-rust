use common::day::Day;
use num_bigint::BigInt;

pub struct Day22 {}

enum DealCmd {
    Inv,
    Inc(BigInt),
    Cut(BigInt),
}

struct Stack {
    off: BigInt,
    inc: BigInt,
    modulo: BigInt,
}

fn modulo_pow(base: &BigInt, exp: &BigInt, m: &BigInt) -> BigInt {
    let mut exp = exp.clone();
    let mut base = base.clone();

    let mut r = BigInt::from(1);
    while exp > BigInt::from(0) {
        if base == BigInt::from(0) {
            return BigInt::from(0);
        }
        if modulo(&exp, &BigInt::from(2)) == BigInt::from(1) {
            r = modulo(&(r * base.clone()), m);
        }
        exp /= 2;
        base = modulo(&(base.clone() * base), m);
    }
    r
}

fn modulo_inv(n: &BigInt, m: &BigInt) -> BigInt {
    modulo_pow(n, &(m - 2), m)
}

fn modulo(n: &BigInt, m: &BigInt) -> BigInt {
    ((n % m) + m) % m
}

fn transform(mut stack: Stack, cmds: &[DealCmd]) -> Stack {
    for cmd in cmds {
        match cmd {
            DealCmd::Inv => {
                stack.inc = modulo(&(-stack.inc), &stack.modulo);
                stack.off = modulo(&(stack.off + stack.inc.clone()), &stack.modulo);
            }
            DealCmd::Cut(k) => {
                stack.off = modulo(&(stack.off + stack.inc.clone() * k), &stack.modulo);
            }
            DealCmd::Inc(k) => {
                stack.inc = modulo(&(stack.inc * modulo_inv(k, &stack.modulo)), &stack.modulo);
            }
        }
    }
    stack
}

fn parse_input(input: &str) -> Vec<DealCmd> {
    input
        .lines()
        .map(|l| {
            let s: Vec<_> = l.split(' ').collect();
            if s[0] == "deal" && s[1] == "into" {
                DealCmd::Inv
            } else if s[0] == "deal" && s[1] == "with" {
                DealCmd::Inc(s[3].parse().unwrap())
            } else if s[0] == "cut" {
                DealCmd::Cut(s[1].parse().unwrap())
            } else {
                unreachable!()
            }
        })
        .collect()
}

impl Day for Day22 {
    fn star1(&self, input: &str) -> String {
        let cmds = parse_input(input);
        let stack_modulo = 10007;
        let stack = Stack {
            off: BigInt::from(0),
            inc: BigInt::from(1),
            modulo: BigInt::from(stack_modulo),
        };
        let stack = transform(stack, &cmds);

        for i in 0..stack_modulo {
            let v = modulo(&(stack.off.clone() + stack.inc.clone() * i), &stack.modulo);
            if v == BigInt::from(2019) {
                return format!("{}", i);
            }
        }
        String::from("not found")
    }

    fn star2(&self, input: &str) -> String {
        let cmds = parse_input(input);
        let stack = Stack {
            off: BigInt::from(0),
            inc: BigInt::from(1),
            modulo: BigInt::from(119315717514047i64),
        };
        let stack = transform(stack, &cmds);

        let off_diff = stack.off;
        let inc_mul = stack.inc;
        let iter = BigInt::from(101741582076661i64);

        let inc_final = modulo_pow(&inc_mul, &iter, &stack.modulo);
        let off_final = modulo(
            &(off_diff
                * (BigInt::from(1) - modulo_pow(&inc_mul, &iter, &stack.modulo))
                * modulo_inv(
                    &modulo(&(BigInt::from(1) - inc_mul), &stack.modulo),
                    &stack.modulo,
                )),
            &stack.modulo,
        );
        let pos_2020 = modulo(&(off_final + BigInt::from(2020) * inc_final), &stack.modulo);
        format!("{}", pos_2020)
    }
}
