//! Complex numbers.

use crate::adaptor::{InputAdaptor, OutputAdaptor};
use crate::Result;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::fmt::Debug;
use sys::{mcomplex, mreal};

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
    pub const fn new(re: T, im: T) -> Self {
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

// Operator Assign

impl<T> AddAssign for Complex<T>
where
    T: AddAssign,
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl<T> SubAssign for Complex<T>
where
    T: SubAssign,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.re -= rhs.re;
        self.im -= rhs.im;
    }
}

impl<T> MulAssign for Complex<T>
where
    T: Clone + AddAssign + SubAssign + MulAssign,
{
    #[allow(clippy::suspicious_op_assign_impl)]
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        let mut imim = self.im.clone();
        imim *= rhs.im.clone();
        self.re *= rhs.re.clone();
        self.re -= imim;
        let mut reim = self.re.clone();
        reim *= rhs.im;
        self.im *= rhs.re;
        self.im += reim;
    }
}

impl<T> DivAssign for Complex<T>
where
    T: Clone + AddAssign + SubAssign + MulAssign + DivAssign,
{
    #[allow(clippy::suspicious_op_assign_impl)]
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        let mut rhs_im2 = rhs.im.clone();
        rhs_im2 *= rhs.im.clone();
        let mut deno = rhs.re.clone();
        deno *= rhs.re.clone();
        deno += rhs_im2;
        let mut imim = self.im.clone();
        imim *= rhs.im.clone();
        self.re *= rhs.re.clone();
        self.re += imim;
        self.re /= deno.clone();
        let mut reim = self.re.clone();
        reim *= rhs.im;
        self.im *= rhs.re;
        self.im -= reim;
        self.im /= deno;
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-10;

    #[test]
    fn convert_int() {
        assert_eq!(Complex::from(1), Complex::new(1, 0));
    }

    #[test]
    fn convert_real() {
        assert_eq!(Complex::from(1.), Complex::new(1., 0.));
    }

    #[test]
    fn norm_sqr_int() {
        assert_eq!(Complex::new(-5, 12).norm_sqr(), 169);
    }

    #[test]
    fn norm_sqr_real() {
        assert!((Complex::new(3.0, -4.0).norm_sqr() - 25f64).abs() <= EPS)
    }

    #[test]
    fn conj_int() {
        assert_eq!(Complex::new(-2, 1).conj(), Complex::new(-2, -1));
    }

    #[test]
    fn conj_real() {
        let a = Complex::new(1.2, -3.1);
        let ca = a.conj();
        let b = Complex::new(1.2f64, 3.1f64);
        assert!((ca.re - b.re).abs() <= EPS && (ca.im - b.im).abs() <= EPS)
    }

    #[test]
    fn add_complex_int() {
        let a = Complex::new(2, 1);
        let b = Complex::new(3, -7);
        let c = Complex::new(5, -6);
        assert_eq!(a + b, c);
    }

    #[test]
    fn add_complex_real() {
        let a = Complex::new(0.1, -3.7);
        let b = Complex::new(2.4, 1.48);
        let c = Complex::new(2.5f64, -2.22f64);
        let ab = a + b;
        assert!((ab.re - c.re).abs() <= EPS && (ab.im - c.im).abs() <= EPS);
    }

    #[test]
    fn sub_complex_int() {
        let a = Complex::new(2, 1);
        let b = Complex::new(3, -7);
        let c = Complex::new(-1, 8);
        assert_eq!(a - b, c);
    }

    #[test]
    fn sub_complex_real() {
        let a = Complex::new(0.1, -3.7);
        let b = Complex::new(2.4, 1.48);
        let c = Complex::new(-2.3f64, -5.18f64);
        let ab = a - b;
        assert!((ab.re - c.re).abs() <= EPS && (ab.im - c.im).abs() <= EPS);
    }

    #[test]
    fn mul_complex_int() {
        let a = Complex::new(2, 1);
        let b = Complex::new(3, -7);
        let c = Complex::new(13, -11);
        assert_eq!(a * b, c);
    }

    #[test]
    fn mul_complex_real() {
        let a = Complex::new(0.1, -3.7);
        let b = Complex::new(2.4, 1.48);
        let c = Complex::new(5.716f64, -8.732f64);
        let ab = a * b;
        assert!((ab.re - c.re).abs() <= EPS && (ab.im - c.im).abs() <= EPS);
    }

    #[test]
    fn div_complex_int() {
        let a = Complex::new(2, 1);
        let b = Complex::new(3, -7);
        let c = Complex::new(0, -3);
        assert_eq!(b / a, c);
    }

    #[test]
    fn div_complex_real() {
        let a = Complex::new(0.5, -1.2);
        let b = Complex::new(4.6, -0.9);
        let c = Complex::new(2f64, 3f64);
        let ab = b / a;
        assert!((ab.re - c.re).abs() <= EPS && (ab.im - c.im).abs() <= EPS);
    }
}
