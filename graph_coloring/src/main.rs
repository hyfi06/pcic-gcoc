use std::time::Instant;

mod examples;
mod graph;

fn main() {
    let graph: graph::Graph = examples::load_30();
    graph.print_ascii_graph(50);
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
    // println!("{:?}", solutions);
    println!("Numero cromático: {}. {:?}", chromatic_num, elapse);
}

fn greedy_coloring(
    graph: &graph::Graph,
    n: usize,
    coloring_node: usize,
    coloration: &mut Vec<usize>,
    solutions: &mut Vec<String>,
) {
    if coloration.iter().all(|node| *node != 0) {
        let canonic = canonic_coloration(coloration);
        if !solutions.contains(&canonic) {
            println!("{}", canonic);
            solutions.push(canonic);
        }
    }
    let neighbors: Vec<usize> = (0..n)
        .filter(|idx| graph.adjacency[coloring_node] >> idx & 1 == 1)
        .collect();
    let neighbors_colors: Vec<usize> = neighbors
        .iter()
        .filter(|&idx| coloration[*idx] != 0)
        .map(|idx| coloration[*idx].trailing_zeros() as usize)
        .collect();

    let color = (0..n).find(|idx| !neighbors_colors.contains(&idx)).unwrap();
    coloration[coloring_node] = 1 << color;
    let next_nodos: Vec<usize> = neighbors
        .into_iter()
        .filter(|&node| coloration[node] == 0)
        .collect();
    for node in next_nodos {
        greedy_coloring(graph, n, node, coloration, solutions);
    }
}

fn backtrack_coloring(
    graph: &graph::Graph,
    n: usize,
    coloring_node: usize,
    coloration: &mut Vec<usize>,
    solutions: &mut Vec<String>,
    chromatic_num: &mut usize,
) {
    let canonic = canonic_coloration(coloration);
    let num_colors = canonic.split(' ').collect::<Vec<&str>>().len();
    if num_colors > *chromatic_num {
        return;
    }
    if coloring_node == n {
        if !solutions.contains(&canonic) {
            println!("{}", canonic);
            solutions.push(canonic);
        }
        if num_colors < *chromatic_num {
            println!("Numero cromático: {num_colors}");
            *chromatic_num = num_colors;
        }
        return;
    }

    let neighbors: Vec<usize> = (0..n)
        .filter(|idx| graph.adjacency[coloring_node] >> idx & 1 == 1)
        .collect();

    let neighbors_colors: Vec<usize> = neighbors
        .iter()
        .filter(|&idx| coloration[*idx] != 0)
        .map(|idx| coloration[*idx].trailing_zeros() as usize)
        .collect();

    let colors: Vec<usize> = (0..num_colors + 1)
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
        coloration[coloring_node] = 0;
    }
}

fn canonic_coloration(coloration: &mut Vec<usize>) -> String {
    let n = coloration.len();
    let mut colors: Vec<Vec<usize>> = vec![Vec::new(); n];
    coloration.iter().enumerate().for_each(|(node, color)| {
        if coloration[node] != 0 {
            colors[color.trailing_zeros() as usize].push(node)
        }
    });
    colors.sort();
    let mut result = String::new();
    colors
        .iter()
        .filter(|nodes| nodes.len() != 0)
        .for_each(|nodes| {
            nodes
                .iter()
                .for_each(|node| result.push_str(&format!("{}", node)));
            result.push(' ');
        });
    return result.trim().to_string();
}
