use crate::graph::Graph;

pub struct Coloration<'a> {
    graph: &'a Graph,
    colors: Vec<usize>,
}

impl<'a> Coloration<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        Self {
            colors: vec![0, graph.nodes.len()],
            graph,
        }
    }

    pub fn get_color(self, node: usize) -> Option<usize> {
        let mask: usize = 1 << node;
        self.colors.iter().enumerate().find_map(
            |(idx, color)| {
                if color & mask != 0 {
                    Some(idx)
                } else {
                    None
                }
            },
        )
    }

    pub fn set_color(self, node: usize, color: usize) -> Result<usize, String> {
        let mut state = true;
        let mut nodes: Vec<usize> = Vec::new();
        let n = self.graph.adjacency.len();
        for i in 0..n {
            if (self.graph.adjacency[node] >> i) & 1 == 1 {
                nodes.push(i);
            }
        }
        
        Ok(())
    }
}
