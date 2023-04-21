use crate::tree::Vertex;
use std::fs;

pub struct Reader {
    path: String,
}

impl Reader {
    pub fn new(path: String) -> Self {
        Reader { path }
    }

    ///Reads vertices input from file and returns a vector of Verticesx
    pub fn get_vertices(&self) -> Vec<Vertex> {
        let contents: String = fs::read_to_string(&self.path)
            .expect("There was an error finding or reading the file.");

        contents
            .split("\n")
            .filter(|x| !x.is_empty())
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| -> Vertex {
                let v: Vec<&str> = x.split(",").collect();
                Vertex(
                    v[0].to_owned().parse::<f64>().unwrap(),
                    v[1].to_owned().parse::<f64>().unwrap(),
                    0
                )
            })
            .collect()
    }
}
