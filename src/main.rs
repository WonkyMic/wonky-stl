use std::fs::OpenOptions;
use polars::prelude::*;

mod map;

fn main() {
    let mut file = OpenOptions::new().read(true).open("./input/Body1.stl").unwrap();
    let mut stl = stl_io::create_stl_reader(&mut file).unwrap();
    let mesh = stl.as_indexed_triangles().unwrap();
    let vertices = mesh.vertices;
    let df = map::stl_vec_to_data_frame(vertices.clone());

    println!("dataframe {:?}", df);

    let descending = vec![false, false, false];
    let lazy_frame = df.lazy();
    let sorted = lazy_frame
        .sort_by_exprs(vec![col("x"), col("y"), col("z")], descending, false)
        .collect().unwrap();

    println!("sorted {:?}", sorted);
    let series_index = &sorted[0];    
    let origin_index =series_index.u32().unwrap().get(0).unwrap();

    println!("origin_index {:?}", origin_index);

    let origin = vertices[usize::try_from(origin_index).unwrap()];

    println!("origin {:?}", origin);
}
