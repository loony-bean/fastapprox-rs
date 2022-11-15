use crate::bits::*;
use crate::faster;

/// Base 2 logarithm.
#[inline]
pub fn log2(x: f32) -> f32 {
    let vx = to_bits(x);
    let mx = from_bits((vx & 0x007FFFFF_u32) | 0x3f000000);
    let y = vx as f32;
    y.mul_add(1.1920928955078125e-7_f32, -124.22551499_f32)
        + mx.mul_add(-1.498030302_f32, -1.72587999_f32 / (0.3520887068_f32 + mx))
}

/// Natural logarithm.
#[inline]
pub fn ln(x: f32) -> f32 {
    0.69314718_f32 * log2(x)
}

/// Raises 2 to a floating point power.
#[inline]
pub fn pow2(p: f32) -> f32 {
    let offset = if p < 0.0 { 1.0_f32 } else { 0.0_f32 };
    let clipp = if p < -126.0 { -126.0_f32 } else { p };
    let w = clipp as i32;
    let z = clipp - (w as f32) + offset;

    let v = ((1 << 23) as f32
        * z.mul_add(
            -1.49012907_f32,
            clipp + 121.2740575_f32 + 27.7280233_f32 / (4.84252568_f32 - z),
        )) as u32;
    from_bits(v)
}

/// Raises a number to a floating point power.
#[inline]
pub fn pow(x: f32, p: f32) -> f32 {
    pow2(p * log2(x))
}

/// Exponential function.
#[inline]
pub fn exp(p: f32) -> f32 {
    pow2(1.442695040_f32 * p)
}

/// Sigmoid function.
#[inline]
pub fn sigmoid(x: f32) -> f32 {
    1.0_f32 / (1.0_f32 + exp(-x))
}

/// Natural logarithm of the Gamma function.
///
/// Only works for positive values.
#[inline]
pub fn ln_gamma(x: f32) -> f32 {
    let logterm = ln(x * (1.0_f32 + x) * (2.0_f32 + x));
    let xp3 = 3.0_f32 + x;

    (2.5_f32 + x).mul_add(
        ln(xp3),
        -2.081061466_f32 - x + 0.0833333_f32 / xp3 - logterm,
    )
}

/// Digamma function.
///
/// Only works for positive values.
#[inline]
pub fn digamma(x: f32) -> f32 {
    let twopx = 2.0_f32 + x;
    let logterm = ln(twopx);

    x.mul_add(
        x.mul_add(x.mul_add(-30.0_f32, -127.0_f32), -157.0_f32),
        -48.0_f32,
    ) / (12.0_f32 * x * (1.0_f32 + x) * twopx * twopx)
        + logterm
}

/// Complementary error function.
#[inline]
pub fn erfc(x: f32) -> f32 {
    const K: f32 = 3.3509633149424609;
    const A: f32 = 0.07219054755431126;
    const B: f32 = 15.418191568719577;
    const C: f32 = 5.609846028328545;

    let mut v = to_bits(C * x);
    let xsq = x * x;
    let xquad = xsq * xsq;

    v |= 0x80000000;

    (-A * x * B.mul_add(xquad, -1.0_f32)).mul_add(
        faster::pow2(from_bits(v)),
        2.0_f32 / (1.0_f32 + pow2(K * x)),
    )
}

/// Error function.
#[inline]
pub fn erf(x: f32) -> f32 {
    1.0_f32 - erfc(x)
}

/// Inverse error function.
#[inline]
pub fn erf_inv(x: f32) -> f32 {
    const INVK: f32 = 0.30004578719350504;
    const A: f32 = 0.020287853348211326;
    const B: f32 = 0.07236892874789555;
    const C: f32 = 0.9913030456864257;
    const D: f32 = 0.8059775923760193;

    let xsq = x * x;

    INVK.mul_add(
        log2((1.0_f32 + x) / (1.0_f32 - x)),
        x * (-B).mul_add(xsq, A) / (-D).mul_add(xsq, C),
    )
}

/// Hyperbolic sine function.
#[inline]
pub fn sinh(p: f32) -> f32 {
    0.5_f32 * (exp(p) - exp(-p))
}

/// Hyperbolic cosine function.
#[inline]
pub fn cosh(p: f32) -> f32 {
    0.5_f32 * (exp(p) + exp(-p))
}

/// Hyperbolic tangent function.
#[inline]
pub fn tanh(p: f32) -> f32 {
    -1.0_f32 + 2.0_f32 / (1.0_f32 + exp(-2.0_f32 * p))
}

