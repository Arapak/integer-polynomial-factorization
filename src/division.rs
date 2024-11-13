use gcd::Gcd;
use crate::MODULO;
use crate::ntt::{mod_power, multiply};
use crate::polynomial::{Polynomial, multiply_by_constant, scale, subtract, add, reduce, sign, normalize};

fn divide_by_constant(p: &Polynomial, c: i64) -> Option<Polynomial> {
    p.into_iter().map(|x| if x % c != 0 { None } else { Some(x / c) }).collect()
}

fn reverse(p: &Polynomial) -> Polynomial {
   p.clone().into_iter().rev().collect()
}
pub fn reciprocal(p: &Polynomial) -> (Polynomial, i64) {
    if p.len() == 1 {
        return (vec![1], 0)
    }
    let k = p.len() as i64;
    let p0 = p[0..p.len()/2].to_vec();
    let p1 = scale(&p, -k / 2);
    let (a,num) = reciprocal(&p0);
    let c = scale(&multiply(&p0, &a),-k/2);
    let b = scale(&multiply(&a,&add(&multiply(&a,&p1), &c))[0..p.len()/2].to_vec(), k/2);
    (subtract(&multiply_by_constant(&a, p[0]), &b), num + 1)
}


pub fn divide(p: &Polynomial, q: &Polynomial) -> (Polynomial, Polynomial) {
    if p.len() == 0 || q.len() == 0 {
        return (vec![], vec![])
    }
    if p.len() < q.len() {
        return (vec![], p.clone())
    }

    let q_rev = reverse(&scale(q, (p.len() - q.len() + 1).next_power_of_two() as i64 - q.len() as i64));
    let (q_recip, _) = reciprocal(&q_rev);
    let q_recip = reduce(q_recip);
    let d = reverse(&multiply(&q_recip, &reverse(p))[0..(p.len() - q.len() + 1)].to_vec());
    let qd = multiply(q, &d);
    let multiplier = qd[qd.len() - 1] / p[p.len() - 1].unsigned_abs().gcd(qd[qd.len() - 1].unsigned_abs()) as i64 * sign( p[p.len() - 1]);
    let r = subtract(&multiply_by_constant(&p, multiplier), &qd);
    (normalize(&reduce(d)), normalize(&reduce(r)))
}

pub fn long_division_mod(p: &Polynomial, q: &Polynomial) -> (Polynomial, Polynomial) {
    if p.len() < q.len() {
        return (vec![], p.clone())
    }
    let multiplier = p[p.len() - 1] * mod_power(q[q.len() - 1], MODULO - 2) % MODULO;
    let (dividend, remainder) = long_division_mod(&reduce(
        subtract(
            &p,
            &scale(
                &multiply_by_constant(
                    &q,
                    multiplier),
                (p.len() - q.len()) as i64)
        )), q);
    (add(&dividend, &scale(&vec![multiplier], (p.len() - q.len()) as i64)), remainder)
}
pub fn remainder(p: &Polynomial, q: &Polynomial) -> Polynomial {
    long_division_mod(p, q).1
}