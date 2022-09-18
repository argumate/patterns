pub struct Graph {
    num_edges_per_vert: usize,
    verts: Vec<Vec<Option<usize>>>,
}

impl Graph {
    pub fn new(num_edges_per_vert: usize) -> Self {
        Graph {
            num_edges_per_vert,
            verts: vec![],
        }
    }

    pub fn num_verts(&self) -> usize {
        self.verts.len()
    }

    pub fn add_vert(&mut self) -> usize {
        let v = self.num_verts();
        let mut new_edges = Vec::with_capacity(self.num_edges_per_vert);
        new_edges.resize(self.num_edges_per_vert, None);
        self.verts.push(new_edges);
        v
    }

    pub fn get_edge(&self, v: usize, e: usize) -> Option<usize> {
        self.verts[v][e]
    }

    pub fn set_edge(&mut self, v: usize, e: usize, x: usize) {
        assert_eq!(self.verts[v][e], None);
        self.verts[v][e] = Some(x);
    }

    pub fn print_graphviz_digraph(&self) {
        println!("digraph {{");
        for v in 0..self.verts.len() {
            for e in 0..self.verts[v].len() {
                if let Some(x) = self.verts[v][e] {
                    println!("{} -> {}", v, x);
                }
            }
        }
        println!("}}");
    }

    pub fn print_graphviz_graph(&self) {
        println!("graph {{");
        for v in 0..self.verts.len() {
            for e in 0..self.verts[v].len() {
                if let Some(x) = self.verts[v][e] {
                    if v < x {
                        println!("{} -- {}", v, x);
                    }
                }
            }
        }
        println!("}}");
    }
}
