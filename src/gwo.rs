use crate::tree::Edge;
use crate::tree::Tree;
use crate::tree::Vertex;
use libm::{pow, sqrt};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rand_distr::Uniform;

pub struct GWO {
    k: usize,
    population: usize,
    vertices: Vec<Vertex>,
    r: StdRng,
}

type Type = Uniform<f64>;

#[derive(Debug, Clone)]
struct Wolf {
    solution: Vec<Edge>,
    vertices: Vec<Vertex>,
    fitness: f64,
    position: Vertex,
}

impl GWO {
    pub fn new(population: usize, vertices: Vec<Vertex>, k: usize, seed: u64) -> Self {
        GWO {
            population,
            vertices,
            k,
            r: StdRng::seed_from_u64(seed),
        }
    }

    fn evolve(&mut self, a: f64, positions: &mut [Wolf]) {
        // let mut alpha = vec![Vertex(0.0, 0.0, 0); self.k];
        // let mut beta = vec![Vertex(0.0, 0.0, 0); self.k];
        // let mut delta = vec![Vertex(0.0, 0.0, 0); self.k];

        let mut r1 = StdRng::seed_from_u64(1);
        let mut r2 = StdRng::seed_from_u64(2);

        let uniform = Uniform::new(0.0, 1.0);
        let mut iter = 0;

        //        let assign_A = move |a : f64, mut r1 :  StdRng, uniform : Type| {2.0 * a * r1.sample(uniform) - a};
        let mut A: Vec<f64> = vec![0.0f64; 3];
        for i in 0..3 {
            A[i] = 2.0 * a * r1.sample(uniform) - a;
        }

        //let assign_C = | r2 : StdRng, uniform : Type| {2.0 * r2.sample(uniform) };
        let mut C: Vec<f64> = vec![0.0f64; 3];
        for i in 0..3 {
            C[i] = 2.0 * r2.sample(uniform);
        }

        let mut X: Vec<f64> = vec![0.0f64; 3];

        // let mut X : Vec<Wolf> = vec![Wolf{
        //     solution : Tree::default(),
        //     fitness : 0.0,
        //     position : 0.0
        // }; 3];

        for i in 0..3 {
            X[i] = positions[0].fitness + A[i] * (positions[1].fitness - positions[2].fitness);
            //let vertex = self.vertices[];
        }

        let mut index: usize = self.r.gen::<usize>() % self.k;
        let mut v_index: usize = self.r.gen::<usize>() % self.vertices.len();
        let mut u_index: usize = self.r.gen::<usize>() % self.vertices.len();

        let distance = |p1: &Vertex, p2: &Vertex| {
            sqrt(pow((p2.0 - p1.0) as f64, 2.0) + pow((p2.1 - p1.1) as f64, 2.0))
        };

        // for each solution remove a random edge and swap a random edge until the weight
        // of the minimum spanning tree is less thant he previous one

        for i in 0..positions.len() {
            let previous_weight = positions[i].fitness;
            let mut new_weight = f64::INFINITY;
            

            while new_weight > previous_weight {
                

                let mut previous_vertex = positions[i].vertices[index];
                let mut new_vertex = self.vertices[v_index];

                while Vertex::equals(&previous_vertex, &new_vertex) {
                    index = self.r.gen::<usize>() % self.k;
                    v_index = self.r.gen::<usize>() % self.vertices.len();

                    previous_vertex = positions[i].vertices[index];
                    new_vertex = self.vertices[v_index];
                }

                //swap the index
                positions[i].vertices[index] = new_vertex;
                
                let mut new_tree = Tree::new(&positions[i].vertices, self.k);
                let new_solution = new_tree.get_mst();

                if new_tree.get_weight() < previous_weight {
                    positions[i].solution = new_solution;
                    positions[i].fitness = new_tree.get_weight();
                }else {
                    positions[i].vertices[index] = previous_vertex;
                }
            }
        }

        

        // if A < 1 then wolf attacks
        //else get another prey

        // for i in 0..3{

        //     while A[i] >= 1.0{
        //         index = self.r.gen::<usize>() % self.k;
        //         u_index = self.r.gen::<usize>() % self.vertices.len();
        //         v_index = self.r.gen::<usize>() % self.vertices.len();
        //         A[i] = 2.0 * a * r1.sample(uniform) - a;
        //     }

        //     positions[i].solution.remove(index);
        //     let weight : f64 = distance(&self.vertices[u_index], &self.vertices[v_index]);
        //     let edge = Edge::new(self.vertices[u_index], self.vertices[v_index], weight);

        //     let mut new_edges : Vec<Edge> = vec![Edge::default(); self.k-1];
        //     for j in 0..positions[i].solution.len(){
        //         if positions[i].solution[j].get_weight() > weight {
        //             new_edges.push(edge);
        //         }else {
        //             new_edges.push(positions[i].solution[j].clone());
        //         }
        //     }
        // }

        // let D_1 = num::abs(C1 * position[0[.x - wol );

        // let mut r = StdRng::seed_from_u64(222);
        // let normal = Uniform::from(0..self.vertices.len());
        // r.gen_range(0.0, 1.0);

        // for i in 0..self.vertices.len(){

        // }
        // let v1 = normal.sample(&mut r); // <- Here we use the generator
        // let v2 = normal.sample(&mut r);

        // let ru = Uniform::new(0.0, 1.0);
        // let r1  : f64 = ru.sample(&mut ru) ;

        // let a1 = 2.0 * a * r1-a;
        // let c1 = 2.0 * r2;

        // let a = 2 - 2 * epoch / (epoch - 1);
        //_, list_best, _ = self.get_special_solutions(self.pop, best=3)
    }

    fn binary_search(edges: Vec<Edge>) {}

    fn get_float(&self) -> f64 {
        0.0
    }

    /// Function that runs the gwo heuristic.
    ///
    // pub fn run_gwo(&mut self, num_iter: usize) {
    //     let mut solutions: Vec<Wolf> = vec![
    //         Wolf {
    //             solution: Tree::default(),
    //             fitness: 0.0,
    //             position: Vertex::default()
    //         };
    //         self.population
    //     ];

    //     // generation of random positions
    //     for i in 0..self.population {
    //         //solutions.push(Tree::new(&self.generate_solution(), self.k));
    //         solutions[i]
    //             .solution
    //             .set_vertices(&self.generate_solution());
    //         solutions[i].solution.get_mst();
    //         // println!("{:?} : {} ", &positions[i].get_mst(), &positions[i].get_weight());
    //     }

    //     // sort the solutions based on
    //     solutions.sort_by(|a, b| {
    //         a.solution
    //             .get_weight()
    //             .partial_cmp(&b.solution.get_weight())
    //             .unwrap()
    //     });

    //     // Sort the solutions from best to worst

    //     let mut counter = 0;
    //     while counter < num_iter {
    //         counter += 1;
    //     }
    // }

    fn generate_solution(&mut self) -> Vec<Vertex> {
        let mut new_vertices: Vec<Vertex> = vec![Vertex(0.0, 0.0, 0); self.k];
        let mut vertices = self.vertices.clone();

        for i in 0..self.k {
            let index: usize = self.r.gen::<usize>() % vertices.len();
            new_vertices[i] = vertices[index];
            vertices.remove(index);
        }
        new_vertices
    }
}
