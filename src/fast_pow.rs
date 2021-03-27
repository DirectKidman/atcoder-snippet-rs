fn pow_mod(x: usize, k: usize, md: usize) -> usize {
    let mut tmp = 1;
    let mut m = k;
    let mut d = x;
    if k == 0 {
        return 1;
    }
    while m > 0 {
        if m & 1 == 1 {
            tmp = (tmp * d) % md;
        }
        d = (d * d) % m;
        m >>= 1;
    }
    tmp
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pow_mod_test() {
        assert_eq!(2 + 2, 4);
    }
}
