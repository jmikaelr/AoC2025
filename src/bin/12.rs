use std::collections::HashSet;

advent_of_code::solution!(12);

#[derive(Debug)]
struct Shape {
    idx: usize,
    mask: u16, // 3x3 bits, bit = y*3 + x
}

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    counts: Vec<u32>,
}

fn rows_to_mask(r0: &str, r1: &str, r2: &str) -> u16 {
    let rows = [r0, r1, r2];
    let mut m = 0u16;
    for (y, row) in rows.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if ch == '#' {
                m |= 1u16 << (y * 3 + x);
            }
        }
    }
    m
}

fn rot90(m: u16) -> u16 {
    let mut r = 0u16;
    for y in 0..3 {
        for x in 0..3 {
            let src = y * 3 + x;
            let nx = 2 - y;
            let ny = x;
            let dst = ny * 3 + nx;
            if ((m >> src) & 1) == 1 {
                r |= 1u16 << dst;
            }
        }
    }
    r
}

fn flip_h(m: u16) -> u16 {
    let mut r = 0u16;
    for y in 0..3 {
        for x in 0..3 {
            let src = y * 3 + x;
            let nx = 2 - x;
            let dst = y * 3 + nx;
            if ((m >> src) & 1) == 1 {
                r |= 1u16 << dst;
            }
        }
    }
    r
}

fn mask_cells_normalized(m: u16) -> (Vec<(usize, usize)>, usize, usize) {
    let mut pts: Vec<(usize, usize)> = Vec::new();
    for y in 0..3 {
        for x in 0..3 {
            let bit = y * 3 + x;
            if ((m >> bit) & 1) == 1 {
                pts.push((x, y));
            }
        }
    }

    let min_x = pts.iter().map(|(x, _)| *x).min().unwrap_or(0);
    let min_y = pts.iter().map(|(_, y)| *y).min().unwrap_or(0);

    for p in &mut pts {
        p.0 -= min_x;
        p.1 -= min_y;
    }

    pts.sort_unstable();

    let max_x = pts.iter().map(|(x, _)| *x).max().unwrap_or(0);
    let max_y = pts.iter().map(|(_, y)| *y).max().unwrap_or(0);

    (pts, max_x + 1, max_y + 1)
}

fn normalize_mask(m: u16) -> u16 {
    let (pts, _w, _h) = mask_cells_normalized(m);
    let mut out = 0u16;
    for (x, y) in pts {
        out |= 1u16 << (y * 3 + x);
    }
    out
}

fn orientations(base: u16) -> Vec<u16> {
    let mut out = Vec::new();
    let mut seen = HashSet::<u16>::new();

    for do_flip in [false, true] {
        let mut m = if do_flip { flip_h(base) } else { base };
        for _ in 0..4 {
            let nm = normalize_mask(m);
            if seen.insert(nm) {
                out.push(nm);
            }
            m = rot90(m);
        }
    }
    out
}

fn placements_for_shape(w: usize, h: usize, shape_orients: &[u16]) -> Vec<Vec<usize>> {
    let mut out: Vec<Vec<usize>> = Vec::new();

    for &m in shape_orients {
        let (cells, ow, oh) = mask_cells_normalized(m);
        if ow > w || oh > h {
            continue;
        }

        for oy in 0..=h - oh {
            for ox in 0..=w - ow {
                let mut idxs: Vec<usize> = Vec::with_capacity(cells.len());
                for &(x, y) in &cells {
                    idxs.push((oy + y) * w + (ox + x));
                }
                idxs.sort_unstable();
                out.push(idxs);
            }
        }
    }

    out.sort();
    out.dedup();
    out
}

#[derive(Clone)]
struct DLX {
    left: Vec<usize>,
    right: Vec<usize>,
    up: Vec<usize>,
    down: Vec<usize>,
    col: Vec<usize>,
    row_id: Vec<usize>,
    size: Vec<usize>,
    is_primary: Vec<bool>,
    head: usize,
    n_cols: usize,
}

