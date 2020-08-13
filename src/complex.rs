use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::fmt::Debug;

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
