use crate::math::mod_pow;
use std::collections::HashMap;

pub fn miller_rabin(n: usize) -> bool {
    if n <= 1 {
        return false;
    }

    let primes = [2, 3, 5, 7, 11, 13];
    if primes.iter().any(|&p| p == n) {
        return true;
    };
    if primes.iter().any(|&p| p % n == 0) {
        return false;
    };

    let vec = if n < 4759123141 {
        vec![2, 7, 61]
    } else {
        vec![2, 325, 9375, 28178, 450775, 9780504, 1795265022]
    };

    let s = (n - 1).trailing_zeros();
    let d = (n - 1) >> s;
    for a in vec {
        let mut t = d;
        let mut y = mod_pow(a, t, n);
        while t != n - 1 && y != 1 && y != n - 1 {
            y = (y * y) % n;
            t <<= 1;
        }
        if y != n - 1 && t & 1 == 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn miller_rabin_test() {
        assert!(miller_rabin((1 << 31) - 1));
        assert!(!miller_rabin(1999 * 1997));
        assert!(!miller_rabin(512461));
    }
}
