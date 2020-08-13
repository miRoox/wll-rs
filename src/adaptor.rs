use crate::errors::{Error, ErrorKind};
use crate::Result;
use std::convert::TryInto;
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
pub trait MArgumentSetter<T: MType>: Sized {
    /// Try to set `MArgument`.
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
        Ok(input != (wll_sys::False as Self::Input))
    }
}

impl OutputAdaptor for bool {
    type Output = mbool;

    #[inline]
    fn try_into_mtype(self) -> Result<Self::Output> {
        Ok(if self { wll_sys::True } else { wll_sys::False } as Self::Output)
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
