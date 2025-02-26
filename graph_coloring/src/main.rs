mod examples;
mod graph;

fn main() {
    let graph: graph::Graph = examples::load_5();
    let n = graph.nodes.len();
    let mut coloration: Vec<usize> = vec![0; n];
    let mut solutions: Vec<String> = Vec::new();

    for i in 0..n {
        backtrack_coloring(&graph, n, i as usize, &mut coloration, &mut solutions);
    }
    println!("{:?}", solutions);
}

fn backtrack_coloring(
    graph: &graph::Graph,
    n: usize,
    coloring_node: usize,
    coloration: &mut Vec<usize>,
    solutions: &mut Vec<String>,
) {
    if coloration.iter().all(|node| *node != 0) {
        solutions.push(canonic_coloration(coloration));
        println!("{}", canonic_coloration(coloration))
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
        backtrack_coloring(graph, n, node, coloration, solutions);
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
    colors.iter().for_each(|nodes| {
        nodes
            .iter()
            .for_each(|node| result.push_str(&format!("{}", node)));
    });
    return result;
}
