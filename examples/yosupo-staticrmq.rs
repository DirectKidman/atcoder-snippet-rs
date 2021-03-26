// This code is cited from https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e
macro_rules! get {
    ($t:ty) => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<$t>().unwrap()
        }
    };
    ($($t:ty),*) => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                $(iter.next().unwrap().parse::<$t>().unwrap(),)*
            )
        }
    };
    ($t:ty; $n:expr) => {
        (0..$n).map(|_|
            get!($t)
        ).collect::<Vec<_>>()
    };
    ($($t:ty),*; $n:expr) => {
        (0..$n).map(|_|
            get!($($t),*)
        ).collect::<Vec<_>>()
    };
    ($t:ty ;;) => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.split_whitespace()
                .map(|t| t.parse::<$t>().unwrap())
                .collect::<Vec<_>>()
        }
    };
    ($t:ty ;; $n:expr) => {
        (0..$n).map(|_| get!($t ;;)).collect::<Vec<_>>()
    };
}

pub struct SparseTable<T: Ord + Copy + Default> {
    table: Vec<Vec<T>>,
    log_table: Vec<usize>,
}
impl<T: Ord + Copy + Default> SparseTable<T> {
    pub fn new(arr: &[T]) -> SparseTable<T> {
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
        SparseTable { table, log_table }
    }
    #[inline]
    pub fn query(&self, l: usize, r: usize) -> T {
        let k = self.log_table[r - l];
        std::cmp::min(self.table[k][l], self.table[k][r - (1 << k)])
    }
}
fn main() {
    let (_, q) = get!(usize, usize);
    let a = get!(usize ;;);
    let lr = get!(usize,usize;q);

    let spt = SparseTable::new(&a);

    let mut res = vec![];
    for (l, r) in lr {
        res.push(spt.query(l, r));
    }

    for e in res {
        println!("{}", e);
    }
}
