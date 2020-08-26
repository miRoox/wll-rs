//! Some adaptor interface for Wolfram LibraryLink.

use crate::errors::{Error, ErrorKind};
use crate::Result;
use std::convert::TryInto;
use std::num::Wrapping;
use sys::{mbool, mcomplex, mint, mreal, MArgument, MImage, MNumericArray, MSparseArray, MTensor};

mod private {
    pub trait Sealed {}
}

/// Basic trait for Wolfram LibraryLink underlying type.
/// Typically doesn’t need to be used directly.
///
/// You **CANNOT** implement it outside.
pub trait MType: private::Sealed + Sized {}

macro_rules! impl_mtypes {
    ($($t:ty),+) => {
        $(
            impl private::Sealed for $t {}
            impl MType for $t {}
        )+
    };
}

impl_mtypes!(
    mbool,
    mint,
    mreal,
    mcomplex,
    MTensor,
    MSparseArray,
    MNumericArray,
    MImage
);

/// [`MType`] or `()`.
/// Typically doesn’t need to be used directly.
///
/// [`MType`]: ./trait.MType.html
pub trait MTypeOrVoid: private::Sealed {}

impl<T: MType> MTypeOrVoid for T {}
impl private::Sealed for () {}
impl MTypeOrVoid for () {}

/// Adaptor for [`MType`] input.
///
/// `InputAdaptor for T` with `type Input = U` implies [`MArgumentGetter`]`<U> for T`
///
/// [`MType`]: ./trait.MType.html
/// [`MArgumentGetter`]: ./trait.MArgumentGetter.html
pub trait InputAdaptor: Sized {
    /// Input type.
    type Input: MType;

    /// Performs the conversion.
    fn mtype_try_from(input: Self::Input) -> Result<Self>;
}

/// Adaptor for [`MType`] output.
///
/// `OutputAdaptor for T` with `type Input = U` implies [`MArgumentSetter`]`<U> for T`
///
/// [`MType`]: ./trait.MType.html
/// [`MArgumentSetter`]: ./trait.MArgumentSetter.html
pub trait OutputAdaptor: Sized {
    /// Output type.
    type Output: MType;

    /// Performs the conversion.
    fn try_into_mtype(self) -> Result<Self::Output>;
}

/// Adaptor trait for getting `MArgument`.
/// Typically doesn’t need to be used directly.
///
/// `MArgumentGetter<T>` will be implemented automatically if proper [`InputAdaptor`] has been implemented.
///
/// **DO NOT** implement this trait yourself.
///
/// [`InputAdaptor`]: ./trait.InputAdaptor.html
pub trait MArgumentGetter<T: MType>: Sized {
    /// Try to get `MArgument`.
    fn try_get_arg(arg: MArgument) -> Result<Self>;
}

/// Adaptor trait for setting `MArgument`.
/// Typically doesn’t need to be used directly.
///
/// `MArgumentSetter<T>` will be implemented automatically if proper [`OutputAdaptor`] has been implemented.
///
/// **DO NOT** implement this trait yourself.
///
/// [`OutputAdaptor`]: ./trait.OutputAdaptor.html
pub trait MArgumentSetter<T: MTypeOrVoid>: Sized {
    /// Try to set `MArgument`.
    fn try_set_arg(self, arg: &MArgument) -> Result<()>;
}

impl MArgumentSetter<()> for () {
    #[inline]
    fn try_set_arg(self, _arg: &MArgument) -> Result<()> {
        Ok(())
    }
}

macro_rules! impl_argument_getter {
    ($fd:ident: $t:ty) => {
        impl<T: InputAdaptor<Input = $t>> MArgumentGetter<$t> for T {
            #[inline]
            fn try_get_arg(arg: MArgument) -> Result<Self> {
                unsafe {
                    let ptr = arg.$fd;
                    if ptr.is_null() {
                        return Err(Error::from(ErrorKind::TypeError));
                    }
                    T::mtype_try_from(std::ptr::read(ptr))
                }
            }
        }
    };
}

macro_rules! impl_argument_setter {
    ($fd:ident: $t:ty) => {
        impl<T: OutputAdaptor<Output = $t>> MArgumentSetter<$t> for T {
            #[inline]
            fn try_set_arg(self, arg: &MArgument) -> Result<()> {
                unsafe {
                    let ptr = arg.$fd;
                    if ptr.is_null() {
                        return Err(Error::from(ErrorKind::TypeError));
                    }
                    std::ptr::write(ptr, self.try_into_mtype()?);
                }
                Ok(())
            }
        }
    };
}

