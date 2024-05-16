use crate::matrix::Matrix;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_projection() {
        println!("{}", projection(80f32.to_radians(), 16./9., 0.1, 100.));
    }
}