use std::time::Instant;

mod examples;
mod graph;

fn main() {
    let graph: graph::Graph = examples::load_30();
    graph.draw_ascii(100, 50);
    let n = graph.nodes.len();
    let mut chromatic_num = n.clone();
    let mut coloration: Vec<usize> = vec![0; n];

    let start = Instant::now();
    backtrack_coloring(&graph, 0, &mut coloration, &mut chromatic_num);
    let elapse = Instant::elapsed(&start);

    println!("Numero cromático: {} en {:?}", chromatic_num, elapse);
}

fn backtrack_coloring(
    graph: &graph::Graph,
    coloring_node: usize,
    coloration: &mut Vec<usize>,
    chromatic_num: &mut usize,
) {
    let num_colors = coloration.iter().filter(|&color| *color != 0).count();
    if num_colors > *chromatic_num {
        return;
    }

    if coloring_node == graph.adjacency.len() {
        if num_colors < *chromatic_num {
            println!("{}", canonic_coloration(coloration));
            println!("Numero cromático: {num_colors}");
            *chromatic_num = num_colors;
        }
        return;
    }

    let colors: Vec<usize> = (0..num_colors + 1)
        .filter(|color| coloration[*color] & graph.adjacency[coloring_node] == 0)
        .collect();

    for color in colors {
        if color >= coloration.len() {
            continue;
        }
        coloration[color] |= 1 << coloring_node;
        backtrack_coloring(graph, coloring_node + 1, coloration, chromatic_num);
        coloration[color] ^= 1 << coloring_node;
    }
}

fn canonic_coloration(coloration: &mut Vec<usize>) -> String {
    let n = coloration
        .iter()
        .fold(0 as usize, |acc, cur| acc | *cur)
        .count_ones();

    coloration
        .iter()
        .filter(|&color| *color != 0)
        .map(|color| {
            (0..n)
                .filter(|node| (color >> *node) & 1 == 1)
                .map(|node| node.to_string())
                .collect::<Vec<_>>()
                .join(",")
        })
        .collect::<Vec<_>>()
        .join(" | ")
}
