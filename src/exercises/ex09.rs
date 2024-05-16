use crate::matrix::Matrix;

impl <const C: usize, const R: usize, K: Copy + Default> Matrix<C, R, K> {
    pub fn transpose(&self) -> Matrix<R, C, K> {
        let mut out = Matrix::default();
        for c in 0..C {
            for r in 0..R {
                out[(r, c)] = self[(c, r)];
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose() {
        assert_eq!(dbg!(Matrix::<3, 3, f32>::unit(1.).transpose()), Matrix::<3, 3, f32>::unit(1.));
        assert_eq!(dbg!(Matrix::from([[4., 2.], [6., 9.]]).transpose()), Matrix::from([[4., 6.], [2., 9.]]));
    }
}