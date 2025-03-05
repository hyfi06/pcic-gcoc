use std::fmt;

pub struct Coloration {
    colors: Vec<usize>,
}

impl Coloration {
    pub fn new(n: usize) -> Self {
        Self { colors: vec![0; n] }
    }

    pub fn count_colors(&self) -> usize {
        self.colors.iter().filter(|&color| *color != 0).count()
    }

    pub fn count_colored_nodes(&self) -> usize {
        self.colors
            .iter()
            .fold(0 as usize, |acc, cur| acc | *cur)
            .count_ones() as usize
    }

    pub fn set_node_color(&mut self, node: usize, color: usize) {
        self.colors[color] |= 1 << node;
    }

    pub fn del_node_color(&mut self, node: usize, color: usize) {
        if (self.colors[color] >> node) & 1 == 1 {
            self.colors[color] ^= 1 << node;
        }
    }

    pub fn get_color_node(&self, node: usize) -> Option<usize> {
        self.colors.iter().enumerate().find_map(|(idx, color)| {
            if (color >> node & 1) == 1 {
                Some(idx)
            } else {
                None
            }
        })
    }
    pub fn get_chromatic_class(&self, color: usize) -> usize {
        self.colors[color]
    }
}

impl fmt::Display for Coloration {
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
