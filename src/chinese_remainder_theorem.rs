use cargo_snippet::snippet;
use crate::math::ext_gcd;

#[snippet("CRT")]
#[snippet(include = "ext_gcd")]
pub fn chinese_remainder_theorem(b: &[i64], modulo: &[i64]) -> Option<(i64, i64)> {
    let (mut result, mut m) = (0, 1);
    for i in 0..b.len() {
        let (d, p, _) = ext_gcd(m, modulo[i]);
        if (b[i] - result) % d != 0 {
            return None;
        }
        let tmp = ((b[i] - result) / d * p) % (modulo[i] / d);
        result += m * tmp;
        m *= modulo[i] / d;
    }
    Some(((result % m + m) % m, m))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crt_test() {
        let a = vec![19i64, 0i64];
        let m = vec![100i64, 23i64];

        assert_eq!(chinese_remainder_theorem(&a, &m), Some((1219, 2300)));
    }
}