impl DLX {
    fn new(n_cols: usize, is_primary: Vec<bool>) -> Self {
        let head = 0usize;
        let mut left = vec![0; n_cols + 1];
        let mut right = vec![0; n_cols + 1];
        let mut up = vec![0; n_cols + 1];
        let mut down = vec![0; n_cols + 1];
        let col = vec![0; n_cols + 1];
        let row_id = vec![0; n_cols + 1];
        let mut size = vec![0; n_cols + 1];

        for c in 0..=n_cols {
            up[c] = c;
            down[c] = c;
            left[c] = c;
            right[c] = c;
            size[c] = 0;
        }

        let mut last = head;
        for c in 1..=n_cols {
            if is_primary[c] {
                right[last] = c;
                left[c] = last;
                last = c;
            }
        }
        right[last] = head;
        left[head] = last;

        Self {
            left,
            right,
            up,
            down,
            col,
            row_id,
            size,
            is_primary,
            head,
            n_cols,
        }
    }

    fn add_row(&mut self, rid: usize, cols: &[usize]) {
        let mut first_node: Option<usize> = None;
        let mut prev_node: usize = 0;

        for &c in cols {
            let node = self.left.len();
            self.left.push(node);
            self.right.push(node);
            self.up.push(0);
            self.down.push(0);
            self.col.push(c);
            self.row_id.push(rid);

            let u = self.up[c];
            self.down[u] = node;
            self.up[node] = u;
            self.down[node] = c;
            self.up[c] = node;

            self.size[c] += 1;

            if first_node.is_none() {
                first_node = Some(node);
                prev_node = node;
            } else {
                self.right[prev_node] = node;
                self.left[node] = prev_node;
                prev_node = node;
            }
        }

        if let Some(first) = first_node {
            self.right[prev_node] = first;
            self.left[first] = prev_node;
        }
    }

    fn cover(&mut self, c: usize) {
        if self.is_primary[c] {
            let l = self.left[c];
            let r = self.right[c];
            self.right[l] = r;
            self.left[r] = l;
        }

        let mut i = self.down[c];
        while i != c {
            let mut j = self.right[i];
            while j != i {
                let cu = self.col[j];
                let u = self.up[j];
                let d = self.down[j];
                self.down[u] = d;
                self.up[d] = u;
                self.size[cu] -= 1;
                j = self.right[j];
            }
            i = self.down[i];
        }
    }

    fn uncover(&mut self, c: usize) {
        let mut i = self.up[c];
        while i != c {
            let mut j = self.left[i];
            while j != i {
                let cu = self.col[j];
                self.size[cu] += 1;
                let u = self.up[j];
                let d = self.down[j];
                self.down[u] = j;
                self.up[d] = j;
                j = self.left[j];
            }
            i = self.up[i];
        }

        if self.is_primary[c] {
            let l = self.left[c];
            let r = self.right[c];
            self.right[l] = c;
            self.left[r] = c;
        }
    }

    fn choose_column(&self) -> Option<usize> {
        let mut c = self.right[self.head];
        if c == self.head {
            return None;
        }

        let mut best = c;
        let mut best_sz = self.size[c];

        c = self.right[c];
        while c != self.head {
            let sz = self.size[c];
            if sz < best_sz {
                best_sz = sz;
                best = c;
                if best_sz == 0 {
                    break;
                }
            }
            c = self.right[c];
        }

        Some(best)
    }

    fn search(&mut self, solution: &mut Vec<usize>) -> bool {
        let c = match self.choose_column() {
            None => return true,
            Some(x) => x,
        };

        if self.size[c] == 0 {
            return false;
        }

        self.cover(c);

        let mut r = self.down[c];
        while r != c {
            solution.push(self.row_id[r]);

            let mut j = self.right[r];
            while j != r {
                self.cover(self.col[j]);
                j = self.right[j];
            }

            if self.search(solution) {
                return true;
            }

            let mut j2 = self.left[r];
            while j2 != r {
                self.uncover(self.col[j2]);
                j2 = self.left[j2];
            }

            solution.pop();
            r = self.down[r];
        }

        self.uncover(c);
        false
    }
}

