// This get macro is cited from https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e
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

fn main() {
    let (_, q) = get!(usize, usize);
    let a = get!(usize ;;);

    let query = get!(usize,usize,usize; q);
    let mut seg = Segtree::new(&a, 0, |x, y| x + y);
    for q in query {
        if q.0 == 0 {
            let tmp = seg.get(q.1);
            seg.set(q.1, tmp + q.2);
        } else {
            let res = seg.prod(q.1, q.2);
            println!("{}", res);
        }
    }
}

pub struct Segtree<T: Copy> {
    n: usize,
    data: Vec<T>,
    identity: T,
    op: fn(T, T) -> T,
}
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
        while id > 0 {
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
