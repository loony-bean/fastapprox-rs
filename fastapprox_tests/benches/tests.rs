#[macro_use]
extern crate bencher;
extern crate fastapprox;
extern crate special;
extern crate statrs;

use bencher::Bencher;
use fastapprox::{fast, faster};
use statrs::function::{gamma, erf};

const ITERATIONS: u32 = 1000;

fn run<F: Fn(f32) -> f32>(bench: &mut Bencher, cb: F) {
    bench.iter(|| {
        (0..ITERATIONS).fold(0.0, |a, b| a + cb(b as f32))
    })
}

fn log2_std(bench: &mut Bencher) {
    run(bench, |b| b.log2())
}

fn log2_fast(bench: &mut Bencher) {
    run(bench, fast::log2)
}

fn log2_faster(bench: &mut Bencher) {
    run(bench, faster::log2)
}

fn ln_std(bench: &mut Bencher) {
    run(bench, |b| b.ln())
}

fn ln_fast(bench: &mut Bencher) {
    run(bench, fast::ln);
}

fn ln_faster(bench: &mut Bencher) {
    run(bench, faster::ln);
}

fn exp_std(bench: &mut Bencher) {
    run(bench, |b| b.exp())
}

fn exp_fast(bench: &mut Bencher) {
    run(bench, fast::exp);
}

fn exp_faster(bench: &mut Bencher) {
    run(bench, faster::exp);
}

fn pow2_std(bench: &mut Bencher) {
    run(bench, |b| 2.0_f32.powf(b))
}

fn pow2_fast(bench: &mut Bencher) {
    run(bench, |b| fast::pow2(b))
}

fn pow2_faster(bench: &mut Bencher) {
    run(bench, |b| faster::pow2(b))
}

fn pow_std(bench: &mut Bencher) {
    run(bench, |b| b.powf(1.5))
}

fn pow_fast(bench: &mut Bencher) {
    run(bench, |b| fast::pow(b, 1.5))
}

fn pow_faster(bench: &mut Bencher) {
    run(bench, |b| faster::pow(b, 1.5))
}

fn sigmoid_std(bench: &mut Bencher) {
    run(bench, |b| 1.0_f32 / (1.0_f32 + (-b).exp()))
}

fn sigmoid_fast(bench: &mut Bencher) {
    run(bench, fast::sigmoid)
}

fn sigmoid_faster(bench: &mut Bencher) {
    run(bench, faster::sigmoid)
}

fn ln_gamma_special(bench: &mut Bencher) {
    run(bench, |b| special::Gamma::ln_gamma(b as f64).0 as f32)
}

fn ln_gamma_statrs(bench: &mut Bencher) {
    run(bench, |b| gamma::ln_gamma(b as f64) as f32)
}

fn ln_gamma_fast(bench: &mut Bencher) {
    run(bench, fast::ln_gamma)
}

fn ln_gamma_faster(bench: &mut Bencher) {
    run(bench, faster::ln_gamma)
}

fn digamma_special(bench: &mut Bencher) {
    run(bench, |b| special::Gamma::digamma(b as f64) as f32)
}

fn digamma_statrs(bench: &mut Bencher) {
    run(bench, |b| gamma::digamma(b as f64) as f32)
}

fn digamma_fast(bench: &mut Bencher) {
    run(bench, fast::digamma)
}

fn digamma_faster(bench: &mut Bencher) {
    run(bench, faster::digamma)
}

fn erfc_special(bench: &mut Bencher) {
    run(bench, |b| special::Error::compl_error(b as f64) as f32)
}

fn erfc_fast(bench: &mut Bencher) {
    run(bench, fast::erfc)
}

fn erfc_faster(bench: &mut Bencher) {
    run(bench, faster::erfc)
}

fn erf_statrs(bench: &mut Bencher) {
    run(bench, |b| erf::erf(b as f64) as f32)
}

fn erf_special(bench: &mut Bencher) {
    run(bench, |b| special::Error::error(b as f64) as f32)
}

fn erf_fast(bench: &mut Bencher) {
    run(bench, fast::erf)
}

fn erf_faster(bench: &mut Bencher) {
    run(bench, faster::erf)
}

fn erf_inv_statrs(bench: &mut Bencher) {
    run(bench, |b| erf::erf_inv(b as f64) as f32)
}

fn erf_inv_fast(bench: &mut Bencher) {
    run(bench, fast::erf_inv)
}

