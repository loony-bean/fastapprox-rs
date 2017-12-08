#![cfg(test)]

extern crate special;
extern crate statrs;
extern crate fastapprox;

use std::convert::Into;
use fastapprox::{fast, faster};
use statrs::function::erf;

mod c;

const FLOATS: &[f32] = &[-5.0, -0.25, -0.05, 0.0, 0.05, 1.0, 2.0, 3.0, 10.0];
const POS_FLOATS: &[f32] = &[0.01, 0.05, 1.0, 2.1, 3.5, 100.0];
const BETWEEN_ONES: &[f32] = &[-0.9, -0.5, -0.1, -0.01, 0.0, 0.01, 0.1, 0.5, 0.9];
const BETWEEN_PIS: &[f32] = &[-3.14, -1.5, -1.0, -0.5, -0.1, -0.01, 0.0, 0.01, 0.1, 0.5, 1.0, 1.5, 3.14];
const BETWEEN_HALFPIS: &[f32] = &[-1.56, -1.5, -1.0, -0.5, -0.1, -0.01, 0.0, 0.01, 0.1, 0.5, 1.0, 1.5, 1.56];

fn compare<F1, F2, T>(func: F1, base: F2, values: &[f32], tolerance: T)
    where
        F1: Fn(f32) -> f32,
        F2: Fn(f32) -> f32,
        T: Into<Option<f32>>
{
    let tol = tolerance.into();
    for value in values {
        let r1 = func(*value);
        let r2 = base(*value);
        if let Some(tolerance) = tol {
            let d = if r2.abs() < 0.1 { (r1 - r2).abs() } else { ((r1 - r2) / r2).abs() };
            assert!(d < tolerance, "func({}) = {}, but base({}) = {}, Δ == {}", value, r1, value, r2, d);
        } else {
            assert_eq!(r1, r2, "func({}) = {}, but base({}) = {}", value, r1, value, r2);
        }
    }
}

fn compare_exact<F1, F2>(func: F1, base: F2, values: &[f32])
    where
        F1: Fn(f32) -> f32,
        F2: Fn(f32) -> f32
{
    compare(func, base, values, None);
}

fn compare_near<F1, F2>(func: F1, base: F2, values: &[f32])
    where
        F1: Fn(f32) -> f32,
        F2: Fn(f32) -> f32
{
    compare(func, base, values, 0.01);
}

fn compare_far<F1, F2>(func: F1, base: F2, values: &[f32])
    where
        F1: Fn(f32) -> f32,
        F2: Fn(f32) -> f32
{
    compare(func, base, values, 0.15);
}

#[test]
fn test_pow2_approx() {
    compare_exact(fast::pow2, c::fastpow2, FLOATS);
    compare_exact(faster::pow2, c::fasterpow2, FLOATS);
}

#[test]
fn test_pow2_exact() {
    compare_near(fast::pow2, |x| (2.0_f32).powf(x) , FLOATS);
    compare_far(faster::pow2, |x| (2.0_f32).powf(x) , FLOATS);
}

#[test]
fn test_log2_approx() {
    compare_exact(fast::log2, c::fastlog2, POS_FLOATS);
    compare_exact(faster::log2, c::fasterlog2, POS_FLOATS);
}

#[test]
fn test_log2_exact() {
    compare_near(fast::log2, f32::log2, POS_FLOATS);
    compare_far(faster::log2, f32::log2, POS_FLOATS);
}

#[test]
fn test_ln_approx() {
    compare_exact(fast::ln, c::fastlog, POS_FLOATS);
    compare_exact(faster::ln, c::fasterlog, POS_FLOATS);
}

#[test]
fn test_ln_exact() {
    compare_near(fast::ln, f32::ln, POS_FLOATS);
    compare_far(faster::ln, f32::ln, POS_FLOATS);
}

#[test]
fn test_exp_approx() {
    compare_exact(fast::exp, c::fastexp, FLOATS);
    compare_exact(faster::exp, c::fasterexp, FLOATS);
}

#[test]
fn test_exp_exact() {
    compare_near(fast::exp, f32::exp, FLOATS);
    compare_far(faster::exp, f32::exp, FLOATS);
}

#[test]
fn test_sigmoid_approx() {
    compare_exact(fast::sigmoid, c::fastsigmoid, FLOATS);
    compare_exact(faster::sigmoid, c::fastersigmoid, FLOATS);
}

#[test]
fn test_sigmoid_exact() {
    compare_near(fast::sigmoid, |x| (1.0_f32 + (-x).exp()).recip(), FLOATS);
    compare_far(faster::sigmoid, |x| (1.0_f32 + (-x).exp()).recip(), FLOATS);
}

#[test]
fn test_lgamma_approx() {
    compare_exact(fast::ln_gamma, c::fastlgamma, POS_FLOATS);
    compare_exact(faster::ln_gamma, c::fasterlgamma, POS_FLOATS);
}

#[test]
fn test_lgamma_exact() {
    compare_near(fast::ln_gamma, |x| special::ln_gamma(x as f64).0 as f32, POS_FLOATS);
    compare_far(faster::ln_gamma, |x| special::ln_gamma(x as f64).0 as f32, POS_FLOATS);
}

#[test]
fn test_digamma_approx() {
    compare_exact(fast::digamma, c::fastdigamma, POS_FLOATS);
    compare_exact(faster::digamma, c::fasterdigamma, POS_FLOATS);
}

#[test]
fn test_digamma_exact() {
    compare_near(fast::digamma, |x| special::digamma(x as f64) as f32, POS_FLOATS);
    compare_far(faster::digamma, |x| special::digamma(x as f64) as f32, POS_FLOATS);
}

