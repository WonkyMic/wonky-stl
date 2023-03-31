use polars::prelude::*;
use polars::df;
use stl_io::Vector;

// pub fn series_to_vector(series: Series) -> Vector<f32> {
//     return Vector::new([f32::try_from(series.get(1).unwrap()).unwrap(), series.get(2).unwrap(), series.get(3).unwrap()]);
// }

pub fn stl_vec_to_data_frame(vertices: Vec<Vector<f32>>) -> DataFrame{
    let x_vec = vertices.iter().map(|v| v[0]);
    let mut x_series = Vec::new();
    x_vec.for_each(|x| {
        x_series.push(x);
    });

    let y_vec = vertices.iter().map(|v| v[1]);
    let mut y_series = Vec::new();
    y_vec.for_each(|y| {
        y_series.push(y);
    });

    let z_vec = vertices.iter().map(|v| v[2]);
    let mut z_series = Vec::new();
    z_vec.for_each(|z| {
        z_series.push(z);
    });

    let mut i_series = Vec::new();
    for i in 0..vertices.len() as u32 {
        i_series.push(i);
    }

    return df![
        "i" => i_series,
        "x" => x_series,
        "y" => y_series,
        "z" => z_series,
    ].unwrap();
}