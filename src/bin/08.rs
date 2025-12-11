advent_of_code::solution!(8);

struct Edge {
    dist: f64,
    a: usize,
    b: usize,
}

#[derive(Clone)]
struct Point {
    x: u64,
    y: u64,
    z: u64,
}

struct DSU {
    parent: Vec<usize>,
    size: Vec<u64>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] != i {
            let root = self.find(self.parent[i]);
            self.parent[i] = root;
        }
        self.parent[i]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);

        if ra == rb {
            return false;
        }

        if self.size[ra] < self.size[rb] {
            // XOR swap trick?
            // std::mem::swap
            ra ^= rb;
            rb ^= ra;
            ra ^= rb;
        }

        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];

        true
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let points = parse_input(input);
    let n = points.len();

    let mut edges = Vec::new();
    for a in 0..n {
        for b in a + 1..n {
            let dist = distance(&points[a], &points[b]);
            edges.push(Edge { dist, a, b });
        }
    }
    edges.sort_by(|e1, e2| e1.dist.partial_cmp(&e2.dist).unwrap());

    let mut dsu = DSU::new(n);

    for edge in edges.iter().take(1000) {
        dsu.union(edge.a, edge.b);
    }

    let mut sizes = Vec::new();
    for i in 0..n {
        if dsu.parent[i] == i {
            sizes.push(dsu.size[i]);
        }
    }

    sizes.sort_unstable_by(|a, b| b.cmp(a));

    Some(sizes[0] * sizes[1] * sizes[2])
}

pub fn part_two(input: &str) -> Option<u64> {
    let points = parse_input(input);
    let n = points.len();

    let mut edges = Vec::new();
    for a in 0..n {
        for b in a + 1..n {
            let dist = distance(&points[a], &points[b]);
            edges.push(Edge { dist, a, b });
        }
    }
    edges.sort_by(|e1, e2| e1.dist.partial_cmp(&e2.dist).unwrap());

    let mut dsu = DSU::new(n);
    let mut comp = n;
    let mut last_edge_index: Option<usize> = None;

    for (i, edge) in edges.iter().enumerate() {
        if dsu.union(edge.a, edge.b) {
            comp -= 1;
            last_edge_index = Some(i);
        }
        if comp == 1 {
            break;
        }
    }

    let idx = last_edge_index?;
    let edge = &edges[idx];

    let x1 = points[edge.a].x;
    let x2 = points[edge.b].x;
    Some(x1 * x2)
}

fn distance(p1: &Point, p2: &Point) -> f64 {
    let dx = (p1.x as f64) - (p2.x as f64);
    let dy = (p1.y as f64) - (p2.y as f64);
    let dz = (p1.z as f64) - (p2.z as f64);

    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(",");

            let x = parts.next().unwrap().trim().parse::<u64>().unwrap();
            let y = parts.next().unwrap().trim().parse::<u64>().unwrap();
            let z = parts.next().unwrap().trim().parse::<u64>().unwrap();

            Point { x, y, z }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
