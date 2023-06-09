#[allow(non_snake_case)]
use crate::tree::Edge;
use crate::tree::Tree;
use crate::tree::Vertex;
use libm::{pow, sqrt};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rand_distr::Uniform;

use svg::node::element::Circle;
use svg::node::element::Line;
use svg::Document;

pub struct GWO {
    k: usize,
    population: usize,
    vertices: Vec<Vertex>,
    r: StdRng,
    r1: StdRng,
    r2: StdRng,
    r3: StdRng,
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
        }
    }

    fn evolve(&mut self, a: f64, positions: &mut Vec<Wolf>) {
        for i in (0..self.population).rev() {
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

            let index = self.get_farthest(&positions[i]);
            let old_vertex = positions[i].solution.get_vertex(index);

            let center = self.get_center(&positions[i]);
            let old_distance = GWO::get_distance(&center, &old_vertex);

            let mut n = self.r3.gen::<usize>() % positions[i].vertices.len();
            let mut new_vertex = positions[i].vertices[n];

            let mut j = 0;
            let mut new_distance = f64::INFINITY;
            while new_distance > old_distance && j < 1000 {
                n = self.r3.gen::<usize>() % positions[i].vertices.len();
                new_vertex = positions[i].vertices[n];
                new_distance = GWO::get_distance(&new_vertex, &center);
                j += 1;
            }

            positions[i].solution.overwrite_vertex(index, new_vertex);
            positions[i].vertices[n] = old_vertex;

            let vertices = positions[i].solution.get_vertices();
            positions[i].solution = Tree::new(&vertices, self.k);

            positions[i].solution.get_mst();

            positions[i].position = new_vertex;

            positions[i].fitness = positions[i].solution.get_weight();

            let file = "image".to_owned() + &i.to_string() + &".svg".to_owned();
            self.plot(&positions[0], file);
        }
    }

    /// Function that runs the gwo heuristic.
    pub fn run_gwo(&mut self, num_iter: usize, phi: f64) -> Tree {
        let mut best_overall = Wolf {
            solution: Tree::new(&self.vertices, self.k),
            vertices: self.vertices.clone(),
            fitness: f64::INFINITY,
            position: Vertex::default(),
        };

        self.assign_vertices();

        //initializing the pack and its values
        let mut pack: Vec<Wolf> = vec![
            Wolf {
                solution: Tree::new(&self.vertices, self.k),
                vertices: self.vertices.clone(),
                fitness: 0.0,
                position: Vertex::default()
            };
            self.population
        ];

        let mut r = StdRng::seed_from_u64(7);

        for i in 0..self.population {
            let index: usize = r.gen::<usize>() % self.k;
            let vertices = self.generate_solution(&mut pack[i]);
            println!("{:?}", vertices);
            pack[i].solution = Tree::new(&vertices, self.k);
            //pack[i].vertices = vertices.clone();

            pack[i].position = vertices[index]; //len = 10

            pack[i].solution.get_mst();
            pack[i].fitness = pack[i].solution.get_weight();
        }

        // sort the solutions based on fitness
        pack.sort_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap());

        let mut a = 2.0;
        let mut i = 0;
        while a > 0.0 {
            let previous_alpha_fitness = pack[0].fitness;

            self.evolve(a, &mut pack);

            pack.sort_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap());

            let new_alpha_fitness = pack[0].fitness;


            if pack[0].fitness < best_overall.fitness {
                best_overall = pack[0].clone();
            }

            if new_alpha_fitness < previous_alpha_fitness {
                println!("new alpha {}", new_alpha_fitness);
                let sol = pack[0].solution.get_mst_edges();
                println!("sol {:?}", pack[0].solution.get_mst_edges());

                for edge in sol {
                    println!(
                        "S,{:?},{:?},{:?},{:?}",
                        edge.u.0, edge.u.1, edge.v.0, edge.v.1
                    );
                }

                a -= phi;
                println!("{}", a);
                i = 0;
            }

            if i > num_iter {
                i = 0;

                a -= phi;

                println!("{}", a);
                continue;
            }

            i += 1;
        }
        for i in 0..self.population {
            dbg!(pack[i].fitness);
            let file = "image".to_owned() + &i.to_string() + &".svg".to_owned();
            self.plot(&pack[i], file);
        }

        println!(
            "{:?}\n F : {}",
            best_overall.solution.get_mst_edges(),
            best_overall.fitness
        );
        self.plot(&best_overall, "alpha.svg".to_owned());
        best_overall.solution.clone()
    }

    fn assign_vertices(&mut self) {
        for i in 0..self.vertices.len() {
            self.vertices[i].2 = i;
        }
    }

    fn generate_solution(&mut self, wolf: &mut Wolf) -> Vec<Vertex> {
        let mut new_vertices: Vec<Vertex> = vec![Vertex(0.0, 0.0, 0); self.k];

        for i in 0..self.k {
            let index: usize = self.r.gen::<usize>() % wolf.vertices.len();
            dbg!(index);
            new_vertices[i] = wolf.vertices[index];
            wolf.vertices.remove(index);
        }
        new_vertices
    }

    fn plot(&self, wolf: &Wolf, file: String) {
        let mut document = Document::new().set("width", 116).set("height", 115);
        let edges = wolf.solution.get_mst_edges();
        for v in &self.vertices {
            let circle = Circle::new().set("cx", v.0).set("cy", v.1).set("r", 1);
            document = document.add(circle);
        }
        for edge in edges {
            let u = edge.u;
            let v = edge.v;
            let line = Line::new()
                .set("x1", u.0)
                .set("y1", u.1)
                .set("x2", v.0)
                .set("y2", v.1)
                .set("stroke-width", 0.5)
                .set("stroke", "black");

            document = document.add(line);
        }
        svg::save(file, &document).unwrap();
    }

    fn get_center(&self, wolf: &Wolf) -> Vertex {
        let vertices = wolf.solution.get_vertices();
        let mut x_sum: f64 = 0.0;
        let mut y_sum: f64 = 0.0;
        for i in 0..self.k {
            x_sum += vertices[i].0;
            y_sum += vertices[i].1;
        }
        Vertex(x_sum / self.k as f64, y_sum / self.k as f64, 0)
    }

    fn get_distance(p1: &Vertex, p2: &Vertex) -> f64 {
        sqrt(pow((p2.0 - p1.0) as f64, 2.0) + pow((p2.1 - p1.1) as f64, 2.0))
    }

    fn get_farthest(&self, wolf: &Wolf) -> usize {
        let vertices = wolf.solution.get_vertices();
        let center = self.get_center(wolf);
        let mut index: usize = 0;
        let mut max_distance = 0.0;
        for i in 0..vertices.len() {
            let new_distance = GWO::get_distance(&center, &vertices[i]);
            if new_distance > max_distance {
                max_distance = new_distance;
                index = i;
            }
        }

        index
    }
}
