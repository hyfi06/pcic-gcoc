use std::time::Instant;

mod examples;
mod graph;

fn main() {
    let graph: graph::Graph = examples::load_30();
    graph.draw_ascii(100, 50);
    let n = graph.nodes.len();
    let mut chromatic_num = n.clone();
    let mut coloration: Vec<usize> = vec![0; n];
    let mut solutions: Vec<String> = Vec::new();
    let start = Instant::now();
    backtrack_coloring(
        &graph,
        n,
        0,
        &mut coloration,
        &mut solutions,
        &mut chromatic_num,
    );
    let elapse = Instant::elapsed(&start);

    println!("Numero cromático: {} en {:?}", chromatic_num, elapse);
}

fn backtrack_coloring(
    graph: &graph::Graph,
    n: usize,
    coloring_node: usize,
    coloration: &mut Vec<usize>,
    solutions: &mut Vec<String>,
    chromatic_num: &mut usize,
) {
    let num_colors: usize = coloration
        .iter()
        .fold(0, |acc, cur| acc | *cur)
        .count_ones()
        .try_into()
        .unwrap_or(0);

    if num_colors >= *chromatic_num {
        return;
    }

    if coloring_node == n {
        if num_colors < *chromatic_num {
            let canonic = canonic_coloration(coloration);
            println!("{}", canonic);
            println!("Numero cromático: {num_colors}");
            *chromatic_num = num_colors;
            solutions.push(canonic);
        }
        return;
    }

    let neighbors_colors: Vec<usize> = (0..coloring_node)
        .filter_map(|idx| {
            if graph.adjacency[coloring_node] >> idx & 1 == 1 && coloration[idx] != 0 {
                Some(coloration[idx].trailing_zeros() as usize)
            } else {
                None
            }
        })
        .collect();

    let colors: Vec<usize> = (0..(num_colors + 1).min(coloration.len()))
        .filter(|idx| !neighbors_colors.contains(&idx))
        .collect();

    for color in colors {
        coloration[coloring_node] = 1 << color;
        backtrack_coloring(
            graph,
            n,
            coloring_node + 1,
            coloration,
            solutions,
            chromatic_num,
        );
    }
    coloration[coloring_node] = 0;
}

fn canonic_coloration(coloration: &mut Vec<usize>) -> String {
    let n = coloration.len();
    let mut colors: Vec<Vec<usize>> = vec![Vec::new(); n];

    coloration.iter().enumerate().for_each(|(node, color)| {
        if *color != 0 {
            colors[color.trailing_zeros() as usize].push(node)
        }
    });

    colors.sort();

    colors
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
        .join(" ")
}
