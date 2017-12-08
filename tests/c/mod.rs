macro_rules! extern_unsafe_wraps {
    (
        $( $fn:ident, )*
    ) => {
        mod cc {
            extern {
                $(
                    pub fn $fn(x: f32) -> f32;
                )*
            }
        }

        $(
            #[inline]
            pub fn $fn(x: f32) -> f32 {
                unsafe { cc::$fn(x) }
            }
        )*
    }
}

extern_unsafe_wraps! {
    // fast
    fastpow2,
    fastlog2,
    fastlog,
    fastexp,
    fastsigmoid,
    fastlgamma,
    fastdigamma,
    fasterf,
    fasterfc,
    fastinverseerf,
    fastsinh,
    fastcosh,
    fasttanh,
    fastlambertw,
    fastlambertwexpx,
    fastsin,
    fastcos,
    fasttan,
    fastsinfull,
    fastcosfull,
    fasttanfull,
    // faster
    fasterpow2,
    fasterlog2,
    fasterlog,
    fasterexp,
    fastersigmoid,
    fasterlgamma,
    fasterdigamma,
    fastererf,
    fastererfc,
    fasterinverseerf,
    fastersinh,
    fastercosh,
    fastertanh,
    fasterlambertw,
    fasterlambertwexpx,
    fastersin,
    fastercos,
    fastertan,
    fastersinfull,
    fastercosfull,
    fastertanfull,
}
