use std::fs::File;
use std::io::Write;

mod vector;
mod matrix;
mod exercises;
mod utils;
mod complex;

fn ex14() {
    let p = exercises::ex14::projection(80f32.to_radians(), 16./9., 0.1, 100.);
    let mut file = File::create("./proj").unwrap();
    for r in 0..4 {
        for c in 0..3 {
            let v = p[(c, r)];
            file.write_fmt(format_args!("{v}{}, ", if v.fract().abs() <= f32::EPSILON { "." } else { "" })).unwrap();
        }
        let v = p[(3, r)];
        file.write_fmt(format_args!("{v}{}\n", if v.fract().abs() <= f32::EPSILON { "." } else { "" })).unwrap();
    }
    println!("generated file './proj', now use ./matrix_display_linux/display or ./matrix_display_mac/display (test executable provided by 42 for corrections)");
}

fn main() {
    ex14();
}
