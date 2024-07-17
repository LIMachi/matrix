mod exercises;

use exercises::*;

///since I use approximations of sqrt and other simplification, I allow myself an error of 1 epsilon ~10^-7
pub fn assert_f32_eq(left: f32, right: f32) {
    let lb = left + std::f32::EPSILON;
    let ll = left - std::f32::EPSILON;
    assert!(lb >= right && ll <= right, "[{ll} {lb}] != {right}");
}

pub fn assert_f64_eq(left: f64, right: f64) {
    let lb = left + std::f64::EPSILON;
    let ll = left - std::f64::EPSILON;
    assert!(lb >= right && ll <= right, "left [{ll} {lb}] != {right}");
}

fn main() {
    ex00();
    ex01();
    ex02();
    ex03();
    ex04();
    ex05();
    ex06();
    ex07();
    ex08();
    ex09();
    ex10();
    ex11();
    ex12();
    ex13();
    ex14();
    ex15();
}
