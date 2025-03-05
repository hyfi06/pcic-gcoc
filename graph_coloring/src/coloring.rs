use std::fmt;

pub trait Coloration {
    fn new(size: usize) -> Self;
    fn count_colors(&self) -> usize;
    fn count_colored_nodes(&self) -> usize;
    fn set_node_color(&mut self, node: usize, color: usize);
    fn del_node_color(&mut self, node: usize, color: usize);
    fn get_color_node(&self, node: usize) -> Option<usize>;
    fn get_chromatic_class(&self, color: usize) -> usize;
}

pub struct ChromaticColoration {
    colors: Vec<usize>,
}

impl Coloration for ChromaticColoration {
    fn new(size: usize) -> Self {
        Self {
            colors: vec![0; size],
        }
    }

    fn count_colors(&self) -> usize {
        self.colors.iter().filter(|&color| *color != 0).count()
    }

    fn count_colored_nodes(&self) -> usize {
        self.colors
            .iter()
            .fold(0 as usize, |acc, cur| acc | *cur)
            .count_ones() as usize
    }

    fn set_node_color(&mut self, node: usize, color: usize) {
        self.colors[color] |= 1 << node;
    }

    fn del_node_color(&mut self, node: usize, color: usize) {
        if (self.colors[color] >> node) & 1 == 1 {
            self.colors[color] ^= 1 << node;
        }
    }

    fn get_color_node(&self, node: usize) -> Option<usize> {
        self.colors.iter().enumerate().find_map(|(idx, color)| {
            if (color >> node & 1) == 1 {
                Some(idx)
            } else {
                None
            }
        })
    }
    fn get_chromatic_class(&self, color: usize) -> usize {
        self.colors[color]
    }
}

impl fmt::Display for ChromaticColoration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let coloration = self
            .colors
            .iter()
            .filter(|&color| *color != 0)
            .map(|color| {
                (0..self.colors.len())
                    .filter(|node| (color >> *node) & 1 == 1)
                    .map(|node| node.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            })
            .collect::<Vec<_>>()
            .join(" | ");
        write!(f, "{}", coloration)
    }
}

pub struct VertexColoring {
    vertex_color: Vec<usize>,
}

impl Coloration for VertexColoring {
    fn new(size: usize) -> Self {
        VertexColoring {
            vertex_color: vec![0; size],
        }
    }

    fn count_colors(&self) -> usize {
        self.vertex_color
            .iter()
            .fold(0, |acc, cur| acc | *cur)
            .count_ones() as usize
    }

    fn count_colored_nodes(&self) -> usize {
        self.vertex_color.iter().filter(|&v| *v != 0).count()
    }

    fn set_node_color(&mut self, node: usize, color: usize) {
        self.vertex_color[node] = 1 << color;
    }

    fn del_node_color(&mut self, node: usize, _: usize) {
        self.vertex_color[node] = 0;
    }

    fn get_color_node(&self, node: usize) -> Option<usize> {
        if self.vertex_color[node] != 0 {
            Some(self.vertex_color[node].trailing_zeros() as usize)
        } else {
            None
        }
    }

    fn get_chromatic_class(&self, color: usize) -> usize {
        self.vertex_color
            .iter()
            .enumerate()
            .filter(|&(_, v)| v ^ 1 << color == 0)
            .fold(0, |acc, curr| acc | (1 << curr.0))
    }
}

impl fmt::Display for VertexColoring {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = self.vertex_color.len();
        let mut colors: Vec<Vec<usize>> = vec![Vec::new(); n];

        self.vertex_color
            .iter()
            .enumerate()
            .for_each(|(node, color)| {
                if *color != 0 {
                    colors[color.trailing_zeros() as usize].push(node)
                }
            });

        // colors.sort();

        let text = colors
            .into_iter()
            .filter(|color| color.len() != 0)
            .map(|color| {
                color
                    .iter()
                    .map(|idx| idx.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            })
            .collect::<Vec<_>>()
            .join(" | ");
        write!(f, "{}", text)
    }
}
