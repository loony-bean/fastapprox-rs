/// Base 2 logarithm.
#[inline]
pub fn log2(x: f32) -> f32 {
    let mut y = x.to_bits() as f32;
    y *= 1.1920928955078125e-7_f32;
    y - 126.94269504_f32
}

/// Natural logarithm.
#[inline]
pub fn ln(x: f32) -> f32 {
    let mut y = x.to_bits() as f32;
    y *= 8.2629582881927490e-8_f32;
    y - 87.989971088_f32
}

/// Raises 2 to a floating point power.
#[inline]
pub fn pow2(p: f32) -> f32 {
    let clipp = if p < -126.0 { -126.0_f32 } else { p };
    let v = ((1 << 23) as f32 * (clipp + 126.94269504_f32)) as u32;
    f32::from_bits(v)
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
    -0.0810614667_f32 - x - ln(x) + (0.5_f32 + x) * ln(1.0_f32 + x)
}

/// Digamma function.
///
/// Only works for positive values.
#[inline]
pub fn digamma(x: f32) -> f32 {
    let onepx = 1.0_f32 + x;
    -1.0_f32 / x - 1.0_f32 / (2.0_f32 * onepx) + ln(onepx)
}

/// Complementary error function.
#[inline]
pub fn erfc(x: f32) -> f32 {
    const K: f32 = 3.3509633149424609;

    2.0_f32 / (1.0_f32 + pow2(K * x))
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

    INVK * log2((1.0_f32 + x) / (1.0_f32 - x))
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

    let c = if x < THRESHOLD { 1.546865557_f32 } else { 1.0_f32 };
    let d = if x < THRESHOLD { 2.250366841_f32 } else { 0.0_f32 };
    let a = if x < THRESHOLD { -0.737769969_f32 } else { 0.0_f32 };

    let logterm = ln(c * x + d);
    let loglogterm = ln(logterm);

    let w = a + logterm - loglogterm + loglogterm / logterm;
    let expw = exp(-w);

    (w * w + expw * x) / (1.0_f32 + w)
}

/// Exponent of Lambert W function.
#[inline]
pub fn lambertwexpx(x: f32) -> f32 {
    const K: f32 = 1.1765631309;
    const A: f32 = 0.94537622168;

    let logarg = x.max(K);
    let powarg = if x < K { A * (x - K) } else { 0.0_f32 };

    let logterm = ln(logarg);
    let powterm = pow2(powarg);

    let w = powterm * (logarg - logterm + logterm / logarg);
    let logw = ln(w);

    w * (1.0_f32 + x - logw) / (1.0_f32 + w)
}

/// Sine of a number in \[-π, π\], in radians.
#[inline]
pub fn sin(x: f32) -> f32 {
    const FOUROVERPI: f32 = 1.2732395447351627;
    const FOUROVERPISQ: f32 = 0.40528473456935109;
    const Q: f32 = 0.77633023248007499;

    let mut p = 0.22308510060189463_f32.to_bits();
    let mut v = x.to_bits();

    let sign: u32 = v & 0x80000000;
    v &= 0x7FFFFFFF;

    let qpprox = FOUROVERPI * x - FOUROVERPISQ * x * f32::from_bits(v);

    p |= sign;

    qpprox * (Q + f32::from_bits(p) * qpprox)
}

/// Sine in radians.
///
/// The range reduction technique used here will be hopelessly inaccurate for |x| >> 1000.
#[inline]
pub fn sinfull(x: f32) -> f32 {
    const TWOPI: f32 = 6.2831853071795865;
    const INVTWOPI: f32 = 0.15915494309189534;

    let k: i32 = (x * INVTWOPI) as i32;
    let half = if x < 0.0_f32 { -0.5_f32 } else { 0.5_f32 };
    sin((half + (k as f32)) * TWOPI - x)
}

/// Cosine of a number in \[-π, π\], in radians.
///
/// # Examples
///
/// ```
/// assert_eq!(f32::cos(1.0), 0.5403023);
/// assert_eq!(fastapprox::faster::cos(1.0), 0.5357177);
/// ```
#[inline]
pub fn cos(x: f32) -> f32 {
    const TWOOVERPI: f32 = 0.63661977236758134;
    const P: f32 = 0.54641335845679634;

    let v = x.to_bits() & 0x7FFFFFFF;

    let qpprox = 1.0_f32 - TWOOVERPI * f32::from_bits(v);

    qpprox + P * qpprox * (1.0_f32 - qpprox * qpprox)
}

/// Cosine in radians.
///
/// The range reduction technique used here will be hopelessly inaccurate for |x| >> 1000.
///
/// # Examples
///
/// ```
/// assert_eq!(f32::cos(10.0), -0.8390715);
/// assert_eq!(fastapprox::faster::cosfull(10.0), -0.8394889);
/// ```
#[inline]
pub fn cosfull(x: f32) -> f32 {
    const HALFPI: f32 = 1.5707963267948966;
    sinfull(x + HALFPI)
}

/// Tangent of a number in \[-π/2, π/2\], in radians.
#[inline]
pub fn tan(x: f32) -> f32 {
    sin(x) / cos(x)
}

/// Tangent in radians.
///
/// The range reduction technique used here will be hopelessly inaccurate for |x| >> 1000.
#[inline]
pub fn tanfull(x: f32) -> f32 {
    const TWOPI: f32 = 6.2831853071795865;
    const INVTWOPI: f32 = 0.15915494309189534;

    let k: i32 = (x * INVTWOPI) as i32;
    let half = if x < 0.0_f32 { -0.5_f32 } else { 0.5_f32 };
    let xnew = x - (half + (k as f32)) * TWOPI;

    sin(xnew) / cos(xnew)
}
