mod examples;
mod graph;

fn main() {
    let x: usize = 0b00100;
    println!("{}", x.trailing_zeros())
    // let graph: graph::Graph = examples::load_5();
    // let mut coloration: Vec<usize> = vec![0, graph.nodes.len()];
    // let mut solutions: Vec<String> = Vec::new();

    // backtrack_coloring(
    //     &graph,
    //     graph.nodes.len(),
    //     0,
    //     &mut coloration,
    //     &mut solutions,
    // );
}

fn backtrack_coloring(
    graph: &graph::Graph,
    n: usize,
    coloring_node: usize,
    coloration: &mut Vec<usize>,
    solutions: &mut Vec<String>,
) {
    if coloring_node == n {
        solutions.push(canonic_coloration(coloration));
    }
    let neighbors: Vec<usize> = (0..coloring_node)
        .filter(|idx| graph.adjacency[coloring_node] >> idx & 1 == 1)
        .collect();
    let colors: Vec<usize> = neighbors
        .iter()
        .map(|idx| coloration[*idx].trailing_zeros() as usize)
        .collect();
    let color = (0..n).find(|idx| !colors.contains(&idx)).unwrap();
    coloration[coloring_node] |= 1 << color;
    backtrack_coloring(graph, n, coloring_node + 1, coloration, solutions);
}

// fn canonic_coloration(coloration: &mut Vec<usize>) -> String {}
