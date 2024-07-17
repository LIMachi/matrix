use utils::{result, ex};
use maths::prelude::Vector;

pub fn ex06() {
    ex(6, "Cross product");
    result!(
        Vector::from([0., 0., 1.]).cross_product(&Vector::from([1., 0., 1.])),
        Vector::from([1., 2., 3.]).cross_product(&Vector::from([4., 5., 6.])),
        Vector::from([0., 0., 1.]).cross_product(&Vector::from([1., 0., 1.])),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0_z_by_x() {
        assert_eq!(dbg!({
            let u = Vector::from([0., 0., 1.]);
            let v = Vector::from([1., 0., 0.]);
            u.cross_product(&v)
        }), Vector::from([0., 1., 0.]));
    }

    #[test]
    fn test_1_cross() {
        assert_eq!(dbg!({
            let u = Vector::from([1., 2., 3.]);
            let v = Vector::from([4., 5., 6.]);
            u.cross_product(&v)
        }), Vector::from([-3., 6., -3.]));
    }

    #[test]
    fn test_2_negatives() {
        assert_eq!(dbg!({
            let u = Vector::from([4., 2., -3.]);
            let v = Vector::from([-2., -5., 16.]);
            u.cross_product(&v)
        }), Vector::from([17., -58., -16.]));
    }
}