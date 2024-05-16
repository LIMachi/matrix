use std::fs::File;
use std::io::Write;

mod vector;
mod matrix;
mod exercises;
mod utils;
mod complex;

fn ex14() {
    let p = exercises::ex14::projection(80f32.to_radians(), 16./9., 0.1, 100.);
    let mut s = String::new();
    for r in 0..4 {
        for c in 0..3 {
            let v = p[(c, r)];
            s += format!("{v}{}, ", if v.fract().abs() <= f32::EPSILON { "." } else { "" }).as_str();
        }
        let v = p[(3, r)];
        s += format!("{v}{}\n", if v.fract().abs() <= f32::EPSILON { "." } else { "" }).as_str();
    }
    let mut file = File::create("./matrix_display/proj").unwrap();
    file.write_all(s.as_bytes()).unwrap();
    println!("generated file './matrix_display/proj', now use ./matrix_display/display (test executable provided by 42 for corrections)");
}

fn main() {
    ex14();
}