fn erf_inv_faster(bench: &mut Bencher) {
    run(bench, faster::erf_inv)
}

fn sinh_std(bench: &mut Bencher) {
    run(bench, |b| b.sinh())
}

fn sinh_fast(bench: &mut Bencher) {
    run(bench, fast::sinh)
}

fn sinh_faster(bench: &mut Bencher) {
    run(bench, faster::sinh)
}

fn cosh_std(bench: &mut Bencher) {
    run(bench, |b| b.cosh())
}

fn cosh_fast(bench: &mut Bencher) {
    run(bench, fast::cosh)
}

fn cosh_faster(bench: &mut Bencher) {
    run(bench, faster::cosh)
}

fn tanh_std(bench: &mut Bencher) {
    run(bench, |b| b.tanh())
}

fn tanh_fast(bench: &mut Bencher) {
    run(bench, fast::tanh)
}

fn tanh_faster(bench: &mut Bencher) {
    run(bench, faster::tanh)
}

fn lambertw_fast(bench: &mut Bencher) {
    run(bench, fast::lambertw)
}

fn lambertw_faster(bench: &mut Bencher) {
    run(bench, faster::lambertw)
}

fn lambertwexpx_fast(bench: &mut Bencher) {
    run(bench, fast::lambertwexpx)
}

fn lambertwexpx_faster(bench: &mut Bencher) {
    run(bench, faster::lambertwexpx)
}

fn sin_std(bench: &mut Bencher) {
    run(bench, |b| b.sin())
}

fn sin_fast(bench: &mut Bencher) {
    run(bench, fast::sin)
}

fn sinfull_fast(bench: &mut Bencher) {
    run(bench, fast::sinfull)
}

fn sin_faster(bench: &mut Bencher) {
    run(bench, faster::sin)
}

fn sinfull_faster(bench: &mut Bencher) {
    run(bench, faster::sinfull)
}

fn cos_std(bench: &mut Bencher) {
    run(bench, |b| b.cos())
}

fn cos_fast(bench: &mut Bencher) {
    run(bench, fast::cos)
}

fn cosfull_fast(bench: &mut Bencher) {
    run(bench, fast::cosfull)
}

fn cos_faster(bench: &mut Bencher) {
    run(bench, faster::cos)
}

fn cosfull_faster(bench: &mut Bencher) {
    run(bench, faster::cosfull)
}

fn tan_std(bench: &mut Bencher) {
    run(bench, |b| b.tan())
}

fn tan_fast(bench: &mut Bencher) {
    run(bench, fast::tan)
}

fn tanfull_fast(bench: &mut Bencher) {
    run(bench, fast::tanfull)
}

fn tan_faster(bench: &mut Bencher) {
    run(bench, faster::tan)
}

fn tanfull_faster(bench: &mut Bencher) {
    run(bench, faster::tanfull)
}

benchmark_group!(benches,
    log2_std,
    log2_fast,
    log2_faster,
    ln_std,
    ln_fast,
    ln_faster,
    exp_std,
    exp_fast,
    exp_faster,
    pow2_std,
    pow2_fast,
    pow2_faster,
    pow_std,
    pow_fast,
    pow_faster,
    sigmoid_std,
    sigmoid_fast,
    sigmoid_faster,
    ln_gamma_special,
    ln_gamma_statrs,
    ln_gamma_fast,
    ln_gamma_faster,
    digamma_special,
    digamma_statrs,
    digamma_fast,
    digamma_faster,
    erf_statrs,
    erf_special,
    erf_fast,
    erf_faster,
    erfc_special,
    erfc_fast,
    erfc_faster,
    erf_inv_statrs,
    erf_inv_fast,
    erf_inv_faster,
    sinh_std,
    sinh_fast,
    sinh_faster,
    cosh_std,
    cosh_fast,
    cosh_faster,
    tanh_std,
    tanh_fast,
    tanh_faster,
    lambertw_fast,
    lambertw_faster,
    lambertwexpx_fast,
    lambertwexpx_faster,
    sin_std,
    sin_fast,
    sinfull_fast,
    sin_faster,
    sinfull_faster,
    cos_std,
    cos_fast,
    cosfull_fast,
    cos_faster,
    cosfull_faster,
    tan_std,
    tan_fast,
    tanfull_fast,
    tan_faster,
    tanfull_faster
    );
benchmark_main!(benches);