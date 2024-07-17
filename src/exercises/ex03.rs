use utils::{result, ex};
use maths::prelude::Vector;

pub fn ex03() {
    ex(3, "Dot product");
    result!(
        Vector::from([0., 0.]).dot(&Vector::from([1., 1.])),
        Vector::from([1., 1.]).dot(&Vector::from([1., 1.])),
        Vector::from([-1., 6.]).dot(&Vector::from([3., 2.]))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0_null_dot() {
        assert_eq!(dbg!({
            let u = Vector::from([0., 0.]);
            let v = Vector::from([1., 1.]);
            u.dot(&v)
        }), 0.);
    }

    #[test]
    fn test_1_dot() {
        assert_eq!(dbg!({
            let u = Vector::from([1., 1.]);
            let v = Vector::from([1., 1.]);
            u.dot(&v)
        }), 2.);
    }

    #[test]
    fn test_2_negative_dot() {
        assert_eq!(dbg!({
            let u = Vector::from([-1., 6.]);
            let v = Vector::from([3., 2.]);
            u.dot(&v)
        }), 9.);
    }
}