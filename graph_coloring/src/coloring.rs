#![allow(dead_code)]
use crate::graph::Graph;
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
        (0..self.vertex_color.len()).into_iter().fold(0, |acc, v| {
            if self.vertex_color[v] ^ 1 << color == 0 {
                acc | 1 << v
            } else {
                acc
            }
        })
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

#[derive(Debug)]
struct LocalState {
    curr_node: usize,
    curr_color: usize,
    last_color: usize,
    max_color: usize,
}

impl LocalState {
    fn from_tuple(tuple: (usize, usize, usize, usize)) -> Self {
        LocalState {
            curr_node: tuple.0,
            curr_color: tuple.1,
            last_color: tuple.2,
            max_color: tuple.3,
        }
    }
}

impl std::fmt::Display for LocalState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "cn:{} cc:{} mc:{} lc: {}",
            self.curr_node, self.curr_color, self.max_color, self.last_color
        )
    }
}

impl Graph {
    pub fn find_chromatic_num_bt_iter(&self) -> usize {
        let n = self.adjacency.len();
        let mut coloration = ChromaticColoration::new(n);
        let mut chromatic_num = n;
        let mut visited_states = 0;
        let mut stack: Vec<LocalState> = vec![LocalState::from_tuple((0, 0, 0, 1))];

        while let Some(local_state) = stack.pop() {
            visited_states += 1;
            coloration.del_node_color(local_state.curr_node, local_state.last_color);
            coloration.set_node_color(local_state.curr_node, local_state.curr_color);
            let num_colors = coloration.count_colors();

            if local_state.max_color == 0 {
                coloration.del_node_color(local_state.curr_node, local_state.curr_color);
            } else if local_state.curr_node == n - 1 {
                if num_colors < chromatic_num {
                    println!("{}", coloration);
                    println!("Numero cromático: {num_colors}");
                    chromatic_num = num_colors;
                }
                coloration.del_node_color(local_state.curr_node, local_state.curr_color);
            } else {
                if let Some(curr_node_next_color) = (local_state.curr_color + 1
                    ..(local_state.max_color).min(chromatic_num - 1))
                    .find(|posible_color| {
                        coloration.get_chromatic_class(*posible_color)
                            & self.adjacency[local_state.curr_node]
                            == 0
                    })
                {
                    stack.push(LocalState::from_tuple((
                        local_state.curr_node,
                        curr_node_next_color,
                        local_state.curr_color,
                        local_state.max_color,
                    )));
                } else {
                    stack.push(LocalState::from_tuple((
                        local_state.curr_node,
                        local_state.curr_color,
                        local_state.curr_color,
                        0,
                    )));
                }

                if let Some(next_node_first_color) = (0..(num_colors + 1).min(chromatic_num - 1))
                    .find(|posible_color| {
                        coloration.get_chromatic_class(*posible_color)
                            & self.adjacency[local_state.curr_node + 1]
                            == 0
                    })
                {
                    stack.push(LocalState::from_tuple((
                        local_state.curr_node + 1,
                        next_node_first_color,
                        next_node_first_color,
                        num_colors + 1,
                    )));
                } else {
                    coloration.del_node_color(local_state.curr_node, local_state.curr_color);
                }
            }

            if false {
                println!("State {} {}", visited_states, local_state);
                println!("{:?}", stack);
                println!("{}", coloration);
                println!("Chromatic {}, Colors {}", chromatic_num, num_colors);
                println!("");
            }
        }
        println!("Num visited states:{}", visited_states);
        chromatic_num
    }

    pub fn find_chromatic_num_bt_iter_v(&self) -> usize {
        let n = self.adjacency.len();
        let mut coloration = VertexColoring::new(n);
        let mut chromatic_num = n;
        let mut visited_states = 0;
        let mut stack: Vec<LocalState> = vec![LocalState::from_tuple((0, 0, 0, 1))];

        while let Some(local_state) = stack.pop() {
            visited_states += 1;
            for node in local_state.curr_node..n {
                coloration.del_node_color(node, 0);
            }
            coloration.set_node_color(local_state.curr_node, local_state.curr_color);
            let num_colors = coloration.count_colors();

            if local_state.curr_node == n - 1 {
                if num_colors < chromatic_num {
                    println!("{}", coloration);
                    println!("Numero cromático: {num_colors}");
                    chromatic_num = num_colors;
                }
            } else {
                if let Some(curr_node_next_color) = (local_state.curr_color + 1
                    ..(local_state.max_color).min(chromatic_num - 1))
                    .find(|posible_color| {
                        coloration.get_chromatic_class(*posible_color)
                            & self.adjacency[local_state.curr_node]
                            == 0
                    })
                {
                    stack.push(LocalState::from_tuple((
                        local_state.curr_node,
                        curr_node_next_color,
                        local_state.curr_color,
                        local_state.max_color,
                    )));
                }
                if let Some(next_node_first_color) = (0..(num_colors + 1).min(chromatic_num - 1))
                    .find(|posible_color| {
                        coloration.get_chromatic_class(*posible_color)
                            & self.adjacency[local_state.curr_node + 1]
                            == 0
                    })
                {
                    stack.push(LocalState::from_tuple((
                        local_state.curr_node + 1,
                        next_node_first_color,
                        next_node_first_color,
                        num_colors + 1,
                    )));
                }
            }

            if false {
                println!("State {} {}", visited_states, local_state);
                println!("{:?}", stack);
                println!("{}", coloration);
                println!("Chromatic {}, Colors {}", chromatic_num, num_colors);
                println!("");
            }
        }
        println!("Num visited states:{}", visited_states);
        chromatic_num
    }
}
