use std::fs::File;
use std::io::Write;
use crate::matrix::Matrix;
use crate::utils::ex;

//TODO: test on a linux or mac with vulkan (as the executable given by 42 uses vulkan and crashes on virtual machines due to vulkan not being available)
pub fn projection(fov: f32, ratio: f32, near: f32, far: f32) -> Matrix<4, 4, f32> {
    let s = 1. / (fov / 2.).tan();
    let l = near - far;
    Matrix::from([
        [s, 0., 0., 0.],
        [0., s / ratio, 0., 0.],
        [0., 0., (far + near) / l, -1.],
        [0., 0., 2. * near * far / l, 1.]
    ])
}

pub fn ex14() {
    ex(14, "Projection matrix");
    let p = projection(80f32.to_radians(), 16./9., 0.1, 100.);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_projection() {
        println!("{}", projection(80f32.to_radians(), 16./9., 0.1, 100.));
    }
}