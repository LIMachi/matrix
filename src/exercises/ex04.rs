use utils::{result, show, ex};
use maths::prelude::{Vector, Norm};

///note: the subject requires self to be mutable, but by definition, the norm of a vector does not require transformation, so I removed the superfluous mut
///I allowed myself to use abs for f32 (as it is basically: `if a < 0. { -a } else { a }`)
///I also had to generalize the norm to work on non vector to work with complex numbers

pub fn ex04() {
    ex(4, "Norm");
    let mut u;
    show!(u = Vector::from([0., 0., 0.]));
    result!(u.norm_1(), u.norm(), u.norm_inf());
    show!(u = Vector::from([1., 2., 3.]));
    result!(u.norm_1(), u.norm(), u.norm_inf());
    let u;
    show!(u = Vector::from([-1., -2.]));
    result!(u.norm_1(), u.norm(), u.norm_inf());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0_null_vector() {
        assert_eq!(dbg!({
            let u = Vector::from([0., 0., 0.]);
            let n1 = u.norm_1();
            let n = u.norm();
            let ni = u.norm_inf();
            println!("{}, {}, {}", n1, n, ni);
            (n1, n, ni)
        }), (0., 0., 0.));
    }

    #[test]
    fn test_1_vector() {
        assert_eq!(dbg!({
            let u = Vector::from([1., 2., 3.]);
            let n1 = u.norm_1();
            let n = u.norm();
            let ni = u.norm_inf();
            println!("{}, {}, {}", n1, n, ni);
            (n1, n, ni)
        }), (6., 3.7416573, 3.));
    }

    #[test]
    fn test_2_negative_vector() {
        assert_eq!(dbg!({
            let u = Vector::from([-1., -2.]);
            let n1 = u.norm_1();
            let n = u.norm();
            let ni = u.norm_inf();
            println!("{}, {}, {}", n1, n, ni);
            (n1, n, ni)
        }), (3.,  2.2360679, 2.));
    }
}