use cargo_snippet::snippet;

#[snippet("Segtree")]
pub struct Segtree<T: Copy> {
    n: usize,
    data: Vec<T>,
    identity: T,
    op: fn(T, T) -> T,
}

#[snippet("Segtree")]
impl<T: Copy> Segtree<T> {
    pub fn new(arr: &Vec<T>, identity: T, op: fn(T, T) -> T) -> Self {
        let n = arr.len();
        let mut b = 1;
        while b < n {
            b <<= 1;
        }
        let mut data = vec![identity; 2 * b];
        for i in 0..n {
            data[i + b] = arr[i];
        }

        for i in (0..b).rev() {
            data[i] = op(data[(i << 1)], data[(i << 1) + 1]);
        }
        Self {
            n: b,
            data,
            identity,
            op,
        }
    }

    pub fn set(&mut self, mut id: usize, val: T) {
        id += self.n;
        self.data[id] = val;
        id >>= 1;
        while (id >> 1) > 0 {
            self.data[id] = (self.op)(self.data[(id << 1)], self.data[(id << 1) + 1]);
            id >>= 1;
        }
    }

    pub fn get(&self, i: usize) -> T {
        self.data[i + self.n]
    }

    pub fn prod(&self, mut l: usize, mut r: usize) -> T {
        let mut vl = self.identity;
        let mut vr = self.identity;
        l += self.n;
        r += self.n;
        while l < r {
            if l & 1 == 1 {
                vl = (self.op)(vl, self.data[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                vr = (self.op)(vr, self.data[r]);
            }

            l >>= 1;
            r >>= 1;
        }

        (self.op)(vl, vr)
    }

    pub fn all_prod(&self) -> T {
        self.data[1]
    }
}

#[cfg(test)]
mod tests {
    use super::Segtree;

    #[test]
    fn segtree_test() {
        let v = vec![1, 3, 5, 2, 6];
        let mut seg = Segtree::new(&v, 0, |x, y| std::cmp::max(x, y));

        assert_eq!(seg.prod(1, 3), 5);
        seg.set(2, 4);
        assert_eq!(seg.prod(0, 5), 6);
        assert_eq!(seg.all_prod(), 6);
    }
}
