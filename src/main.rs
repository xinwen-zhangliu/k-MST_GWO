use k_mst_gwo::gwo::GWO;
#[allow(non_snake_case)]
use k_mst_gwo::reader::Reader;
use k_mst_gwo::tree::Tree;
use k_mst_gwo::tree::Vertex;

fn main() {
    let r = Reader::new("src/puntos.txt".to_owned());
    let vertices = r.get_vertices();
    let mut sub_vertices = vec![Vertex(0.0, 0.0, 0); 10];
    sub_vertices[..].clone_from_slice(&vertices[..10]);
    let mut t = Tree::new(&vertices, 150);

    // let mut gwo = GWO::new(3 , vertices.clone(), 10);
    // gwo.run_gwo(1);

    let mut gwo = GWO::new(10, vertices.clone(), 10, 9);
    let solution = gwo.run_gwo(1000000, 0.98);

    // let tree = t.get_mst();
    // println!("{:?}", &tree);
    // println!("{}", t.get_weight());
    // println!("{}", tree.len());

    println!("{}", &solution.get_weight());
    println!("{:?}", &solution.get_mst_edges());
}
