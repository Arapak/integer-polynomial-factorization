use rand::distributions::Uniform;
use rand::Rng;
use crate::ddf::mod_power_polynomial;
use crate::division::long_division_mod;
use crate::MODULO;
use crate::ntt::multiply;
use crate::polynomial::{gcd, normalize, Polynomial, reduce, subtract};

pub fn random_polynomial(degree: usize) -> Polynomial {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0, MODULO - 1);

    [(0..degree-1).map(|_| rng.sample(&range)).collect(), vec![1]].concat()
}

pub fn edf(p: &Polynomial, d: i64) -> Vec<Polynomial> {
    let n = p.len() - 1;
    let r = n / d as usize;
    let mut res: Vec<Polynomial> = vec![p.clone()];
    while res.len() < r {
        let h = reduce(random_polynomial(n));
        let mut g: Polynomial = gcd(p, &h);
        if g.len() == 1 {
            g = vec![1];
            let hq = mod_power_polynomial(h, (MODULO - 1) / 2, p);
            for _ in 0..d {
                g = mod_power_polynomial(g, MODULO, p);
                g = multiply(&g, &hq);
            }
            g = subtract(&g, &vec![1]);
        }
        g = reduce(g);
        let mut new_res: Vec<Polynomial> = vec![];
        for x in res {
            let gc = if x.len() >= g.len() { gcd(&x, &g) } else { gcd(&g, &x) };
            if gc.len() != 1 && gc.len() != x.len() {
                new_res.push(normalize(&long_division_mod(&x, &gc).0));
                new_res.push(normalize(&gc));
            } else {
                new_res.push(x);
            }
        }
        res = new_res;
        // println!("####### res: {:?} i: {:?}", res, i);
    }
    res
}