use std::f64::consts::PI;

#[derive(Debug)]
pub struct Graph {
    pub nodes: Vec<usize>,
    pub adjacency: Vec<usize>,
}

impl Graph {
    pub fn new(input: (Vec<usize>, Vec<(usize, usize)>)) -> Self {
        let nodes = input.0;
        let edges = input.1;
        let n = nodes.len();

        let mut adjacency = vec![0; n];

        for (u, v) in edges {
            adjacency[u] |= 1 << v;
            adjacency[v] |= 1 << u;
        }

        Self { nodes, adjacency }
    }

    pub fn iter_subgraph(&self) -> impl Iterator<Item = usize> {
        let mut subgraph: usize = 0;
        let max_state: usize = 1 << self.adjacency.len();
        return std::iter::from_fn(move || {
            if subgraph >= max_state {
                return None;
            }
            let result = subgraph.clone();
            subgraph += 1;
            return Some(result);
        });
    }

    pub fn print_ascii_graph(&self, size: usize) {
        let n = self.nodes.len();
        let radius = size as f64 / 2.0;
        let center = (radius, radius);

        let mut grid = vec![vec![' '; size + 1]; size + 1];

        let mut positions = vec![];
        for i in 0..n {
            let angle = 2.0 * PI * (i as f64) / (n as f64);
            let x = (center.0 + radius * angle.cos()).round() as usize;
            let y = (center.1 + radius * angle.sin()).round() as usize;
            positions.push((x, y));
            grid[y][x] = char::from_digit(self.nodes[i] as u32, 36).unwrap_or('?');
        }

        for (i, &adj) in self.adjacency.iter().enumerate() {
            let (x1, y1) = positions[i];

            for j in 0..n {
                if (adj >> j) & 1 == 1 {
                    let (x2, y2) = positions[j];
                    let mut x = x1 as isize;
                    let mut y = y1 as isize;
                    let dx = x2 as isize - x1 as isize;
                    let dy = y2 as isize - y1 as isize;

                    let step_x = if dx > 0 { 1 } else { -1 };
                    let step_y = if dy > 0 { 1 } else { -1 };

                    let dx = dx.abs();
                    let dy = dy.abs();

                    if dx > dy {
                        let mut error = dx / 2;
                        for _ in 0..dx {
                            x += step_x;
                            error += dy;
                            if error >= dx {
                                y += step_y;
                                error -= dx;
                            }
                            if (x, y) != (x1 as isize, y1 as isize)
                                && (x, y) != (x2 as isize, y2 as isize)
                            {
                                grid[y as usize][x as usize] = '.';
                            }
                        }
                    } else {
                        let mut error = dy / 2;
                        for _ in 0..dy {
                            y += step_y;
                            error += dx;
                            if error >= dy {
                                x += step_x;
                                error -= dy;
                            }
                            if (x, y) != (x1 as isize, y1 as isize)
                                && (x, y) != (x2 as isize, y2 as isize)
                            {
                                grid[y as usize][x as usize] = '.';
                            }
                        }
                    }
                }
            }
        }

        for row in grid {
            println!("{}", row.iter().collect::<String>());
        }
    }
}
