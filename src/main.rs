use graph::Graph;

mod graph;

fn hex_pattern() -> Graph {
    let mut g = Graph::new(6);
    let a = g.add_vert();
    let b = g.add_vert();
    let c = g.add_vert();
    let d = g.add_vert();
    g.set_edge(a, 0, b);
    g.set_edge(b, 1, a);
    g.set_edge(c, 0, d);
    g.set_edge(d, 1, c);
    g.set_edge(a, 2, c);
    g.set_edge(c, 3, a);
    g.set_edge(b, 2, d);
    g.set_edge(d, 3, b);
    g.set_edge(b, 4, c);
    g.set_edge(c, 5, b);
    g
}

fn main() {
    let g = hex_pattern();
    g.print_graphviz_graph();
}
