use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;
use crate::division::remainder;
use crate::MODULO;
use crate::ntt::mod_power;

pub type Polynomial = Vec<i64>;
pub fn reduce(mut p: Polynomial) -> Polynomial {
    if p.len() == 0 {
        return vec![]
    }
    while p.len() > 0 && p[p.len() - 1] == 0 {
        p.pop();
    }
    p
}

pub fn sign(x: i64) -> i64 {
    (x > 0) as i64 - (x < 0) as i64
}

pub fn normalize(p: &Polynomial) -> Polynomial {
    if p.len() == 0 {
        return vec![]
    }
    // let gcd = p.into_iter().fold(0, |g, x| g.gcd(x.unsigned_abs())) as i64 * sign(p[p.len() - 1]);
    // p.into_iter().map(|x| x / gcd).collect()
    let multiplier = mod_power(p[p.len() - 1], MODULO - 2);
    p.into_iter().map(|x| x * multiplier % MODULO).collect()
}
pub fn multiply_by_constant(p: &Polynomial, c: i64) -> Polynomial {
p.into_iter().map(|x| x * c % MODULO).collect()
}

pub fn subtract(p: &Polynomial, q: &Polynomial) -> Polynomial {
    p.into_iter().zip_longest(q).map(|x|match x {
        Both(a,b) => (a - b + MODULO) % MODULO,
        Left(a) => *a,
        Right(b) => MODULO -*b,
    }).collect()
}
pub fn add(p: &Polynomial, q: &Polynomial) -> Polynomial {
    p.into_iter().zip_longest(q).map(|x|match x {
        Both(a,b) => (a + b) % MODULO,
        Left(a) => *a,
        Right(b) => *b,
    }).collect()
}

pub fn scale(p: &Polynomial, c: i64) -> Polynomial {
    if c >= 0 {
        [[0].repeat(c as usize), p.clone()].concat()
    } else {
        p[(-c) as usize..p.len()].to_vec()
    }
}

pub fn gcd(p: &Polynomial, q: &Polynomial) -> Polynomial {
    assert!(p.len() >= q.len());
    if q.len() == 0 {
        return p.clone()
    }
    gcd(q, &remainder(p, q))
}

pub fn derivative(p: &Polynomial) -> Polynomial {
    p[1..p.len()].into_iter().enumerate().map(|(i,x)| (*x)*((i+1) as i64)).collect()
}