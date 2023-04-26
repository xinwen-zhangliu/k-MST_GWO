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
    r1: StdRng,
    r2: StdRng,
    r3: StdRng,
    r4: StdRng,
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
            r1: StdRng::seed_from_u64(seed + 1),
            r2: StdRng::seed_from_u64(seed + 2),
            r3: StdRng::seed_from_u64(seed + 3),
            r4: StdRng::seed_from_u64(seed + 4),
        }
    }

    fn evolve(&mut self, a: f64, positions: &mut Vec<Wolf>) {
        for i in (3..self.population).rev() {
            let uniform = Uniform::new(0.0, 1.0);

            let mut A: Vec<f64> = vec![0.0f64; 3];
            for i in 0..3 {
                A[i] = 2.0 * a * self.r1.sample(uniform) - a;
            }

            let mut C: Vec<f64> = vec![0.0f64; 3];
            for i in 0..3 {
                C[i] = 2.0 * self.r2.sample(uniform);
            }

            let mut X: Vec<f64> = vec![0.0f64; 3];

            let mut sum = 0.0;
            for i in 0..3 {
                let new_x = positions[i].position.2 as f64
                    + A[i]
                        * (C[i] * positions[i].position.2 as f64 - positions[i].position.2 as f64)
                            as f64;

                X[i] = new_x;
                sum += new_x;
            }

            let mut new_vertex = self.vertices
                [(sum * self.vertices.len() as f64 / 3.0).round() as usize % self.vertices.len()];
            let index =
                (sum * self.vertices.len() as f64 / 3.0).round() as usize % self.vertices.len();

            //println!("{:?}",index);

            let index = self.r.gen::<usize>() % self.k;

            while GWO::repeated(&positions[i].vertices, &new_vertex) {
                let n = self.r3.gen::<usize>() % self.vertices.len();
                new_vertex = self.vertices[n];
            }

            //println!("old{:?}", positions[i].vertices);

            positions[i].vertices[index] = new_vertex;
            //println!("new{:?}", positions[i].vertices);

            positions[i].solution = Tree::new(&positions[i].vertices, self.k);
            //            pack[i].vertices = vertices.clone();
            positions[i].position = new_vertex;

            positions[i].solution.get_mst();
            positions[i].fitness = positions[i].solution.get_weight();
        }
    }

    fn repeated(vertices: &Vec<Vertex>, vertex: &Vertex) -> bool {
        for i in 0..vertices.len() {
            if Vertex::equals(&vertices[i], &vertex) {
                // println!("{:?}\n {:?}", vertices, vertex);
                return true;
            }
        }
        false
    }

    fn binary_search(edges: Vec<Edge>) {}

    fn get_float(&self) -> f64 {
        0.0
    }

    /// Function that runs the gwo heuristic.
    pub fn run_gwo(&mut self, num_iter: usize, phi: f64) -> Tree {
        self.assign_vertices();

        //initializing the pack and its values
        let mut pack: Vec<Wolf> = vec![
            Wolf {
                solution: Tree::new(&self.vertices, self.k),
                vertices: vec![Vertex::default(); self.k],
                fitness: 0.0,
                position: Vertex::default()
            };
            self.population
        ];

        for i in 0..self.population {
            let vertices = self.generate_solution();
            pack[i].solution = Tree::new(&vertices, self.k);
            pack[i].vertices = vertices.clone();
            pack[i].position = vertices[i];

            pack[i].solution.get_mst();
            pack[i].fitness = pack[i].solution.get_weight();
            //dbg!(pack[i].fitness) ;
        }

        // sort the solutions based on fitness
        pack.sort_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap());
        //dbg!("f",pack[0].fitness);
        let mut a = 2.0;
        let mut i = 0;
        while a > 0.0 {
            //println!("{}", a);
            let previous_alpha_fitness = pack[0].fitness;

            self.evolve(a, &mut pack);

            //pack.iter().map(|t| t.solution.get_mst());
            pack.sort_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap());

            let new_alpha_fitness = pack[0].fitness;

            println!("new alpha {}", new_alpha_fitness);
            a -= phi;
            
            
            // if new_alpha_fitness < previous_alpha_fitness {
            //     println!("new alpha {}", new_alpha_fitness);
            //     println!("sol {:?}", pack[0].solution.get_mst_edges());
            //     println!("new iter fitness");

            //     a -= phi;
            //     i = 0;
            // }

            // if i > num_iter {
            //     i = 0;
            //     println!("new iter begger than num iter");
            //     //self.mix_vertices();
            //     a -= phi;
            //     dbg!(a);
            //     continue;
            // }

            i += 1;
            self.mix_vertices();
            //break;
        }
        pack[0].solution.clone()
    }

    fn mix_vertices(&mut self) {
        for i in 0..self.vertices.len() {
            let mut index = self.r4.gen::<usize>() % self.vertices.len();
            while i == index {
                index = self.r4.gen::<usize>() % self.vertices.len();
            }

            let temp = self.vertices[i];
            self.vertices[i] = self.vertices[index];
            self.vertices[index] = temp;
        }
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
