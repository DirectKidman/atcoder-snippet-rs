use cargo_snippet::snippet;

#[snippet("Comb")]
pub struct Combination {
    fac: Vec<usize>,
    finv: Vec<usize>,
    m: usize,
}

#[snippet("Comb")]
impl Combination {
    pub fn new(n: usize, m: usize) -> Self {
        let mut inv = vec![1; n + 1];
        let mut fac = vec![1; n + 1];
        let mut finv = vec![1; n + 1];

        for i in 2..(n + 1) {
            fac[i] = (fac[i - 1] * i) % m;
            inv[i] = m - (inv[m % i] * (m / i) % m);
            finv[i] = (finv[i - 1] * inv[i]) % m;
        }

        Combination { fac, finv, m }
    }

    pub fn comb(&self, n: usize, k: usize) -> usize {
        if n < k {
            return 0;
        }

        (self.fac[n] * (self.finv[k] * self.finv[n - k] % self.m)) % self.m
    }
}
