use crate::complex::Complex;
use crate::quaternion::Quaternion;

///helper trait used to get the value of 1 for random types (and also flag types that do not have a value of 1 available)
///https://github.com/rust-lang/rfcs/pull/3490
///still waiting for const trait functions :)
pub trait Unit {
    fn unit() -> Self;
}

impl Unit for f32 { fn unit() -> Self { 1. } }
impl Unit for f64 { fn unit() -> Self { 1. } }
impl Unit for i8 { fn unit() -> Self { 1 } }
impl Unit for i16 { fn unit() -> Self { 1 } }
impl Unit for i32 { fn unit() -> Self { 1 } }
impl Unit for i64 { fn unit() -> Self { 1 } }
impl Unit for i128 { fn unit() -> Self { 1 } }
impl Unit for u8 { fn unit() -> Self { 1 } }
impl Unit for u16 { fn unit() -> Self { 1 } }
impl Unit for u32 { fn unit() -> Self { 1 } }
impl Unit for u64 { fn unit() -> Self { 1 } }
impl Unit for u128 { fn unit() -> Self { 1 } }

impl Unit for Complex {
    fn unit() -> Self {
        Self {
            r: 1.,
            i: 0.
        }
    }
}

impl Unit for Quaternion {
    fn unit() -> Self {
        Self {
            r: 1.,
            i: 0.,
            j: 0.,
            k: 0.
        }
    }
}

//value=positive normalized number (not nan, inf, denormalized or <0) -> 1/sqrt(value)
//derived from formula seen there: https://www.youtube.com/watch?v=p8u_k2LIZyo
#[allow(dead_code)]
fn quake_rsqrt(value: f32) -> f32 {
    //in rust, direct conversion of ref to pointer of different type is not allowed, so I do a temporary cast to an intermediary pointer before casting it to the final pointer
    //in terms of safety: there is none, if the input value is not normal (ex: infinity, NaN or other degenerate forms) this will most likely result in NaN/Infinity
    //if value is < 0, it will result in an underflow (in release mode, it will usually result in -Inf)
    let mut y = unsafe { *(&(0x5f3759df - (*(&value as *const f32 as *const u32) >> 1)) as *const u32 as *const f32) };
    y = y * (1.5 - value * 0.5 * y * y);
    y * (1.5 - value * 0.5 * y * y)
}

#[test]
fn test_quake_rsqrt() {
    dbg!(quake_rsqrt(1.)); //tend to 1
    dbg!(quake_rsqrt(0.)); //tend to positive infinity (divide by zero)
    dbg!(quake_rsqrt(4.)); //tend to 0.5
    dbg!(quake_rsqrt(2.));
    dbg!({
        let t = 1. / quake_rsqrt(2.);
        t * t
    }); //tend to 2.
    dbg!(quake_rsqrt(f32::NAN)); //NaN
    dbg!(quake_rsqrt(f32::INFINITY)); //inf
    #[cfg(not(debug_assertions))]
    { //results in underflow error in debug mode (and yes, you can run tests in release mode)
        dbg!(quake_rsqrt(-1.)); //-inf
        dbg!(quake_rsqrt(f32::NEG_INFINITY)); //-inf
    }
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

///simplified version of dbg (does not show file/line, use display instead of debug)
#[macro_export]
macro_rules! result {
    () => {
        println!()
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                println!("{} = {}", stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(result!($val)),+,)
    }
}

///debug even more simplified: show the expression (and execute it) without the result
#[macro_export]
macro_rules! show {
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                println!("{}", stringify!($val));
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(show!($val)),+,)
    };
}

pub fn ex(number: usize, title: &str) {
    let l = title.chars().count() as i32;
    let ll = ((63 - l) / 2).max(0);
    let lr = (63 - l - ll).max(0);
    println!("\n{} exercise {number:02}: '{title:.63}' {}", "*".repeat(ll as usize), "*".repeat(lr as usize));
}