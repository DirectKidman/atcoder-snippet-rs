use cargo_snippet::snippet;

#[snippet("SPT")]
pub struct SparseTable<T: Ord + Copy + Default> {
    table: Vec<Vec<T>>,
    log_table: Vec<usize>,
}

#[snippet("SPT")]
impl<T: Ord + Copy + Default> SparseTable<T> {
    pub fn new(arr: &[T]) -> Self {
        let mut b = 0;
        let n = arr.len();
        while (1 << b) <= n {
            b += 1;
        }

        let mut table: Vec<Vec<T>> = vec![vec![T::default(); n]; b];
        for i in 0..n {
            table[0][i] = arr[i];
        }

        for i in 1..b {
            for j in 0..=(n - (1 << i)) {
                table[i][j] = std::cmp::min(table[i - 1][j], table[i - 1][j + (1 << (i - 1))]);
            }
        }
        let mut log_table = vec![0; n + 1];
        for i in 2..(n + 1) {
            log_table[i] = log_table[i >> 1] + 1;
        }

        Self { table, log_table }
    }

    #[inline]
    pub fn query(&self, l: usize, r: usize) -> T {
        let k = self.log_table[r - l];

        std::cmp::min(self.table[k][l], self.table[k][r - (1 << k)])
    }
}

#[cfg(test)]
mod tests {
    use super::SparseTable;
    #[test]
    fn spt_test() {
        let v = vec![3, 2, 4, 5, 1, 32, 12, 9];
        let spt = SparseTable::new(&v);

        assert_eq!(spt.query(1, 3), 2);
        assert_eq!(spt.query(5, 8), 9);
        assert_eq!(spt.query(0, 8), 1);
    }
}
