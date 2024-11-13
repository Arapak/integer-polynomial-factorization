use crate::division::{divide, remainder};
use crate::ntt::multiply;
use crate::polynomial::{gcd, Polynomial, reduce, subtract};

pub fn mod_power_polynomial(mut p: Polynomial, mut exp: i64, modulo: &Polynomial) -> Polynomial {
    let mut res: Polynomial = vec![1];
    while exp > 0 {
        if exp & 1 == 1 {
            res = remainder(&multiply(&res, &p), modulo);
        }
        p = remainder(&multiply(&p, &p), modulo);
        exp /= 2;
    }
    res
}

pub fn dff(mut p: Polynomial) -> Vec<Polynomial> {
    let mut g = vec![0,1];
    let mut res = vec![vec![1];p.len()];
    let mut i = 1;
    while p.len() > 2*i  {
        println!("i: {:?}", i);
        g = mod_power_polynomial(g.clone(), 5, &p);
        println!("g: {:?}", g);
        res[i] = gcd(&p, &reduce(subtract(&g, &vec![0,1])));
        println!("res[i]: {:?}", res[i]);
        if res[i] != vec![1] {
            p = divide(&p, &res[i]).0;
        }
        println!("p: {:?}", p);
        i += 1;
    }
    if p.len() != 0 {
        res[p.len() - 1] = p.clone();
    }
    res
}