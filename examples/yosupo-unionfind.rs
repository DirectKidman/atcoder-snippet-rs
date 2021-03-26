macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

pub struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
}
impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            par: (0..n).into_iter().collect(),
            siz: vec![1; n],
        }
    }
    pub fn root(&mut self, x: usize) -> usize {
        if self.par[x] != x {
            self.par[x] = self.root(self.par[x]);
        }
        self.par[x]
    }
    pub fn unite(&mut self, a: usize, b: usize) {
        let (mut ra, mut rb) = (self.root(a), self.root(b));
        if ra == rb {
            return;
        }
        if self.siz[ra] < self.siz[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.siz[ra] += self.siz[rb];
        self.par[rb] = ra;
    }
    pub fn same(&mut self, a: usize, b: usize) -> bool {
        self.root(a) == self.root(b)
    }
    /// This function is cited from ac-library-rs.
    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let size = self.par.len();
        let mut root_vec = vec![0; size];
        let mut group_size = vec![0; size];
        for i in 0..size {
            root_vec[i] = self.root(i);
            group_size[root_vec[i]] += 1;
        }
        let mut result = vec![Vec::new(); size];
        for i in 0..size {
            result[i].reserve(group_size[i]);
        }
        for i in 0..size {
            result[root_vec[i]].push(i);
        }
        result
            .into_iter()
            .filter(|x| !x.is_empty())
            .collect::<Vec<Vec<usize>>>()
    }
}
fn main() {
    input! {
        n: usize,
        q: usize,
        e: [(usize,usize,usize); q],
    }

    let mut uf = UnionFind::new(n);
    for (t, u, v) in e {
        match t {
            0 => uf.unite(u, v),
            1 => {
                if uf.same(u, v) {
                    println!("1");
                } else {
                    println!("0");
                }
            }
            _ => panic!("Why"),
        }
    }
}
