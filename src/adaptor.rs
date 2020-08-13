use crate::errors::{Error, ErrorKind};
use crate::Result;
use wll_sys::{mbool, mcomplex, mint, mreal, MArgument};

mod private {
    pub trait Sealed {}
}

/// Basic trait for Wolfram LibraryLink underlying type.
/// Typically doesn’t need to be used directly.
///
/// You **CANNOT** implement it outside.
pub trait MType: private::Sealed + Sized {}

macro_rules! impl_mtypes {
    ($( $t:ty )+) => {
        $(
            impl private::Sealed for $t {}
            impl MType for $t {}
        )+
    };
}

impl_mtypes!(mbool mint mreal mcomplex);

pub trait InputAdaptor: Sized {
    type Input: MType;

    fn mtype_try_from(input: Self::Input) -> Result<Self>;
}

pub trait OutputAdaptor: Sized {
    type Output: MType;

    fn try_into_mtype(self) -> Result<Self::Output>;
}

/// Adaptor trait for getting `MArgument`.
///
/// **DO NOT** implement this trait yourself.
pub trait MArgumentGetter<T: MType>: Sized {
    fn try_get_arg(arg: MArgument) -> Result<Self>;
}

/// Adaptor trait for setting `MArgument`.
///
/// **DO NOT** implement this trait yourself.
pub trait MArgumentSetter<T: MType>: Sized {
    fn try_set_arg(arg: &mut MArgument, val: Self) -> Result<()>;
}

macro_rules! impl_argument_getter {
    ($t:ty, $fd:ident) => {
        impl<T: InputAdaptor<Input = $t>> MArgumentGetter<$t> for T {
            #[inline]
            fn try_get_arg(arg: MArgument) -> Result<Self> {
                unsafe {
                    let ptr = arg.$fd;
                    if ptr.is_null() {
                        return Err(Error::from(ErrorKind::TypeError));
                    }
                    T::mtype_try_from(*ptr)
                }
            }
        }
    };
}

macro_rules! impl_argument_setter {
    ($t:ty, $fd:ident) => {
        impl<T: OutputAdaptor<Output = $t>> MArgumentSetter<$t> for T {
            #[inline]
            fn try_set_arg(arg: &mut MArgument, val: Self) -> Result<()> {
                unsafe {
                    let ptr = arg.$fd;
                    if ptr.is_null() {
                        return Err(Error::from(ErrorKind::TypeError));
                    }
                    *ptr = val.try_into_mtype()?;
                }
                Ok(())
            }
        }
    };
}

impl_argument_getter!(mbool, boolean);
impl_argument_getter!(mint, integer);
impl_argument_getter!(mreal, real);
impl_argument_getter!(mcomplex, cmplex);

impl_argument_setter!(mbool, boolean);
impl_argument_setter!(mint, integer);
impl_argument_setter!(mreal, real);
impl_argument_setter!(mcomplex, cmplex);

impl InputAdaptor for bool {
    type Input = mbool;

    #[inline]
    fn mtype_try_from(input: Self::Input) -> Result<Self> {
        Ok(input != 0)
    }
}

impl OutputAdaptor for bool {
    type Output = mbool;

    #[inline]
    fn try_into_mtype(self) -> Result<Self::Output> {
        Ok(if self { 1 } else { 0 })
    }
}
