mod square_grid;
fn main() {
    let sqg = square_grid::SqGrid::new();
    sqg.draw_ascii(80, 50);
    println!("{:?}", sqg.edges);
    for adj in sqg.adjacency.iter() {
        println!("{:12b}",adj.as_value());
    }
}
