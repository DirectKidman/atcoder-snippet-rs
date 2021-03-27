use cargo_snippet::snippet;

#[snippet("UFT")]
pub struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
}

#[snippet("UFT")]
impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
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

#[cfg(test)]
mod tests {
    use super::UnionFind;

    #[test]
    fn uf_test() {
        let mut uf = UnionFind::new(4);
        uf.unite(0, 1);
        uf.unite(1, 2);
        assert!(uf.same(0, 2));
        assert_eq!(vec![vec![0, 1, 2], vec![3]], uf.groups())
    }
}
