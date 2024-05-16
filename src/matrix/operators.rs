use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};
use crate::matrix::Matrix;
use crate::vector::Vector;

impl <const C: usize, const R: usize, K> Index<usize> for Matrix<C, R, K> {
    type Output = Vector<C, K>;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < R, "Invalid row index {index} for matrix of size {C} (columns) * {R} (rows)");
        &self.0[index]
    }
}

impl <const C: usize, const R: usize, K> IndexMut<usize> for Matrix<C, R, K> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < R, "Invalid column index {index} for matrix of size {C} (columns) * {R} (rows)");
        &mut self.0[index]
    }
}

impl <const C: usize, const R: usize, K> Index<(usize, usize)> for Matrix<C, R, K> {
    type Output = K;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        assert!(index.0 < C, "Invalid column index {} for matrix of size {C} (columns) * {R} (rows)", index.0);
        assert!(index.1 < R, "Invalid row index {} for matrix of size {C} (columns) * {R} (rows)", index.1);
        &self.0[index.1][index.0]
    }
}

impl <const C: usize, const R: usize, K> IndexMut<(usize, usize)> for Matrix<C, R, K> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        assert!(index.0 < C, "Invalid column index {} for matrix of size {C} (columns) * {R} (rows)", index.0);
        assert!(index.1 < R, "Invalid row index {} for matrix of size {C} (columns) * {R} (rows)", index.1);
        &mut self.0[index.1][index.0]
    }
}

impl <const C: usize, const R: usize, K: Add<Output = K> + Copy> Add for Matrix<C, R, K> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        for r in 0..R {
            for c in 0..C {
                self[(c, r)] = self[(c, r)] + rhs[(c, r)];
            }
        }
        self
    }
}

impl <const C: usize, const R: usize, K: Add<Output = K> + Copy> Add<K> for Matrix<C, R, K> {
    type Output = Self;

    fn add(mut self, rhs: K) -> Self::Output {
        for r in 0..R {
            for c in 0..C {
                self[(c, r)] = self[(c, r)] + rhs;
            }
        }
        self
    }
}

impl <const C: usize, const R: usize, K: Add<Output = K> + Copy> Add<Vector<C, K>> for Matrix<C, R, K> {
    type Output = Self;

    fn add(mut self, rhs: Vector<C, K>) -> Self::Output {
        for r in 0..R {
            self[r] = self[r] + rhs;
        }
        self
    }
}

impl <const C: usize, const R: usize, K: Sub<Output = K> + Copy> Sub for Matrix<C, R, K> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        for r in 0..R {
            for c in 0..C {
                self[(c, r)] = self[(c, r)] - rhs[(c, r)];
            }
        }
        self
    }
}

impl <const C: usize, const R: usize, K: Sub<Output = K> + Copy> Sub<K> for Matrix<C, R, K> {
    type Output = Self;

    fn sub(mut self, rhs: K) -> Self::Output {
        for r in 0..R {
            for c in 0..C {
                self[(c, r)] = self[(c, r)] - rhs;
            }
        }
        self
    }
}

impl <const C: usize, const R: usize, K: Sub<Output = K> + Copy> Sub<Vector<C, K>> for Matrix<C, R, K> {
    type Output = Self;

    fn sub(mut self, rhs: Vector<C, K>) -> Self::Output {
        for r in 0..R {
            self[r] = self[r] - rhs;
        }
        self
    }
}

impl <const C: usize, const R: usize, K: Mul<Output = K> + Copy> Mul<K> for Matrix<C, R, K> {
    type Output = Self;

    fn mul(mut self, rhs: K) -> Self::Output {
        for r in 0..R {
            for c in 0..C {
                self[(c, r)] = self[(c, r)] * rhs;
            }
        }
        self
    }
}

impl <const C: usize, const R: usize, K: Div<Output = K> + Copy> Div<K> for Matrix<C, R, K> {
    type Output = Self;

    fn div(mut self, rhs: K) -> Self::Output {
        for r in 0..R {
            for c in 0..C {
                self[(c, r)] = self[(c, r)] / rhs;
            }
        }
        self
    }
}

impl <const C: usize, const R: usize, K: PartialEq> PartialEq for Matrix<C, R, K> {
    fn eq(&self, other: &Self) -> bool {
        for r in 0..R {
            for c in 0..C {
                if self[(c, r)] != other[(c, r)] {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_accessor() {
        dbg!(Matrix::from([[1., 2.], [3., 4.]])[1]);
        dbg!(Matrix::from([[1., 2.], [3., 4.]])[(1, 1)]);
    }

    #[test]
    fn test_operator_add() {
        dbg!(Matrix::from([[1., 2.], [3., 4.]]) + Matrix::from([[5., 6.], [7., 8.]]));
        dbg!(Matrix::from([[1., 2.], [3., 4.]]) + 2.);
        dbg!(Matrix::from([[1., 2.], [3., 4.]]) + Vector::from([5., 6.]));
    }

    #[test]
    fn test_operator_sub() {
        dbg!(Matrix::from([[1., 2.], [3., 4.]]) - Matrix::from([[5., 6.], [7., 8.]]));
        dbg!(Matrix::from([[1., 2.], [3., 4.]]) - 2.);
        dbg!(Matrix::from([[1., 2.], [3., 4.]]) - Vector::from([5., 6.]));
    }

    #[test]
    fn test_operator_mul() {
        dbg!(Matrix::from([[1., 2.], [3., 4.]]) * 2.);
    }

    #[test]
    fn test_operator_div() {
        dbg!(Matrix::from([[1., 2.], [3., 4.]]) / 2.);
    }
}