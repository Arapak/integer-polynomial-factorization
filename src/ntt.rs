use crate::{MODULO, ROOT};
use crate::polynomial::Polynomial;

pub fn mod_power(mut x: i64, mut exp: i64) -> i64 {
    let mut res: i64 = 1;
    while exp > 0 {
        if exp & 1 == 1 {
            res = res * x % MODULO;
        }
        x = x * x % MODULO;
        exp /= 2;
    }
    res
}

fn ntt(p: &mut Polynomial) {
    let n = p.len();
    let l = 63 - n.leading_zeros();
    let mut rt : Vec<i64> = vec![1,1];
    let mut k: usize = 2;
    let mut s: i64 = 2;
    while k < n {
        rt.resize(n, 0);
        let z = [1, mod_power(ROOT, MODULO>>s)];
        for i in k..2*k {
            rt[i] = rt[i/2] * z[i&1] % MODULO;
        }
        k *= 2;
        s += 1;
    }
    let mut rev = vec![0i64; n];
    for i in 0..n {
        rev[i] = (rev[i/2] | ((i&1) as i64) << l) / 2;
    }
    for i in 0..n {
        if (i as i64) < rev[i] {
            (p[i], p[rev[i] as usize]) = (p[rev[i] as usize], p[i])
        }
    }
    k = 1;
    while k < n {
        let mut i = 0;
        while i < n {
            for j in 0..k {
                let z = rt[j+k] * p[i+j+k] % MODULO;
                p[i+j+k] = p[i+j] - z;
                if z > p[i+j] {
                    p[i+j+k] += MODULO;
                }
                p[i+j] += z;
                if p[i+j] >= MODULO {
                    p[i+j] -= MODULO;
                }
            }
            i += 2*k;
        }
        k *= 2;
    }
}

pub fn multiply(p: &Polynomial, q: &Polynomial) -> Polynomial {
    if p.len() == 0 || q.len() == 0 {
        return vec![]
    }
    let s = p.len() + q.len() - 1;
    let b = 64 - s.leading_zeros();
    let n: usize = 1 << b;
    let inv = mod_power(n as i64, MODULO - 2);
    let mut l = p.clone();
    let mut r = q.clone();
    let mut out = vec![0; n];
    l.resize(n, 0);
    r.resize(n , 0);
    ntt(&mut l);
    ntt(&mut r);
    for i in 0..n {
        out[((-(i as i64)) & ((n-1) as i64)) as usize] = (l[i] * r[i] % MODULO) * inv % MODULO;
    }
    ntt(&mut out);
    // out[0..s].to_vec()
    // out[0..s].into_iter().map(|x| if *x > MODULO/2 { x - MODULO } else { *x }).collect()
    out[0..s].into_iter().map(|x| x % MODULO).collect()
}