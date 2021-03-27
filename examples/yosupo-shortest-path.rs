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
    let (n, m, s, t) = get!(usize, usize, usize, usize);
    let mut g: Graph = vec![vec![]; n];

    for _ in 0..m {
        let (a, b, c) = get!(usize, usize, usize);
        g[a].push(Edge::new(b, c));
    }

    let (cost, path) = shortest_path(&g, s, t);
    if cost == usize::MAX / 2 {
        println!("{}", -1);
    } else {
        println!("{} {}", cost, path.len() - 1);
        for i in 0..path.len() - 1 {
            println!("{} {}", path[i], path[i + 1]);
        }
    }
}

type Graph = Vec<Vec<Edge>>;
#[derive(Debug, Copy, Clone)]
pub struct Edge {
    pub to: usize,
    pub cost: usize,
}
impl Edge {
    pub fn new(to: usize, cost: usize) -> Self {
        Self { to, cost }
    }
}
pub fn shortest_path(graph: &Graph, start: usize, goal: usize) -> (usize, Vec<usize>) {
    use std::cmp::Reverse;
    let mut dist = vec![usize::MAX / 2; graph.len()];
    let mut prev = vec![None; graph.len()];
    dist[start] = 0;
    let mut pq = std::collections::BinaryHeap::new();
    pq.push(Reverse((0, start)));
    while let Some(Reverse((cost, pos))) = pq.pop() {
        if dist[pos] < cost {
            continue;
        }
        for edge in &graph[pos] {
            if dist[edge.to] > dist[pos] + edge.cost {
                dist[edge.to] = dist[pos] + edge.cost;
                pq.push(Reverse((dist[edge.to], edge.to)));
                prev[edge.to] = Some(pos);
            }
        }
    }
    let mut id = goal;
    let mut path = vec![goal];
    while let Some(v) = prev[id] {
        path.push(v);
        id = v;
    }
    path.reverse();
    (dist[goal], path)
}
