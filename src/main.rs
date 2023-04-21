#[allow(non_snake_case)]
use k_mst_gwo::reader::Reader;
use k_mst_gwo::tree::Tree;
use k_mst_gwo::gwo::GWO;

fn main(){
    let r = Reader::new("src/puntos.txt".to_owned());
    let vertices = r.get_vertices();

    let mut t = Tree::new(vertices.clone(), 10);

    let mut gwo = GWO::new(3 , vertices.clone(), 10);
    gwo.run_gwo(1);
    println!("{:?}", t.get_mst());
    println!("{}", t.get_weight())
}
