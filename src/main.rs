use std::fs::File;
use std::io::Write;

pub mod vector;
pub mod matrix;
pub mod exercises;
pub mod complex;
pub mod quaternion;
pub mod unit;

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

///since I use approximations of sqrt and other simplification, I allow myself an error of 1 epsilon ~10^-7
pub fn assert_f32_eq(left: f32, right: f32) {
    let lb = left + f32::EPSILON;
    let ll = left - f32::EPSILON;
    assert!(lb >= right && ll <= right, "[{ll} {lb}] != {right}");
}

pub fn assert_f64_eq(left: f64, right: f64) {
    let lb = left + f64::EPSILON;
    let ll = left - f64::EPSILON;
    assert!(lb >= right && ll <= right, "left [{ll} {lb}] != {right}");
}

fn main() {
    ex14();
}
