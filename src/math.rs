use cargo_snippet::snippet;

#[snippet]
pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[snippet]
#[snippet(include = "gcd")]
pub fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

#[snippet]
pub fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (d, q, p) = ext_gcd(b, a % b);
        (d, p, q - a / b * p)
    }
}

#[snippet]
#[snippet(include = "ext_gcd")]
pub fn mod_inv(a: usize, m: i64) -> usize {
    let (_, mut x, _) = ext_gcd(a as i64, m as i64);
    x %= m;
    if x < 0 {
        x += m
    }

    x as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ext_gcd_test() {
        assert_eq!(ext_gcd(3, 2), (1, 1, -1));
        assert_eq!(ext_gcd(8, 3), (1, -1, 3));
    }

    #[test]
    fn mod_inv_test() {
        assert_eq!(mod_inv(3, 1_000_000_007), 333333336);
    }
}
