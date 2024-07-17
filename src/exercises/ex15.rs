use maths::utils::Lerp;
use maths::prelude::{Norm, Matrix, Vector};
use utils::{result, show, ex};
use maths::complex::Complex;
use crate::exercises::ex01::linear_combination;

pub fn ex15() {
    ex(15, "Complex vector spaces");
    println!("-- 00 Add, Subtract and Scale");
    result!(
        Complex::new(5., 1.5) + Complex::new(-1., 0.5),
        Complex::new(5., 1.5) - Complex::new(-1., 0.5),
        Complex::new(5., 1.5) * 2.,
    );
    println!("\n-- 01 Linear combination");
    let c42;
    let c19;
    let c01;
    let c10;
    let c00;
    show!(
        c42 = Complex::new(4., 2.),
        c19 = Complex::new(1., 9.),
        c01 = Complex::new(0., 1.),
        c10 = Complex::new(1., 0.),
        c00 = Complex::default(),
    );
    result!(linear_combination(&[Vector::from([c42, c19]), Vector::from([c01, c10])], &[0.5f32.into(), Complex::from_i(1.)]));
    println!("\n-- 02 Linear interpolation");
    result!(Complex::new(4., 2.).lerp(Complex::new(2., 4.), 0.5));
    println!("\n-- 03 Dot product");
    result!(Vector::from([c01, c10]).dot(&Vector::from([c10, c01])));
    println!("\n-- 04 Norm");
    let vr00i;
    let v000i;
    let v00r0;
    let v00ri;
    show!(
        v00ri = Vector::from([Complex::default(), Complex::new(1., 1.)]),
        v000i = Vector::from([Complex::default(), Complex::from_i(1.)]),
        v00r0 = Vector::from([Complex::default(), 1f32.into()])
    );
    show!(vr00i = Vector::from([c10, c01]));
    result!(vr00i.norm_1(), vr00i.norm(), vr00i.norm_inf());
    result!(v000i.norm_1(), v000i.norm(), v000i.norm_inf());
    result!(v00ri.norm_1(), v00ri.norm(), v00ri.norm_inf());
    println!("\n-- 05 Cosine");
    result!(v00ri.angle_cos(&v000i));
    result!(v000i.angle_cos(&v00ri));
    result!(v000i.angle_cos(&v000i));
    result!(v00r0.angle_cos(&v00r0));
    println!("\n-- 06 Cross product");
    result!(Vector::from([c10, c00, c01]).cross_product(&Vector::from([c00, c00, c01])));
    println!("\n-- 07 Linear map, matrix multiplication");
    let cm;
    let r0;
    let r1;
    let r2;
    show!(
        r0 = [c01, c10, c00],
        r1 = [c42, c00, c19],
        r2 = [c00, c42, Complex::from_i(41.)],
        cm = Matrix::from([r0, r1, r2]),
    );
    result!(
        cm * Vector::from([c00, c10, c01]),
        cm * cm
    );
    println!("\n-- 08 Trace");
    result!(cm.trace());
    println!("\n-- 09 Transpose");
    result!(cm.transpose());
    println!("\n-- 10 Row-echelon form");
    result!(cm.row_echelon());
    let mut cm2;
    show!(
        cm2 = cm,
        cm2[(1, 1)] = Complex::new(42., 64.),
        for i in 0..3 { cm2[(i, 0)] = Complex::default() }
    );
    result!(
        cm2,
        cm2.row_echelon()
    );
    println!("\n--11 Determinant");
    result!(
        cm.determinant(),
        cm2.determinant()
    );
    println!("\n--12 Inverse");
    result!(
        cm.inverse().unwrap(),
        format!("{:?}", cm2.inverse())
    );
    println!("\n--13 Rank");
    result!(
        cm.rank(),
        cm2.rank()
    );
}