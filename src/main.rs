use crate::ddf::dff;
use crate::division::{divide, long_division_mod, reciprocal};
use crate::edf::edf;
use crate::ntt::multiply;
use crate::polynomial::{gcd, scale};

mod polynomial;
mod ntt;
mod division;
mod ddf;
mod edf;
mod factorize;

// const MODULO: i64 = 17;
const MODULO: i64 = (119 << 23) + 1;
// const ROOT: i64 = 2;
const ROOT: i64 = 62;

fn main() {
    // let res = gcd(&vec![1,0,1], &vec![4,1]);
    // let res = divide(&vec![1,0,1], &vec![2,1]);
    // let res = divide(&multiply(&vec![0,1,1,1], &vec![0,1,1,1]), &vec![1,1,1,1]);
    // let res = gcd(&vec![-1,0,0,0,1], &vec![1,1]);
    // let p = vec![2,0,1,1];
    // let res = reciprocal(&p);
    // let res = dff(vec![1,1,1,1]);
    let res = edf(&vec![1,1,1,1], 1);
    println!("res: {:?}", res);
    // println!("{:?}", multiply(&res, &p));
    // let mult = multiply(&res, &p);
    // println!("mult: {:?}", mult);
    // println!("div: {:?}", divide(&scale(&vec![1], 6), &p));
}