fn solve_region_dlx(w: usize, h: usize, shape_orients: &[Vec<u16>], counts: &[u32]) -> bool {
    let n_shapes = shape_orients.len();
    let area = w * h;

    let mut need_area = 0usize;
    for i in 0..n_shapes.min(counts.len()) {
        let (cells, _, _) = mask_cells_normalized(shape_orients[i][0]);
        need_area += (counts[i] as usize) * cells.len();
    }
    if need_area > area {
        return false;
    }

    let mut shape_placements: Vec<Vec<Vec<usize>>> = Vec::with_capacity(n_shapes);
    for i in 0..n_shapes {
        shape_placements.push(placements_for_shape(w, h, &shape_orients[i]));
    }

    for i in 0..n_shapes.min(counts.len()) {
        if counts[i] > 0 && shape_placements[i].is_empty() {
            return false;
        }
    }

    let mut instances: Vec<usize> = Vec::new();
    for i in 0..n_shapes.min(counts.len()) {
        for _ in 0..counts[i] {
            instances.push(i);
        }
    }
    if instances.is_empty() {
        return true;
    }

    instances.sort_by_key(|&si| shape_placements[si].len());

    let n_primary = instances.len();
    let n_secondary = area;
    let n_cols = n_primary + n_secondary;

    let mut is_primary = vec![false; n_cols + 1];
    for c in 1..=n_primary {
        is_primary[c] = true;
    }

    let mut dlx = DLX::new(n_cols, is_primary);

    let mut rid: usize = 0;
    for (k, &shape_idx) in instances.iter().enumerate() {
        let piece_col = 1 + k;

        for placement in &shape_placements[shape_idx] {
            let mut cols: Vec<usize> = Vec::with_capacity(1 + placement.len());
            cols.push(piece_col);
            for &cell in placement {
                cols.push(1 + n_primary + cell);
            }
            cols.sort_unstable();
            dlx.add_row(rid, &cols);
            rid += 1;
        }
    }

    let mut sol = Vec::new();
    dlx.search(&mut sol)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut shapes, regions) = parse_input(input);
    shapes.sort_by_key(|s| s.idx);

    let mut shape_orients: Vec<Vec<u16>> = Vec::with_capacity(shapes.len());
    for s in &shapes {
        shape_orients.push(orientations(s.mask));
    }

    let mut ok = 0u64;
    for r in regions {
        if solve_region_dlx(r.width, r.height, &shape_orients, &r.counts) {
            ok += 1;
        }
    }

    Some(ok)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

fn parse_input(input: &str) -> (Vec<Shape>, Vec<Region>) {
    let mut shapes: Vec<Shape> = Vec::new();
    let mut regions: Vec<Region> = Vec::new();

    let mut lines = input.lines().map(str::trim).peekable();

    while let Some(&line) = lines.peek() {
        if line.is_empty() {
            lines.next();
            continue;
        }
        if line.contains('x') && line.contains(':') {
            break;
        }

        let header = lines.next().unwrap();
        let (idx_str, _) = header.split_once(':').unwrap();
        let idx: usize = idx_str.trim().parse().unwrap();

        let r0 = lines.next().unwrap();
        let r1 = lines.next().unwrap();
        let r2 = lines.next().unwrap();

        let mask = normalize_mask(rows_to_mask(r0, r1, r2));
        shapes.push(Shape { idx, mask });
    }

    while let Some(line) = lines.next() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (wh, rest) = line.split_once(':').unwrap();
        let (w_str, h_str) = wh.split_once('x').unwrap();

        let width: usize = w_str.trim().parse().unwrap();
        let height: usize = h_str.trim().parse().unwrap();

        let counts: Vec<u32> = rest
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        regions.push(Region {
            width,
            height,
            counts,
        });
    }

    (shapes, regions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
