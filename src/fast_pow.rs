use cargo_snippet::snippet;

#[snippet]
pub fn pow_mod(x: usize, mut p: usize, md: usize) -> usize {
    let mut res = 1;
    let mut d = x;
    if p == 0 {
        return 1;
    }
    while p > 0 {
        if p & 1 == 1 {
            res = (res * d) % md;
        }

        d = (d * d) % md;
        p >>= 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pow_mod_test() {
        assert_eq!(pow_mod(2, 3, 3), 2);
        assert_eq!(pow_mod(3, 1_000_000_005, 1_000_000_007), 333333336);
    }
}
