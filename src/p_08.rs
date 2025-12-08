use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::hash::Hash;
use std::rc::Rc;

fn read_input(path: &str) -> Vec<Pos> {
    let mut inp: Vec<Pos> = Vec::new();
    for line in read_to_string(path).expect("Unable to read file").lines() {
        let mut pos_ = line.splitn(3, ',');
        inp.push(Pos::new(
            pos_.next()
                .expect("No x coordinate found")
                .parse::<i64>()
                .expect("Unable to parse x coordinate to i64"),
            pos_.next()
                .expect("No y coordinate found")
                .parse::<i64>()
                .expect("Unable to parse y coordinate to i64"),
            pos_.next()
                .expect("No z coordinate found")
                .parse::<i64>()
                .expect("Unable to parse z coordinate to i64"),
        ));
    }
    inp
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Pos {
    x: i64,
    y: i64,
    z: i64,
}

impl Pos {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x: x, y: y, z: z }
    }

    pub fn dist(&self, p: &Pos) -> i64 {
        let dx = self.x as i64 - p.x as i64;
        let dy = self.y as i64 - p.y as i64;
        let dz = self.z as i64 - p.z as i64;

        dx * dx + dy * dy + dz * dz
    }
}

#[derive(Debug)]
pub struct Edge {
    u: Pos,
    v: Pos,
    dist: i64,
}

fn build_clusters_1(nodes: &[Pos], num_connections: usize) -> Vec<Vec<Pos>> {
    let mut remaining: HashSet<Pos> = nodes.iter().copied().collect();

    let mut edges: Vec<Edge> = Vec::new();
    for i in 0..nodes.len() {
        for j in i + 1..nodes.len() {
            edges.push(Edge {
                u: nodes[i],
                v: nodes[j],
                dist: nodes[i].dist(&nodes[j]),
            });
        }
    }
    edges.sort_by_key(|e| e.dist);

    let mut node_to_cluster: HashMap<Pos, Rc<RefCell<Vec<Pos>>>> = HashMap::new();
    let mut edges_processed = 0;

    for edge in edges.iter() {
        if edges_processed >= num_connections {
            break;
        }
        edges_processed += 1;
        let u_in = remaining.contains(&edge.u);
        let v_in = remaining.contains(&edge.v);
        match (u_in, v_in) {
            (true, true) => {
                let cluster = Rc::new(RefCell::new(vec![edge.u, edge.v]));
                node_to_cluster.insert(edge.u, Rc::clone(&cluster));
                node_to_cluster.insert(edge.v, Rc::clone(&cluster));
                remaining.remove(&edge.u);
                remaining.remove(&edge.v);
            }
            (true, false) => {
                if let Some(cluster) = node_to_cluster.get(&edge.v) {
                    cluster.borrow_mut().push(edge.u);
                    node_to_cluster.insert(edge.u, Rc::clone(cluster));
                    remaining.remove(&edge.u);
                }
            }
            (false, true) => {
                if let Some(cluster) = node_to_cluster.get(&edge.u) {
                    cluster.borrow_mut().push(edge.v);
                    node_to_cluster.insert(edge.v, Rc::clone(cluster));
                    remaining.remove(&edge.v);
                }
            }
            (false, false) => {
                let cluster_u = node_to_cluster.get(&edge.u).cloned();
                let cluster_v = node_to_cluster.get(&edge.v).cloned();

                if let (Some(cluster_u), Some(cluster_v)) = (cluster_u, cluster_v) {
                    if !Rc::ptr_eq(&cluster_u, &cluster_v) {
                        let nodes_from_v = cluster_v.borrow().clone();
                        cluster_v.borrow_mut().clear();
                        for node in nodes_from_v {
                            cluster_u.borrow_mut().push(node);
                            node_to_cluster.insert(node, Rc::clone(&cluster_u));
                        }
                    }
                }
            }
        }
    }

    for node in remaining.iter() {
        let cluster = Rc::new(RefCell::new(vec![*node]));
        node_to_cluster.insert(*node, cluster);
    }

    let mut unique_clusters = HashSet::new();
    for cluster_rc in node_to_cluster.values() {
        unique_clusters.insert(Rc::as_ptr(cluster_rc));
    }
    let clusters: Vec<Vec<Pos>> = unique_clusters
        .into_iter()
        .map(|ptr| unsafe { (&*ptr).borrow().clone() })
        .filter(|cluster| !cluster.is_empty())
        .collect();
    clusters
}

fn part_1(path: &str, num_connections: usize) -> usize {
    let input = read_input(path);
    let clusters = build_clusters_1(&input, num_connections);
    println!("{:?}", clusters);
    let mut cluster_sizes: Vec<usize> = clusters.iter().map(|c| c.len()).collect();
    cluster_sizes.sort_by(|a, b| b.cmp(a));
    cluster_sizes.iter().take(3).product()
}

pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    num_components: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            num_components: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
        } else {
            self.parent[root_y] = root_x;
            self.rank[root_x] += 1;
        }

        self.num_components -= 1;
        true
    }

    fn num_components(&self) -> usize {
        self.num_components
    }
}

fn build_mst(nodes: &[Pos]) -> (Pos, Pos) {
    let n = nodes.len();

    let mut edges: Vec<(i64, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in i + 1..n {
            edges.push((nodes[i].dist(&nodes[j]), i, j));
        }
    }
    edges.sort_unstable();

    let mut uf = UnionFind::new(n);
    let mut last_edge = None;

    for (dist, i, j) in edges.iter() {
        if uf.union(*i, *j) {
            last_edge = Some((nodes[*i], nodes[*j]));
            if uf.num_components() == 1 {
                break;
            }
        }
    }

    last_edge.unwrap()
}

fn part_2(path: &str) -> i64 {
    let input = read_input(path);
    // let (last_u, last_v) = build_clusters_2(&input);
    // Trying the MST + UnionFind based approach
    let (last_u, last_v) = build_mst(&input);
    println!("Last edge: {:?} <-> {:?}", last_u, last_v);
    last_u.x * last_v.x
}

fn build_clusters_2(nodes: &[Pos]) -> (Pos, Pos) {
    let mut remaining: HashSet<Pos> = nodes.iter().copied().collect();

    let mut edges: Vec<Edge> = Vec::new();
    for i in 0..nodes.len() {
        for j in i + 1..nodes.len() {
            edges.push(Edge {
                u: nodes[i],
                v: nodes[j],
                dist: nodes[i].dist(&nodes[j]),
            });
        }
    }
    edges.sort_by_key(|e| e.dist);

    let mut node_to_cluster: HashMap<Pos, Rc<RefCell<Vec<Pos>>>> = HashMap::new();
    let mut num_clusters = nodes.len();
    let mut last_edge: Option<(Pos, Pos)> = None;

    for edge in edges.iter() {
        if num_clusters == 1 {
            break;
        }

        let u_in = remaining.contains(&edge.u);
        let v_in = remaining.contains(&edge.v);
        match (u_in, v_in) {
            (true, true) => {
                let cluster = Rc::new(RefCell::new(vec![edge.u, edge.v]));
                node_to_cluster.insert(edge.u, Rc::clone(&cluster));
                node_to_cluster.insert(edge.v, Rc::clone(&cluster));
                remaining.remove(&edge.u);
                remaining.remove(&edge.v);
                num_clusters -= 1;
                last_edge = Some((edge.u, edge.v));
            }
            (true, false) => {
                if let Some(cluster) = node_to_cluster.get(&edge.v) {
                    cluster.borrow_mut().push(edge.u);
                    node_to_cluster.insert(edge.u, Rc::clone(cluster));
                    remaining.remove(&edge.u);
                    num_clusters -= 1;
                    last_edge = Some((edge.u, edge.v));
                }
            }
            (false, true) => {
                if let Some(cluster) = node_to_cluster.get(&edge.u) {
                    cluster.borrow_mut().push(edge.v);
                    node_to_cluster.insert(edge.v, Rc::clone(cluster));
                    remaining.remove(&edge.v);
                    num_clusters -= 1;
                    last_edge = Some((edge.u, edge.v));
                }
            }
            (false, false) => {
                let cluster_u = node_to_cluster.get(&edge.u).cloned();
                let cluster_v = node_to_cluster.get(&edge.v).cloned();

                if let (Some(cluster_u), Some(cluster_v)) = (cluster_u, cluster_v) {
                    if !Rc::ptr_eq(&cluster_u, &cluster_v) {
                        let nodes_from_v = cluster_v.borrow().clone();
                        cluster_v.borrow_mut().clear();
                        for node in nodes_from_v {
                            cluster_u.borrow_mut().push(node);
                            node_to_cluster.insert(node, Rc::clone(&cluster_u));
                        }
                        num_clusters -= 1;
                        last_edge = Some((edge.u, edge.v));
                    }
                }
            }
        }
    }

    last_edge.unwrap()
}

#[cfg(test)]
mod test {
    use crate::p_08::{part_1, part_2, read_input};

    #[test]
    fn test_sample_1() {
        const PATH: &str = "/Users/saranshagarwal/Code/aoc2025/src/p_08_sample.txt";
        println!("The solution is : {}", part_1(PATH, 10));
    }

    #[test]
    fn test_1() {
        const PATH: &str = "/Users/saranshagarwal/Code/aoc2025/src/p_08.txt";
        println!("The solution is : {}", part_1(PATH, 1000));
    }

    #[test]
    fn test_sample_2() {
        const PATH: &str = "/Users/saranshagarwal/Code/aoc2025/src/p_08_sample.txt";
        println!("The solution is : {}", part_2(PATH));
    }

    #[test]
    fn test_2() {
        const PATH: &str = "/Users/saranshagarwal/Code/aoc2025/src/p_08.txt";
        println!("The solution is : {}", part_2(PATH));
    }
}
