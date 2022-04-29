//! Constants for all SI prefixes as `Quad`s
use qd::{Quad, qd};
use lazy_static::lazy_static;

lazy_static! {
    /// The SI prefix for 10^24
    pub static ref YOTTA: Quad =qd!( 1e24);
    /// The SI prefix for 10^21
    pub static ref ZETTA: Quad =qd!( 1e21);
    /// The SI prefix for 10^18
    pub static ref EXA: Quad   =qd!( 1e18);
    /// The SI prefix for 10^15
    pub static ref PETA: Quad  =qd!( 1e15);
    /// The SI prefix for 10^12
    pub static ref TERA: Quad  =qd!( 1e12);
    /// The SI prefix for 10^9
    pub static ref GIGA: Quad  =qd!( 1e9);
    /// The SI prefix for 10^6
    pub static ref MEGA: Quad  =qd!( 1e6);
    /// The SI prefix for 10^3
    pub static ref KILO: Quad  =qd!( 1e3);
    /// The SI prefix for 10^2
    pub static ref HECTO: Quad =qd!( 1e2);
    /// The SI prefix for 10^1
    pub static ref DECA: Quad  =qd!( 1e1);

    /// The SI prefix for 10^-1
    pub static ref DECI: Quad  =qd!( 1e-1);
    /// The SI prefix for 10^-2
    pub static ref CENTI: Quad =qd!( 1e-2);
    /// The SI prefix for 10^-3
    pub static ref MILLI: Quad =qd!( 1e-3);
    /// The SI prefix for 10^-6
    pub static ref MICRO: Quad =qd!( 1e-6);
    /// The SI prefix for 10^-9
    pub static ref NANO: Quad  =qd!( 1e-9);
    /// The SI prefix for 10^-12
    pub static ref PICO: Quad  =qd!( 1e-12);
    /// The SI prefix for 10^-15
    pub static ref FEMTO: Quad =qd!( 1e-15);
    /// The SI prefix for 10^-18
    pub static ref ATTO: Quad  =qd!( 1e-18);
    /// The SI prefix for 10^-21
    pub static ref ZEPTO: Quad =qd!( 1e-21);
    /// The SI prefix for 10^-24
    pub static ref YOCTO: Quad =qd!( 1e-24);
}

