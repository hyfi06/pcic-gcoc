use std::time::Instant;

use coloring::Coloration;
mod coloring;
mod examples;
mod graph;

fn main() {
    let graph: graph::Graph = examples::load_30();
    // graph.draw_ascii(100, 50);

    let start = Instant::now();
    let chromatic_num = backtrack_coloring(&graph);
    let elapse = Instant::elapsed(&start);

    println!("Numero cromático: {} en {:?}", chromatic_num, elapse);
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

fn backtrack_coloring(graph: &graph::Graph) -> usize {
    let n = graph.adjacency.len();
    let mut coloration = Coloration::new(n);
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
                        & graph.adjacency[local_state.curr_node]
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

            if let Some(next_node_first_color) =
                (0..(num_colors + 1).min(chromatic_num - 1)).find(|posible_color| {
                    coloration.get_chromatic_class(*posible_color)
                        & graph.adjacency[local_state.curr_node + 1]
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

        if visited_states < 100 && 27 < visited_states {
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
