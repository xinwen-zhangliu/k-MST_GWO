use float_cmp::approx_eq;
use libm::{pow, sqrt};

#[derive(Debug, Clone, Copy)]
///Struct that saves the x,y and index assignment of a vertex
pub struct Vertex(pub f64, pub f64, pub usize);

impl Vertex {
    /// Constructor that initializes all values in zeros
    pub fn default() -> Self {
        Vertex(0.0, 0.0, 0)
    }

    /// Equals function that checks if two vetices are equal using float partial comp
    pub fn equals(v1: &Vertex, v2: &Vertex) -> bool {
        approx_eq!(f64, v1.0, v2.0, ulps = 4) && approx_eq!(f64, v1.1, v2.1, ulps = 4)
        //v1.2 == v2.2
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    u: Vertex,
    v: Vertex,
    w: f64,
}

impl Edge {
    pub fn new(u: Vertex, v: Vertex, w: f64) -> Self {
        Edge { u, v, w }
    }

    pub fn default() -> Self {
        Edge {
            u: Vertex(0.0, 0.0, 0),
            v: Vertex(0.0, 0.0, 0),
            w: 0.0,
        }
    }

    pub fn equals(e1: Edge, e2: Edge) -> bool {
        approx_eq!(f64, e1.w, e2.w, ulps = 4)
            && (Vertex::equals(&e1.u, &e2.u) || Vertex::equals(&e1.u, &e2.v))
            && (Vertex::equals(&e1.v, &e2.u) || Vertex::equals(&e1.v, &e2.v))
    }

    pub fn get_weight(&self) -> f64 {
        self.w
    }
}

#[derive(Debug, Clone)]
pub struct Tree {
    k: usize,
    edges: Vec<Edge>,
    vertices: Vec<Vertex>,
    weight: f64,
    parent: Vec<usize>,
    rank: Vec<usize>,
    mst : Vec<Edge>
}

impl Tree {
    pub fn new(vertices: &Vec<Vertex>, k: usize) -> Self {
        Tree {
            k,
            vertices: vertices.clone(),
            edges: vec![Edge::default(); k * (k - 1) / 2],
            weight: 0.0,
            parent: vec![k; k],
            rank: vec![1; k],
            mst : vec![Edge::default(); k-1]
        }
    }

    pub fn default() -> Self {
        Tree {
            k: 0,
            vertices: Vec::new(),
            edges: Vec::new(),
            weight: 0.0,
            parent: Vec::new(),
            rank: Vec::new(),
            mst: Vec::new()
        }
    }

    fn assign_indexes(&mut self) {
        for i in 0..self.k {
            self.vertices[i].2 = i;
        }
    }

    pub fn get_num_edges(&self) -> usize {
        self.edges.len()
    }

    fn generate_graph(&mut self) {
        let mut counter = 0;
        for i in 0..self.k {
            for j in (i + 1)..self.k {
                let u = self.vertices[i].clone();
                let v = self.vertices[j].clone();
                self.edges[counter] = Edge::new(
                    u,
                    v,
                    Tree::get_distance(&self.vertices[i], &self.vertices[j]),
                );
                counter += 1;
            }
        }
    }

    fn get_distance(p1: &Vertex, p2: &Vertex) -> f64 {
        sqrt(pow((p2.0 - p1.0) as f64, 2.0) + pow((p2.1 - p1.1) as f64, 2.0))
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == self.k {
            return i;
        }
        self.parent[i] = self.find(self.parent[i]);
        self.parent[i]
    }

    fn combine(&mut self, x: usize, y: usize) {
        let s1 = self.find(x);
        let s2 = self.find(y);

        if s1 != s2 {
            if self.rank[s1] < self.rank[s2] {
                self.parent[s1] = s2;
            } else if self.parent[s1] > self.parent[s2] {
                self.parent[s2] = s1;
            } else {
                self.parent[s2] = s1;
                self.rank[s1] += 1;
            }
        }
    }

    // fn overwrite_edge(&mut self, edge : Edge) {
    //     self.edges[self.edges.len()-1] = edge;
    // }

    pub fn get_mst(&mut self) -> Vec<Edge> {
        self.assign_indexes();
        self.generate_graph();

        let mut mst: Vec<Edge> = Vec::new();

        self.edges.sort_by(|a, b| a.w.partial_cmp(&b.w).unwrap());

        let mut weight = 0.0;
        for i in 0..self.edges.len() {
            let w = self.edges[i].w;
            let x = self.edges[i].u.2;
            let y = self.edges[i].v.2;

            if self.find(x) != self.find(y) {
                self.combine(x, y);
                weight += w;
                mst.push(Edge::new(
                    self.edges[i].u.clone(),
                    self.edges[i].v.clone(),
                    self.edges[i].w,
                ));
            }
        }
        self.weight = weight;
        self.mst = mst.clone();
        mst
    }

    pub fn get_weight(&self) -> f64 {
        self.weight
    }

    pub fn get_mst_edges(&self) -> &Vec<Edge> {
        &self.mst
    }
    
    pub fn set_vertices(&mut self, vertices: &Vec<Vertex>) {
        self.vertices = vertices.clone();
    }

    pub fn overwrite_vertex(&mut self, index : usize, vertex : Vertex){
        self.vertices[index] = vertex
    }
    
    pub fn overwrite_edge(&mut self, index : usize, edge : Edge) {
        self.mst[index] = edge;
    }

}
