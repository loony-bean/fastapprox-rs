#![deny(missing_docs)]

//! # fastapprox
//!
//! Rust version of a [library](https://code.google.com/archive/p/fastapprox/) by Paul Mineiro.
//!
//! Fast approximate versions of certain functions that arise in machine learning.
//!
//! E.g. in [Vowpal Wabbit](https://github.com/JohnLangford/vowpal_wabbit) this is one of the many clever tricks used to reach it's incredible training speed.
//!
//! Module names `fast` and `faster` come from the original work, and represent sets of the same algorithms with different speed-accuracy levels.
//!
//! Although approximate functions could give you some speedup (but not necessarily!), some warnings should be provided:
//!
//! - Run your own benchmarks
//! - Make sure math is a bottleneck in your algorithm
//! - Pay attention to convergence
//! - SIMD versions are not implemented (yet, until `simd` crate will graduate from the nursery)
//!
//! ## Benchmarks
//! Running `cargo bench` on MacBook Pro (Late 2013), 2.6 GHz Intel Core i7, gives the following output:
//!
//! ```text
//! test cos_fast            ... bench:       3,674 ns/iter (+/- 778)
//! test cos_faster          ... bench:       1,559 ns/iter (+/- 206)
//! test cos_std             ... bench:       7,329 ns/iter (+/- 104)
//! test cosfull_fast        ... bench:       6,880 ns/iter (+/- 1,021)
//! test cosfull_faster      ... bench:       4,238 ns/iter (+/- 183)
//! test cosh_fast           ... bench:       8,270 ns/iter (+/- 4,190)
//! test cosh_faster         ... bench:       2,451 ns/iter (+/- 175)
//! test cosh_std            ... bench:       4,407 ns/iter (+/- 798)
//! test digamma_fast        ... bench:       4,644 ns/iter (+/- 1,126)
//! test digamma_faster      ... bench:       3,770 ns/iter (+/- 417)
//! test digamma_special     ... bench:      16,260 ns/iter (+/- 1,374)
//! test digamma_statrs      ... bench:      14,401 ns/iter (+/- 3,198)
//! test erf_fast            ... bench:      54,401 ns/iter (+/- 20,448)
//! test erf_faster          ... bench:       2,359 ns/iter (+/- 426)
//! test erf_inv_fast        ... bench:       5,958 ns/iter (+/- 2,794)
//! test erf_inv_faster      ... bench:       2,113 ns/iter (+/- 153)
//! test erf_inv_statrs      ... bench:         769 ns/iter (+/- 83)
//! test erf_special         ... bench:       4,948 ns/iter (+/- 1,443)
//! test erf_statrs          ... bench:       6,287 ns/iter (+/- 246)
//! test erfc_fast           ... bench:      54,201 ns/iter (+/- 6,307)
//! test erfc_faster         ... bench:       2,052 ns/iter (+/- 112)
//! test erfc_special        ... bench:       4,887 ns/iter (+/- 362)
//! test exp_fast            ... bench:       3,774 ns/iter (+/- 235)
//! test exp_faster          ... bench:       1,400 ns/iter (+/- 260)
//! test exp_std             ... bench:       2,760 ns/iter (+/- 202)
//! test lambertw_fast       ... bench:      21,040 ns/iter (+/- 2,523)
//! test lambertw_faster     ... bench:      24,840 ns/iter (+/- 8,939)
//! test lambertwexpx_fast   ... bench:      13,240 ns/iter (+/- 858)
//! test lambertwexpx_faster ... bench:       8,857 ns/iter (+/- 1,439)
//! test ln_fast             ... bench:       2,087 ns/iter (+/- 528)
//! test ln_faster           ... bench:       1,214 ns/iter (+/- 447)
//! test ln_gamma_fast       ... bench:       6,193 ns/iter (+/- 901)
//! test ln_gamma_faster     ... bench:       2,952 ns/iter (+/- 211)
//! test ln_gamma_special    ... bench:      36,726 ns/iter (+/- 846)
//! test ln_gamma_statrs     ... bench:      56,295 ns/iter (+/- 973)
//! test ln_std              ... bench:       5,175 ns/iter (+/- 321)
//! test log2_fast           ... bench:       1,998 ns/iter (+/- 335)
//! test log2_faster         ... bench:       1,206 ns/iter (+/- 188)
//! test log2_std            ... bench:       5,087 ns/iter (+/- 1,736)
//! test pow2_fast           ... bench:       3,472 ns/iter (+/- 192)
//! test pow2_faster         ... bench:       1,069 ns/iter (+/- 356)
//! test pow2_std            ... bench:       2,942 ns/iter (+/- 172)
//! test pow_fast            ... bench:       6,769 ns/iter (+/- 1,460)
//! test pow_faster          ... bench:       2,365 ns/iter (+/- 293)
//! test pow_std             ... bench:      13,147 ns/iter (+/- 1,000)
//! test sigmoid_fast        ... bench:       4,818 ns/iter (+/- 428)
//! test sigmoid_faster      ... bench:       1,869 ns/iter (+/- 56)
//! test sigmoid_std         ... bench:       3,312 ns/iter (+/- 131)
//! test sin_fast            ... bench:       3,038 ns/iter (+/- 685)
//! test sin_faster          ... bench:       1,875 ns/iter (+/- 481)
//! test sin_std             ... bench:       7,215 ns/iter (+/- 39)
//! test sinfull_fast        ... bench:       6,483 ns/iter (+/- 405)
//! test sinfull_faster      ... bench:       3,973 ns/iter (+/- 874)
//! test sinh_fast           ... bench:       8,394 ns/iter (+/- 618)
//! test sinh_faster         ... bench:       2,588 ns/iter (+/- 286)
//! test sinh_std            ... bench:       4,434 ns/iter (+/- 329)
//! test tan_fast            ... bench:       5,165 ns/iter (+/- 174)
//! test tan_faster          ... bench:       3,423 ns/iter (+/- 1,083)
//! test tan_std             ... bench:       7,695 ns/iter (+/- 453)
//! test tanfull_fast        ... bench:      10,299 ns/iter (+/- 1,761)
//! test tanfull_faster      ... bench:       6,214 ns/iter (+/- 524)
//! test tanh_fast           ... bench:       5,598 ns/iter (+/- 709)
//! test tanh_faster         ... bench:       1,921 ns/iter (+/- 445)
//! test tanh_std            ... bench:       4,056 ns/iter (+/- 306)
//! ```

/// Fast approximations with small error.
pub mod fast;

/// Faster approximations with considerable error.
pub mod faster;
