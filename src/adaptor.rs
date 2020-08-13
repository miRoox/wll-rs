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

    fn try_from(input: Self::Input) -> Result<Self>;
}

pub trait OutputAdaptor: Sized {
    type Output: MType;

    fn try_into(self) -> Result<Self::Output>;
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
                    T::try_from(*ptr)
                }
            }
        }
    };
}

impl<T: OutputAdaptor<Output = mbool>> MArgumentSetter<mbool> for T {
    #[inline]
    fn try_set_arg(arg: &mut MArgument, val: Self) -> Result<()> {
        unsafe {
            let ptr = arg.boolean;
            if ptr.is_null() {
                return Err(Error::from(ErrorKind::TypeError));
            }
            *ptr = val.try_into()?;
        }
        Ok(())
    }
}

impl_argument_getter!(mbool, boolean);
impl_argument_getter!(mint, integer);
impl_argument_getter!(mreal, real);
impl_argument_getter!(mcomplex, cmplex);

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
