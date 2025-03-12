use std::time::Instant;

mod coloring;
mod examples;
mod graph;

fn main() {
    let graph: graph::Graph = examples::load_30();
    // graph.draw_ascii(100, 50);

    let start = Instant::now();
    let chromatic_num = graph.find_chromatic_num_bt_iter();
    let elapse = Instant::elapsed(&start);

    println!("Numero cromático: {} en {:?}", chromatic_num, elapse);
}