#[test]
fn test_erf_approx() {
    compare_exact(fast::erf, c::fasterf, POS_FLOATS);
    compare_exact(faster::erf, c::fastererf, POS_FLOATS);
}

#[test]
fn test_erf_exact() {
    compare_near(fast::erf, |x| special::erf(x as f64) as f32, POS_FLOATS);
    compare_far(faster::erf, |x| special::erf(x as f64) as f32, POS_FLOATS);
}

#[test]
fn test_erfc_approx() {
    compare_exact(fast::erfc, c::fasterfc, POS_FLOATS);
    compare_exact(faster::erfc, c::fastererfc, POS_FLOATS);
}

#[test]
fn test_erfc_exact() {
    compare_near(fast::erfc, |x| special::erfc(x as f64) as f32, POS_FLOATS);
    compare_far(faster::erfc, |x| special::erfc(x as f64) as f32, POS_FLOATS);
}

#[test]
fn test_inverse_erf_approx() {
    compare_exact(fast::erf_inv, c::fastinverseerf, BETWEEN_ONES);
    compare_exact(faster::erf_inv, c::fasterinverseerf, BETWEEN_ONES);
}

#[test]
fn test_inverse_erf_exact() {
    compare_near(fast::erf_inv, |x| erf::erf_inv(x as f64) as f32, BETWEEN_ONES);
    compare_far(faster::erf_inv, |x| erf::erf_inv(x as f64) as f32, BETWEEN_ONES);
}

#[test]
fn test_sinh_approx() {
    compare_exact(fast::sinh, c::fastsinh, BETWEEN_PIS);
    compare_exact(faster::sinh, c::fastersinh, BETWEEN_PIS);
}

#[test]
fn test_sinh_exact() {
    compare_near(fast::sinh, f32::sinh, BETWEEN_PIS);
    compare_far(faster::sinh, f32::sinh, BETWEEN_PIS);
}

#[test]
fn test_cosh_approx() {
    compare_exact(fast::cosh, c::fastcosh, BETWEEN_PIS);
    compare_exact(faster::cosh, c::fastercosh, BETWEEN_PIS);
}

#[test]
fn test_cosh_exact() {
    compare_near(fast::cosh, f32::cosh, BETWEEN_PIS);
    compare_far(faster::cosh, f32::cosh, BETWEEN_PIS);
}

#[test]
fn test_tanh_approx() {
    compare_exact(fast::tanh, c::fasttanh, FLOATS);
    compare_exact(faster::tanh, c::fastertanh, FLOATS);
}

#[test]
fn test_tanh_exact() {
    compare_near(fast::tanh, f32::tanh, FLOATS);
    compare_far(faster::tanh, f32::tanh, FLOATS);
}

#[test]
fn test_lambertw_approx() {
    compare_exact(fast::lambertw, c::fastlambertw, FLOATS);
    compare_exact(faster::lambertw, c::fasterlambertw, FLOATS);
}

#[test]
fn test_lambertwexpx_approx() {
    compare_exact(fast::lambertwexpx, c::fastlambertwexpx, FLOATS);
    compare_exact(faster::lambertwexpx, c::fasterlambertwexpx, FLOATS);
}

#[test]
fn test_sin_approx() {
    compare_exact(fast::sin, c::fastsin, BETWEEN_PIS);
    compare_exact(faster::sin, c::fastersin, BETWEEN_PIS);
}

#[test]
fn test_sin_exact() {
    compare_near(fast::sin, f32::sin, BETWEEN_PIS);
    compare_far(faster::sin, f32::sin, BETWEEN_PIS);
}

#[test]
fn test_sinfull_approx() {
    compare_exact(fast::sinfull, c::fastsinfull, FLOATS);
    compare_exact(faster::sinfull, c::fastersinfull, FLOATS);
}

#[test]
fn test_sinfull_exact() {
    compare_near(fast::sinfull, f32::sin, FLOATS);
    compare_far(faster::sinfull, f32::sin, FLOATS);
}

#[test]
fn test_cos_approx() {
    compare_exact(fast::cos, c::fastcos, BETWEEN_PIS);
    compare_exact(faster::cos, c::fastercos, BETWEEN_PIS);
}

#[test]
fn test_cos_exact() {
    compare_near(fast::cos, f32::cos, BETWEEN_PIS);
    compare_far(faster::cos, f32::cos, BETWEEN_PIS);
}

#[test]
fn test_cosfull_approx() {
    compare_exact(fast::cosfull, c::fastcosfull, FLOATS);
    compare_exact(faster::cosfull, c::fastercosfull, FLOATS);
}

#[test]
fn test_cosfull_exact() {
    compare_near(fast::cosfull, f32::cos, FLOATS);
    compare_far(faster::cosfull, f32::cos, FLOATS);
}

#[test]
fn test_tan_approx() {
    compare_exact(fast::tan, c::fasttan, BETWEEN_HALFPIS);
    compare_exact(faster::tan, c::fastertan, BETWEEN_HALFPIS);
}

#[test]
fn test_tan_exact() {
    compare_near(fast::tan, f32::tan, BETWEEN_HALFPIS);
    compare_far(faster::tan, f32::tan, BETWEEN_HALFPIS);
}

#[test]
fn test_tanfull_approx() {
    compare_exact(fast::tanfull, c::fasttanfull, FLOATS);
    compare_exact(faster::tanfull, c::fastertanfull, FLOATS);
}

#[test]
fn test_tanfull_exact() {
    compare_near(fast::tanfull, f32::tan, FLOATS);
    compare_far(faster::tanfull, f32::tan, FLOATS);
}
