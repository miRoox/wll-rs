use crate::num_traits::{ComplexType, Number, RealNumber, RealType};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::fmt::Debug;

/// Generic complex number.
#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Default)]
pub struct Complex<T: RealNumber> {
    /// Real part.
    pub re: T,
    /// Imaginary part.
    pub im: T,
}

impl<T: RealNumber> Complex<T> {
    /// Construct a new complex number.
    #[inline]
    pub fn new(re: T, im: T) -> Self {
        Complex { re, im }
    }

    /// Returns imaginary unit.
    #[inline]
    pub fn i() -> Self {
        Self::new(T::zero(), T::one())
    }
}

impl<T: RealNumber> Number for Complex<T> {
    #[inline]
    fn zero() -> Self {
        Self::new(T::zero(), T::zero())
    }

    #[inline]
    fn one() -> Self {
        Self::new(T::one(), T::zero())
    }
}

impl<T: RealType> Into<wll_sys::mcomplex> for Complex<T> {
    #[inline]
    fn into(self) -> wll_sys::mcomplex {
        wll_sys::mcomplex {
            ri: [self.re.into(), self.im.into()],
        }
    }
}

impl<T: RealType> From<wll_sys::mcomplex> for Complex<T> {
    #[inline]
    fn from(z: wll_sys::mcomplex) -> Self {
        Self::new(T::from(z.ri[0]), T::from(z.ri[1]))
    }
}

impl<T: RealType> ComplexType for Complex<T> {}

impl<T: RealNumber> Complex<T>
where
    T: Add<Output = T> + Mul<Output = T>,
{
    /// Returns the square of the norm.
    #[inline]
    pub fn norm_sqr(&self) -> T {
        self.re.clone() * self.re.clone() + self.im.clone() * self.im.clone()
    }
}

impl<T: RealNumber> Complex<T>
where
    T: Neg<Output = T>,
{
    /// Returns the complex conjugate.
    #[inline]
    pub fn conj(&self) -> Self {
        Self::new(self.re.clone(), -self.im.clone())
    }
}

impl<T: RealNumber> From<T> for Complex<T> {
    #[inline]
    fn from(re: T) -> Self {
        Self::new(re, T::zero())
    }
}

impl<'a, T: RealNumber> From<&'a T> for Complex<T> {
    #[inline]
    fn from(re: &T) -> Self {
        From::from(re.clone())
    }
}

impl<T: RealNumber> Add<Complex<T>> for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl<T: RealNumber> Sub<Complex<T>> for Complex<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.re - rhs.re, self.im - rhs.im)
    }
}

impl<T: RealNumber> Mul<Complex<T>> for Complex<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let re = self.re.clone() * rhs.re.clone() - self.im.clone() * rhs.im.clone();
        let im = self.re * rhs.im + self.im * rhs.re;
        Self::Output::new(re, im)
    }
}

impl<T: RealNumber> Div<Complex<T>> for Complex<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
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
