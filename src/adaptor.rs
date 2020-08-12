use crate::errors::{Error, ErrorKind};
use crate::Result;
use wll_sys::{mbool, mcomplex, mint, mreal, MArgument};

pub trait InputAdaptor: Sized {
    type Input;

    fn try_from(input: Self::Input) -> Result<Self>;
}

pub trait OutputAdaptor: Sized {
    type Output;

    fn try_into(self) -> Result<Self::Output>;
}

/// Adaptor trait for getting `MArgument`.
///
/// **DO NOT** implement this trait yourself.
pub trait MArgumentGetter<T>: Sized {
    fn try_get_arg(arg: MArgument) -> Result<Self>;
}

/// Adaptor trait for setting `MArgument`.
///
/// **DO NOT** implement this trait yourself.
pub trait MArgumentSetter<T>: Sized {
    fn try_set_arg(arg: &mut MArgument, val: Self) -> Result<()>;
}

impl<T: InputAdaptor<Input = mbool>> MArgumentGetter<mbool> for T {
    #[inline]
    fn try_get_arg(arg: MArgument) -> Result<Self> {
        unsafe {
            let ptr = arg.boolean;
            if ptr.is_null() {
                return Err(Error::from(ErrorKind::TypeError));
            }
            T::try_from(*ptr)
        }
    }
}

impl<T: InputAdaptor<Input = mint>> MArgumentGetter<mint> for T {
    #[inline]
    fn try_get_arg(arg: MArgument) -> Result<Self> {
        unsafe {
            let ptr = arg.integer;
            if ptr.is_null() {
                return Err(Error::from(ErrorKind::TypeError));
            }
            T::try_from(*ptr)
        }
    }
}

impl<T: InputAdaptor<Input = mreal>> MArgumentGetter<mreal> for T {
    #[inline]
    fn try_get_arg(arg: MArgument) -> Result<Self> {
        unsafe {
            let ptr = arg.real;
            if ptr.is_null() {
                return Err(Error::from(ErrorKind::TypeError));
            }
            T::try_from(*ptr)
        }
    }
}

impl<T: InputAdaptor<Input = mcomplex>> MArgumentGetter<mcomplex> for T {
    #[inline]
    fn try_get_arg(arg: MArgument) -> Result<Self> {
        unsafe {
            let ptr = arg.cmplex;
            if ptr.is_null() {
                return Err(Error::from(ErrorKind::TypeError));
            }
            T::try_from(*ptr)
        }
    }
}

impl InputAdaptor for bool {
    type Input = mbool;

    #[inline]
    fn try_from(input: Self::Input) -> Result<Self> {
        Ok(input != 0)
    }
}

impl OutputAdaptor for bool {
    type Output = mbool;

    #[inline]
    fn try_into(self) -> Result<Self::Output> {
        Ok(if self { 1 } else { 0 })
    }
}
