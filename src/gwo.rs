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
    solution: Tree,
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

    fn evolve(&mut self, a: f64, positions: &mut Vec<Wolf>) {
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

        for i in 0..3 {
            X[i] = positions[0].fitness + A[i] * (positions[1].fitness - positions[2].fitness);
        }


        for i in 3..positions.len(){
            
        }

        let mut index: usize = self.r.gen::<usize>() % self.k;
        let mut v_index: usize = self.r.gen::<usize>() % self.vertices.len();
        //let mut u_index: usize = self.r.gen::<usize>() % self.vertices.len();

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
                    positions[i].solution = new_tree;
                    positions[i].fitness = new_tree.get_weight();
                }else {
                    positions[i].vertices[index] = previous_vertex;
                }
            }
        }

        

     
    }

    fn binary_search(edges: Vec<Edge>) {}

    fn get_float(&self) -> f64 {
        0.0
    }

    /// Function that runs the gwo heuristic.
    ///
    pub fn run_gwo(&mut self, num_iter: usize, phi : f64) {

        self.assign_vertices();

        //initializing the pack and its values
        let mut pack : Vec<Wolf> = vec![
            Wolf {
                solution: Tree::default(),
                vertices : vec![Vertex::default(); self.k],
                fitness: 0.0,
                position : Vertex::default()
            };
            self.population
        ];

        for i in 0..self.population{
            let vertices = self.generate_solution();
            pack[i].solution = Tree::new(&vertices, self.k);
            pack[i].vertices = vertices;
            pack[i].position = vertices[i];

            pack[i].solution.get_mst();
            pack[i].fitness = pack[i].solution.get_weight();
        }

         // sort the solutions based on fitness
        pack.sort_by(|a, b| {
            a.fitness
                .partial_cmp(&b.fitness)
                .unwrap()
        });
        

        let a = 2.0;
        let i = 0;
        while a > 0.0 && num_iter < i {
            let previous_alpha_fitness = pack[0].fitness;


            self.evolve(a, &mut pack);

            let new_alpha_fitness = pack[0].fitness;

            if new_alpha_fitness < previous_alpha_fitness {
                a *= phi;
            }
            i+=1;
        }
        
        
        
        
        // generation of random positions
        // for i in 0..self.population {
        //     //solutions.push(Tree::new(&self.generate_solution(), self.k));
        //     pack[i]
        //         .solution
        //         .set_vertices(&self.generate_solution());
        //     solutions[i].solution.get_mst();
        //     // println!("{:?} : {} ", &positions[i].get_mst(), &positions[i].get_weight());
        // }

     
    }

    fn assign_vertices(&mut self) {
        for i in 0..self.vertices.len() {
            self.vertices[i].2 = i;
        }
    }

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