macro_rules! impl_argument_getter_setter {
    {$($fd:ident: $t:ty,)*} => {
        $(
            impl_argument_getter!($fd: $t);
            impl_argument_setter!($fd: $t);
        )*
    };
}

impl_argument_getter_setter! {
    boolean: mbool,
    integer: mint,
    real: mreal,
    cmplex: mcomplex,
    tensor: MTensor,
    sparse: MSparseArray,
    numeric: MNumericArray,
    image: MImage,
}

impl InputAdaptor for bool {
    type Input = mbool;

    #[inline]
    fn mtype_try_from(input: Self::Input) -> Result<Self> {
        Ok(input != sys::False)
    }
}

impl OutputAdaptor for bool {
    type Output = mbool;

    #[inline]
    fn try_into_mtype(self) -> Result<Self::Output> {
        Ok(if self { sys::True } else { sys::False })
    }
}

macro_rules! impl_int_adaptor {
    ($($t:ty),+) => {
        $(
            impl InputAdaptor for $t {
                type Input = mint;

                #[inline]
                fn mtype_try_from(input: Self::Input) -> Result<Self> {
                    input
                        .try_into()
                        .map_err(|_| Error::from(ErrorKind::TypeError))
                }
            }
            impl OutputAdaptor for $t {
                type Output = mint;

                #[inline]
                fn try_into_mtype(self) -> Result<Self::Output> {
                    self.try_into()
                        .map_err(|_| Error::from(ErrorKind::TypeError))
                }
            }
        )+
    }
}

impl_int_adaptor!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize);

macro_rules! impl_wrapping_int_adaptor {
    ($($t:ty),+) => {
        $(
            impl InputAdaptor for Wrapping<$t> {
                type Input = mint;

                #[inline]
                fn mtype_try_from(input: Self::Input) -> Result<Self> {
                    Ok(Wrapping(<$t>::mtype_try_from(input)?))
                }
            }
            impl OutputAdaptor for Wrapping<$t> {
                type Output = mint;

                #[inline]
                fn try_into_mtype(self) -> Result<Self::Output> {
                    self.0.try_into_mtype()
                }
            }
        )+
    };
}

impl_wrapping_int_adaptor!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize);

macro_rules! impl_real_adaptor {
    ($($t:ty),+) => {
        $(
            impl InputAdaptor for $t {
                type Input = mreal;

                fn mtype_try_from(input: Self::Input) -> Result<Self> {
                    Ok(input as Self)
                }
            }
            impl OutputAdaptor for $t {
                type Output = mreal;

                fn try_into_mtype(self) -> Result<Self::Output> {
                    Ok(self as Self::Output)
                }
            }
        )+
    };
}

impl_real_adaptor!(f32, f64);

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::MaybeUninit;
    use sys::{mbool, MArgument};

    #[test]
    fn bool_true_input() {
        assert_eq!(bool::mtype_try_from(sys::True), Ok(true));
    }

    #[test]
    fn bool_false_input() {
        assert_eq!(bool::mtype_try_from(sys::False), Ok(false));
    }

    #[test]
    fn bool_true_output() {
        assert_eq!(true.try_into_mtype(), Ok(sys::True));
    }

    #[test]
    fn bool_false_output() {
        assert_eq!(false.try_into_mtype(), Ok(sys::False));
    }

    #[test]
    fn bool_true_get() {
        let mut mb = sys::True;
        let arg = MArgument { boolean: &mut mb };
        assert_eq!(bool::try_get_arg(arg), Ok(true));
    }

    #[test]
    fn bool_false_get() {
        let mut mb = sys::False;
        let arg = MArgument { boolean: &mut mb };
        assert_eq!(bool::try_get_arg(arg), Ok(false));
    }

    #[test]
    fn bool_true_set() {
        let mut mb = MaybeUninit::<mbool>::uninit();
        let arg = MArgument {
            boolean: mb.as_mut_ptr(),
        };
        let res = true.try_set_arg(&arg);
        let mb = unsafe { mb.assume_init() };
        assert_eq!((mb, res), (sys::True, Ok(())));
    }

    #[test]
    fn bool_false_set() {
        let mut mb = MaybeUninit::<mbool>::uninit();
        let arg = MArgument {
            boolean: mb.as_mut_ptr(),
        };
        let res = false.try_set_arg(&arg);
        let mb = unsafe { mb.assume_init() };
        assert_eq!((mb, res), (sys::False, Ok(())));
    }
}
