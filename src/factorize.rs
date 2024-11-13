use crate::polynomial::{add, multiply_by_constant, Polynomial, reduce, scale, subtract};

pub fn long_division(p: &Polynomial, q: &Polynomial) -> (Polynomial, Polynomial) {
    if p.len() < q.len() {
        return (vec![], p.clone())
    }
    let multiplier = p[p.len() - 1];
    let (dividend, remainder) = long_division(&reduce(
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