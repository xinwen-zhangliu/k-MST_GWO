extern crate clap;
use std::env;
use std::path::PathBuf;

use clap::{arg, value_parser, ArgAction, Command};
use k_mst_gwo::gwo::GWO;
#[allow(non_snake_case)]
use k_mst_gwo::reader::Reader;
use k_mst_gwo::tree::Tree;
use k_mst_gwo::tree::Vertex;
#[allow(unused_assignments)]
/* Main thread of execution. */
fn main() {
    let mut population: u32 = 0;
    let mut k: u32 = 10;

    let db_path = env::var("CARGO_MANIFEST_DIR").unwrap_or("../..".to_string());
    let mut path = db_path + "/data/t3.txt";

    let matches = Command::new("Grey Wolf Optimization")
        .next_line_help(true)
        .arg(
            arg!(
                -f --file <PATH>
                    "Path to the file that contians the list of points to use.")
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(-p --population <VALUE>
                 "The number of wolves to be created.")
            .required(false)
            .action(ArgAction::Set)
            .value_parser(value_parser!(u32)),
        )
        .arg(
            arg!(-k  --ksize <VALUE>
                 "The number of vertices that the minimum spanning tree is to contain.")
            .required(false)
            .action(ArgAction::Set)
            .value_parser(value_parser!(u32)),
        )
        .get_matches();

    if let Some(cities_path) = matches.get_one::<PathBuf>("file") {
        path = cities_path.clone().into_os_string().into_string().unwrap();
    }
    if let Some(pop) = matches.get_one::<u32>("population") {
        population = *pop;
    }

    if let Some(nk) = matches.get_one::<u32>("ksize") {
        k = *nk;
    }

    let r = Reader::new(path);
    let vertices = r.get_vertices();
    let mut sub_vertices = vec![Vertex(0.0, 0.0, 0); 10];
    sub_vertices[..].clone_from_slice(&vertices[..10]);
    let mut t = Tree::new(&vertices, 150);

    // gwo.run_gwo(1);
    //let mut gwo = GWO::new(10, vertices.clone(), 10, 54);
    //let mut gwo = GWO::new(10, vertices.clone(), 10, 3); 51
    //let mut gwo = GWO::new(15, vertices.clone(), 10, 4295);


    //57.515
      //  let mut gwo = GWO::new(10, vertices.clone(), 10, 9572);
    // let solution = gwo.run_gwo(1000, 0.01);

           let mut gwo = GWO::new(15, vertices.clone(), 10, 43290);
    let solution = gwo.run_gwo(1000, 0.01);


    



    // println!("{}", &solution.get_weight());
    // println!("{:?}", &solution.get_mst_edges());
}
