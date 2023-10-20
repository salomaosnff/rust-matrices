mod macros;
mod matrix;
mod angle;
mod image;

use image::Image;
use matrix::*;

use crate::angle::Angle;

fn main() {
    // let p = point![1.0, 1.0];
    // let t = translate![1.0, 1.0];
    // let s = scale![2.0, 2.0];
    // let r = rotate![Angle::degrees(90.0)];

    // println!("Ponto: {p}");
    // println!("Translação: {t}");
    // println!("Escala: {s}");
    // println!("Rotação: {r}");

    // let transform_matrix = transform!(p, t, s);

    // println!("Resultado: {}", transform_matrix);

    let mut img_data = Image::new(mat![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0;
        0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0;
        0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0;
        0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0;
        0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0;
        0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0;
        0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0;
        0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0;
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
    ]);

    println!("{}", mat![
        1.0, 2.0, 3.0;
        3.0, 4.0, 5.0
    ] * mat![
        1.0;
        2.0;
        3.0
    ]);
    
    img_data.transform(scale!(0.5, 0.5));

    println!("{}", img_data);
}
