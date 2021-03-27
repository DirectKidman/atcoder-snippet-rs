use cargo_snippet::snippet;

#[snippet("Graph")]
type Graph = Vec<Vec<Edge>>;

#[snippet("Graph")]
#[derive(Debug, Copy, Clone)]
pub struct Edge {
    pub to: usize,
    pub cost: usize,
}

#[snippet("Graph")]
impl Edge {
    pub fn new(to: usize, cost: usize) -> Self {
        Self { to, cost }
    }
}

#[snippet]
#[snippet(include = "Graph")]
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shortest_path_test() {
        let mut g: Graph = vec![Vec::new(); 5];
        g[0].push(Edge::new(3, 5));
        g[0].push(Edge::new(4, 3));
        g[2].push(Edge::new(4, 2));
        g[4].push(Edge::new(3, 10));
        g[4].push(Edge::new(0, 7));
        g[2].push(Edge::new(1, 5));
        g[1].push(Edge::new(0, 1));

        let (cost, path) = shortest_path(&g, 2, 3);
        assert_eq!(cost, 11);
        assert_eq!(path, vec![2, 1, 0, 3]);
    }
}
