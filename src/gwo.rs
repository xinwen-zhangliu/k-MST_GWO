use crate::tree::Tree;
use crate::tree::Vertex;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rand_distr::Uniform;

pub struct GWO {
    k: usize,
    population: usize,
    vertices: Vec<Vertex>,
}

impl GWO {
    pub fn new(population: usize, vertices: Vec<Vertex>, k: usize) -> Self {
        GWO {
            population,
            vertices,
            k,
        }
    }

    pub fn evolve(&mut self, a: f64, positions: &mut [Tree]) {
        // let mut alpha = vec![Vertex(0.0, 0.0, 0); self.k];
        // let mut beta = vec![Vertex(0.0, 0.0, 0); self.k];
        // let mut delta = vec![Vertex(0.0, 0.0, 0); self.k];

        let mut r1 = StdRng::seed_from_u64(1);
        let mut r2 = StdRng::seed_from_u64(2);
        //let mut r3 = StdRng::seed_from_u64(3);
        let mut uniform = Uniform::new(0.0, 1.0);
        let mut iter = 0;

        let A1 = 2.0 * a * r1.sample(uniform) - a;
        let C1 = 2.0 * r2.sample(uniform);
        // let D_1 = num::abs(C1 * position[0[.x - wol );
        iter += 1;


        for i in 0..positions.len(){
            
        }

        
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

    pub fn run_gwo(&mut self, num_iter : usize) {
        let mut positions : Vec<Tree> = Vec::new();
        // generation of random positions
        for _i in 0..self.population{
            positions.push(Tree::new(self.generate_solution(), self.k));
           // println!("{:?} : {} ", &positions[i].get_mst(), &positions[i].get_weight());
        }
        
        let mut counter = 0;
        while counter < num_iter{

            
            
            counter += 1;
        }
        
    }


    fn generate_solution(&self) -> Vec<Vertex> {
        let mut new_vertices : Vec<Vertex> = vec![Vertex(0.0, 0.0, 0); self.k];
        let mut vertices = self.vertices.clone();
        let mut r = StdRng::seed_from_u64(8);
        for i in 0..self.k{
            let index : usize = r.gen::<usize>() % vertices.len();
            new_vertices[i] = vertices[index];
            vertices.remove(index);
        }
        new_vertices
    }
}
