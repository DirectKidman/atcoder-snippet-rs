use std::cmp::min;
// use std::fmt;
const INF: i64 = 1 << 31;

#[allow(dead_code)]
pub struct LazySegmentTree {
    n: usize,
    sz: usize,
    node: Vec<i64>,
    lazy: Vec<i64>,
    lazy_flag: Vec<bool>,
}

impl LazySegmentTree {
    pub fn new(x: &Vec<i64>) -> Self {
        let sz = x.len();
        let mut n = 1;
        while n < sz {
            n *= 2;
        }
        let mut node = vec![INF; 2 * n - 1];
        let lazy = vec![0; 2 * n - 1];
        let lazy_flag = vec![false; 2 * n - 1];

        for i in 0..sz {
            node[i + n - 1] = x[i];
        }
        for i in 0..n - 1 {
            let id = n - 2 - i;
            node[id] = min(node[id * 2 + 1], node[id * 2 + 2]);
        }
        Self {
            n,
            sz,
            node,
            lazy,
            lazy_flag,
        }
    }

    pub fn eval(&mut self, k: usize, l: usize, r: usize) {
        if self.lazy[k] != 0 {
            self.node[k] += self.lazy[k];

            if r - l > 1 {
                if self.lazy[2 * k + 1] != INF {
                    self.lazy[2 * k + 1] += self.lazy[k];
                }
                if self.lazy[2 * k + 2] != INF {
                    self.lazy[2 * k + 2] += self.lazy[k];
                }
            }
            self.lazy[k] = 0;
        }
    }

    pub fn update_sub(&mut self, a: usize, b: usize, x: i64, k: usize, l: usize, r: usize) {
        self.eval(k, l, r);
        if b <= l || r <= a {
            return;
        }
        if a <= l && r <= b {
            self.lazy[k] += x;
            self.eval(k, l, r);
        } else {
            self.update_sub(a, b, x, 2 * k + 1, l, (l + r) / 2);
            self.update_sub(a, b, x, 2 * k + 2, (l + r) / 2, r);
            self.node[k] = min(self.node[2 * k + 1], self.node[2 * k + 2]);
        }
    }
    pub fn update(&mut self, a: usize, b: usize, x: i64) {
        self.update_sub(a, b, x, 0, 0, self.n);
    }
    pub fn find_sub(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> i64 {
        if b <= l || r <= a {
            return INF;
        }
        self.eval(k, l, r);
        if a <= l && r <= b {
            return self.node[k];
        }
        let vl = self.find_sub(a, b, 2 * k + 1, l, (l + r) / 2);
        let vr = self.find_sub(a, b, 2 * k + 2, (l + r) / 2, r);
        min(vl, vr)
    }
    pub fn find(&mut self, a: usize, b: usize) -> i64 {
        self.find_sub(a, b, 0, 0, self.n)
    }
}

// impl fmt::Display for SegmentTree{
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         for i in 0..self.n{
//             write!(f,"{} ",self.node[i+self.sz-1])?;
//         }
//         Ok(())
//     }
// }
