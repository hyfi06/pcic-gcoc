use std::f64::consts::PI;

use bitmaps::Bitmap;

// macro_rules! square_grid_by_n_m {
//     ($struct_name:ident, $m:expr, $n:expr) => {

//     };
// }

const M: usize = 4;
const N: usize = 3;
const SIZE: usize = M * N;

#[derive(Debug, Clone)]
pub struct SqGrid {
    pub labels: Vec<usize>,
    pub adjacency: Vec<Bitmap<{ SIZE }>>,
    pub edges: Vec<(usize, usize)>,
}

impl SqGrid {
    pub fn new() -> Self {
        let size: usize = SIZE;
        let labels: Vec<usize> = (0..size).collect();
        let mut adjacency: Vec<Bitmap<{ SIZE }>> = Vec::new();
        let mut edges = Vec::new();

        for _ in 0..size {
            adjacency.push(Bitmap::<{ SIZE }>::new())
        }

        for i in 0..size {
            let residue = i % N;

            if residue != N - 1 {
                edges.push((i, i + 1));
                adjacency[i].set(i + 1, true);
                adjacency[i + 1].set(i, true);
            }
            if i < (M - 1) * N {
                edges.push((i, i + N));
                adjacency[i].set(i + N, true);
                adjacency[i + N].set(i, true);
            }
        }
        Self {
            labels,
            adjacency,
            edges,
        }
    }
    pub fn draw_ascii(&self, width: usize, height: usize) {
        let n = self.adjacency.len();
        let size = 100;
        let radius = size as f64 / 2.0 - 1.0;
        let center = (radius, radius);
        let mut positions: Vec<(usize, usize)> = vec![];
        for i in 0..n {
            let angle = 2.0 * PI * (i as f64) / (n as f64);
            let x = (center.0 + radius * angle.cos()).round() as usize;
            let y = (center.1 + radius * angle.sin()).round() as usize;
            positions.push((x, y));
        }

        // Mapear coordenadas al rango discreto de la cuadrícula
        let map_to_grid = |x: usize, y: usize| {
            let grid_x = (x as f32 / size as f32 * (width - 1) as f32).round() as usize;
            let grid_y = (y as f32 / size as f32 * (height - 1) as f32).round() as usize;
            (grid_x, grid_y)
        };

        let mut grid: Vec<Vec<char>> = vec![vec![' '; width]; height];

        for (i, &adj) in self.adjacency.iter().enumerate() {
            let (x1, y1) = map_to_grid(positions[i].0, positions[i].1);

            for j in 0..n {
                if adj.get(j) {
                    let (x2, y2) = map_to_grid(positions[j].0, positions[j].1);
                    let (dx, dy) = (x2 as isize - x1 as isize, y2 as isize - y1 as isize);
                    let steps = dx.abs().max(dy.abs());
                    let mut last_x = x1 as isize;
                    let mut last_y = y1 as isize;
                    for step in 1..steps {
                        let x = x1 as isize + step * dx / steps;
                        let y = y1 as isize + step * dy / steps;
                        if x >= 0
                            && y >= 0
                            && (y as usize) < height
                            && (x as usize) < width
                            && (dx.abs() == steps && last_y != y
                                || dy.abs() == 0
                                || dy.abs() == steps && last_x != x
                                || dx.abs() == 0)
                        {
                            grid[y as usize][x as usize] = '.';
                            last_x = x;
                            last_y = y;
                        }
                    }
                }
            }
        }
        for (i, position) in positions.iter().enumerate() {
            let (x, y) = map_to_grid(position.0, position.1);
            grid[y][x] = char::from_digit(self.labels[i] as u32, 36).unwrap_or('?');
        }

        for row in grid {
            println!("{}", row.iter().collect::<String>());
        }
    }
}