/// Lambert W function.
#[inline]
pub fn lambertw(x: f32) -> f32 {
    const THRESHOLD: f32 = 2.26445;

    let c = if x < THRESHOLD {
        1.546865557_f32
    } else {
        1.0_f32
    };
    let d = if x < THRESHOLD {
        2.250366841_f32
    } else {
        0.0_f32
    };
    let a = if x < THRESHOLD {
        -0.737769969_f32
    } else {
        0.0_f32
    };

    let logterm = ln(c.mul_add(x, d));
    let loglogterm = ln(logterm);

    let minusw = -a - logterm + loglogterm - loglogterm / logterm;
    let expminusw = exp(minusw);
    let xexpminusw = x * expminusw;
    let pexpminusw = xexpminusw - minusw;

    2.0_f32.mul_add(
        xexpminusw,
        -minusw * 4.0_f32.mul_add(xexpminusw, -minusw * pexpminusw),
    ) / pexpminusw.mul_add(2.0_f32 - minusw, 2.0_f32)
}

/// Exponent of Lambert W function.
#[inline]
pub fn lambertwexpx(x: f32) -> f32 {
    const K: f32 = 1.1765631309;
    const A: f32 = 0.94537622168;

    let logarg = x.max(K);
    let powarg = if x < K { A * (x - K) } else { 0.0_f32 };

    let logterm = ln(logarg);
    let powterm = faster::pow2(powarg); // don't need accuracy here

    let w = powterm * (logarg - logterm + logterm / logarg);
    let logw = ln(w);
    let p = x - logw;

    w * w.mul_add(p.mul_add(2.0_f32, 3.0_f32), 2.0_f32 + p)
        / w.mul_add(w.mul_add(2.0_f32, 5.0_f32), 2.0_f32 - p)
}

/// Sine of a number in \[-π, π\], in radians.
#[inline]
pub fn sin(x: f32) -> f32 {
    const FOUROVERPI: f32 = 1.2732395447351627;
    const FOUROVERPISQ: f32 = 0.40528473456935109;
    const Q: f32 = 0.78444488374548933;

    let mut p = to_bits(0.20363937680730309_f32);
    let mut r = to_bits(0.015124940802184233_f32);
    let mut s = to_bits(-0.0032225901625579573_f32);

    let mut v = to_bits(x);
    let sign = v & 0x80000000;
    v &= 0x7FFFFFFF;

    let qpprox = FOUROVERPI.mul_add(x, -FOUROVERPISQ * x * from_bits(v));
    let qpproxsq = qpprox * qpprox;

    p |= sign;
    r |= sign;
    s ^= sign;

    Q.mul_add(
        qpprox,
        qpproxsq * qpproxsq.mul_add(qpproxsq.mul_add(from_bits(s), from_bits(r)), from_bits(p)),
    )
}

/// Sine in radians.
///
/// The range reduction technique used here will be hopelessly inaccurate for |x| >> 1000.
#[inline]
pub fn sinfull(x: f32) -> f32 {
    const TWOPI: f32 = 6.2831853071795865;
    const INVTWOPI: f32 = 0.15915494309189534;

    let k: u32 = (x * INVTWOPI) as u32;
    let half = if x < 0_f32 { -0.5_f32 } else { 0.5_f32 };
    sin((half + (k as f32)).mul_add(TWOPI, -x))
}

/// Cosine of a number in \[-π, π\], in radians.
///
/// # Examples
///
/// ```
/// assert_eq!(f32::cos(1.0), 0.5403023);
/// assert_eq!(fastapprox::fast::cos(1.0), 0.54029506);
/// ```
#[inline]
pub fn cos(x: f32) -> f32 {
    const HALFPI: f32 = 1.5707963267948966;
    const HALFPIMINUSTWOPI: f32 = -4.7123889803846899;
    let offset = if x > HALFPI { HALFPIMINUSTWOPI } else { HALFPI };
    sin(x + offset)
}

/// Cosine in radians.
///
/// The range reduction technique used here will be hopelessly inaccurate for |x| >> 1000.
///
/// # Examples
///
/// ```
/// assert_eq!(f32::cos(10.0), -0.8390715);
/// assert_eq!(fastapprox::fast::cosfull(10.0), -0.83908);
/// ```
#[inline]
pub fn cosfull(x: f32) -> f32 {
    const HALFPI: f32 = 1.5707963267948966;
    sinfull(x + HALFPI)
}

/// Tangent of a number in \[-π/2, π/2\], in radians.
#[inline]
pub fn tan(x: f32) -> f32 {
    const HALFPI: f32 = 1.5707963267948966;
    sin(x) / sin(x + HALFPI)
}

/// Tangent in radians.
///
/// The range reduction technique used here will be hopelessly inaccurate for |x| >> 1000.
#[inline]
pub fn tanfull(x: f32) -> f32 {
    const TWOPI: f32 = 6.2831853071795865;
    const INVTWOPI: f32 = 0.15915494309189534;

    let k: u32 = (x * INVTWOPI) as u32;
    let half = if x < 0_f32 { -0.5_f32 } else { 0.5_f32 };
    let xnew = (half + k as f32).mul_add(-TWOPI, x);

    sin(xnew) / cos(xnew)
}
