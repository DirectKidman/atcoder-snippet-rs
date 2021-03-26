pub mod graph;
pub mod sparce_table;
pub mod union_find;

#[cfg(test)]
mod tests {
    use super::sparce_table::SparseTable;
    use super::union_find::UnionFind;

    #[test]
    fn uf_test() {
        let mut uf = UnionFind::new(4);
        uf.unite(0, 1);
        uf.unite(1, 2);
        assert!(uf.same(0, 2));
        assert_eq!(vec![vec![0, 1, 2], vec![3]], uf.groups())
    }

    #[test]
    fn spt_test() {
        let v = vec![3, 2, 4, 5, 1, 32, 12, 9];
        let spt = SparseTable::new(&v);

        assert_eq!(spt.query(1, 3), 2);
        assert_eq!(spt.query(5, 8), 9);
        assert_eq!(spt.query(0, 8), 1);
    }
}
