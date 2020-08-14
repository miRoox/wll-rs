//! Complex numbers.

use crate::adaptor::{InputAdaptor, OutputAdaptor};
use crate::Result;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::fmt::Debug;
use wll_sys::{mcomplex, mreal};

/// Generic complex number.
#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Default)]
pub struct Complex<T> {
    /// Real part.
    pub re: T,
    /// Imaginary part.
    pub im: T,
}

impl<T> Complex<T> {
    /// Construct a new complex number.
    #[inline]
    pub fn new(re: T, im: T) -> Self {
        Complex { re, im }
    }
}

impl<T> From<T> for Complex<T>
where
    T: Default,
{
    #[inline]
    fn from(re: T) -> Self {
        Self::new(re, T::default())
    }
}

impl<'a, T> From<&'a T> for Complex<T>
where
    T: Clone + Default,
{
    #[inline]
    fn from(re: &T) -> Self {
        From::from(re.clone())
    }
}

// -- Unary Operator --

impl<T> Complex<T>
where
    T: Clone + Add<Output = T> + Mul<Output = T>,
{
    /// Returns the square of the norm.
    #[inline]
    pub fn norm_sqr(&self) -> T {
        self.re.clone() * self.re.clone() + self.im.clone() * self.im.clone()
    }
}

impl<T> Complex<T>
where
    T: Clone + Neg<Output = T>,
{
    /// Returns the complex conjugate.
    #[inline]
    pub fn conj(&self) -> Self {
        Self::new(self.re.clone(), -self.im.clone())
    }
}

impl<T> Neg for Complex<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output::new(-self.re, -self.im)
    }
}

impl<'a, T> Neg for &'a Complex<T>
where
    T: Clone + Neg<Output = T>,
{
    type Output = Complex<T>;

    #[inline]
    fn neg(self) -> Self::Output {
        -self.clone()
    }
}

// -- Binary Operator --

impl<T> Add<Complex<T>> for Complex<T>
where
    T: Clone + Add<Output = T>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl<T> Sub<Complex<T>> for Complex<T>
where
    T: Clone + Sub<Output = T>,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.re - rhs.re, self.im - rhs.im)
    }
}

impl<T> Mul<Complex<T>> for Complex<T>
where
    T: Clone + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let re = self.re.clone() * rhs.re.clone() - self.im.clone() * rhs.im.clone();
        let im = self.re * rhs.im + self.im * rhs.re;
        Self::Output::new(re, im)
    }
}

impl<T> Div<Complex<T>> for Complex<T>
where
    T: Clone + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        let deno = rhs.norm_sqr();
        let re = self.re.clone() * rhs.re.clone() + self.im.clone() * rhs.im.clone();
        let im = self.im * rhs.re - self.re * rhs.im;
        Self::Output::new(re / deno.clone(), im / deno)
    }
}

macro_rules! forward_ref_ref_binop {
    (impl $tr:ident => $method:ident where $($reqs:ident),+) => {
        impl<'a, 'b, T> $tr<&'b Complex<T>> for &'a Complex<T>
        where
            T: Clone $(+ $reqs<Output = T>)+,
        {
            type Output = Complex<T>;

            #[inline]
            fn $method(self, rhs: &'b Complex<T>) -> Self::Output {
                self.clone().$method(rhs.clone())
            }
        }
    };
}

macro_rules! forward_val_ref_binop {
    (impl $tr:ident => $method:ident where $($reqs:ident),+) => {
        impl<'a, T> $tr<Complex<T>> for &'a Complex<T>
        where
            T: Clone $(+ $reqs<Output = T>)+,
        {
            type Output = Complex<T>;

            #[inline]
            fn $method(self, rhs: Complex<T>) -> Self::Output {
                self.clone().$method(rhs)
            }
        }
    };
}

macro_rules! forward_ref_val_binop {
    (impl $tr:ident => $method:ident where $($reqs:ident),+) => {
        impl<'a, T> $tr<&'a Complex<T>> for Complex<T>
        where
            T: Clone $(+ $reqs<Output = T>)+,
        {
            type Output = Complex<T>;

            #[inline]
            fn $method(self, rhs: &'a Complex<T>) -> Self::Output {
                self.$method(rhs.clone())
            }
        }
    };
}

macro_rules! forward_all_binop {
    (impl $tr:ident => $method:ident where $($reqs:ident),+) => {
        forward_ref_ref_binop!(impl $tr => $method where $($reqs),+);
        forward_val_ref_binop!(impl $tr => $method where $($reqs),+);
        forward_ref_val_binop!(impl $tr => $method where $($reqs),+);
    };
}

forward_all_binop!(impl Add => add where Add);
forward_all_binop!(impl Sub => sub where Sub);
forward_all_binop!(impl Mul => mul where Add, Sub, Mul);
forward_all_binop!(impl Div => div where Add, Sub, Mul, Div);

// -- Adaptor --

macro_rules! impl_complex_real_adaptor {
    ($($t:ty),+) => {
        $(
            impl InputAdaptor for Complex<$t> {
                type Input = mcomplex;

                #[inline]
                fn mtype_try_from(input: Self::Input) -> Result<Self> {
                    Ok(Self::new(input.ri[0] as $t, input.ri[1] as $t))
                }
            }
            impl OutputAdaptor for Complex<$t> {
                type Output = mcomplex;

                #[inline]
                fn try_into_mtype(self) -> Result<Self::Output> {
                    Ok(Self::Output {
                        ri: [self.re as mreal, self.im as mreal],
                    })
                }
            }
        )+
    };
}

// Note: Complex<integral type> can only be used in output.
macro_rules! impl_complex_int_adaptor {
    ($($t:ty),+) => {
        $(
            impl OutputAdaptor for Complex<$t> {
                type Output = mcomplex;

                #[inline]
                fn try_into_mtype(self) -> Result<Self::Output> {
                    Ok(Self::Output {
                        ri: [self.re as mreal, self.im as mreal],
                    })
                }
            }
        )+
    };
}

impl_complex_real_adaptor!(f32, f64);
impl_complex_int_adaptor!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize);
