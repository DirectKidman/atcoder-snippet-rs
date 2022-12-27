use cargo_snippet::snippet;

#[snippet]
pub fn prime_list(n: usize) -> Vec<usize> {
    let mut list = vec![];
    let mut is_prime = vec![true; n + 1];

    for i in 2..n {
        if !is_prime[i] {
            continue;
        }
        list.push(i);

        for j in (i * i..=n).step_by(i) {
            is_prime[j] = false;
        }
    }
    list
}

#[snippet]
pub fn prime_factorize(mut n: usize) -> Vec<(usize, usize)> {
    let mut res = vec![];
    let mut i = 2;
    while i * i <= n {
        if n % i != 0 {
            i += 1;
            continue;
        }

        let mut ex = 0;
        while n % i == 0 {
            ex += 1;
            n /= i;
        }
        res.push((i, ex));

        i += 1;
    }
    if n != 1 {
        res.push((n, 1));
    }
    res
}

#[snippet("Factorize")]
use std::collections::BTreeMap;
#[snippet("Factorize")]
pub struct Factorize {
    fac: Vec<usize>,
}

#[snippet("Factorize")]
impl Factorize {
    pub fn new(max_n: usize) -> Self {
        let mut fac: Vec<usize> = (0..=max_n).collect();

        let mut i = 2;
        while i * i <= max_n {
            if fac[i] == i {
                for j in (i * i..=max_n).step_by(i) {
                    if fac[j] == j {
                        fac[j] = i;
                    }
                }
            }
            i += 1;
        }
        Factorize { fac }
    }

    pub fn factorize(&self, n: usize) -> BTreeMap<usize, usize> {
        let mut hm = BTreeMap::new();
        let mut tmp = n;
        while tmp != 1 {
            let cnt = hm.entry(self.fac[tmp]).or_insert(0);
            *cnt += 1;
            tmp /= self.fac[tmp];
        }
        hm
    }

    pub fn is_prime(&self, n: usize) -> bool {
        n > 1 && self.fac[n] == n
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn prime_list_test() {
        assert_eq!(prime_list(10), vec![2, 3, 5, 7]);
    }

    #[test]
    fn factorize_test() {
        let fac = Factorize::new(1000);
        let mut ans = BTreeMap::new();
        ans.insert(2, 3);
        ans.insert(3, 1);
        assert_eq!(fac.factorize(24), ans);
    }

    #[test]
    fn prime_factorize_test() {
        let factors = prime_factorize(24);
        assert_eq!(vec![(2, 3), (3, 1)], factors);
    }

    #[test]
    fn is_prime_test() {
        let fac = Factorize::new(100);
        assert!(fac.is_prime(37));
        assert!(!fac.is_prime(1));
        assert!(!fac.is_prime(24));
    }
}
