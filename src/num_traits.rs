use std::convert::{TryFrom, TryInto};

/// The basic trait for numeric types.
pub trait Number: Clone + PartialEq {
    /// Returns the zero element of `Self`.
    fn zero() -> Self;
    /// Returns the identity element of `Self`.
    fn one() -> Self;
}

/// The basic trait for real numbers.
pub trait RealNumber: Number {}

/// Compatible trait for `wll_sys::mcomplex`.
pub trait ComplexType: Number + TryFrom<wll_sys::mcomplex> + TryInto<wll_sys::mcomplex> {}

/// Compatible trait for `wll_sys::mreal`.
pub trait RealType: RealNumber + Into<wll_sys::mreal> {
    fn from(num: wll_sys::mreal) -> Self;
}

/// Compatible trait for `wll_sys::mint`.
pub trait IntegerType: RealNumber + TryFrom<wll_sys::mint> + TryInto<wll_sys::mint> {}

macro_rules! impl_number_trait {
    ($($t:ty),+) => (
        $(impl Number for $t {
            fn zero() -> Self { 0 as $t }
            fn one() -> Self { 1 as $t }
        })*
    )
}

macro_rules! impl_real_number {
    ($($t:ty),+) => ($(impl RealNumber for $t {})*)
}

macro_rules! impl_real_type {
    ($($t:ty),+) => ($(impl RealType for $t {
        fn from(num: wll_sys::mreal) -> Self { num as $t }
    })*)
}

macro_rules! impl_int_type {
    ($($t:ty),+) => ($(impl IntegerType for $t {})*)
}

impl_number_trait!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64);
impl_real_number!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64);
impl_real_type!(f32, f64);
impl_int_type!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize);
