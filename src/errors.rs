use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::os::raw;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct WLLError(Repr);

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Repr {
    Simple(ErrorKind),
    Raw(/*code*/ raw::c_int),
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ErrorKind {
    TypeError = 1,
    RankError,
    DimensionError,
    NumericalError,
    MemoryError,
    FunctionError,
    VersionError,
}

impl WLLError {
    pub fn from_raw_error(code: raw::c_int) -> Option<Self> {
        if code == wll_sys::LIBRARY_NO_ERROR {
            None
        } else {
            Some(WLLError(Repr::Raw(code)))
        }
    }

    pub fn to_raw_error(&self) -> raw::c_int {
        match self.0 {
            Repr::Simple(kind) => kind.to_raw_error(),
            Repr::Raw(code) => code,
        }
    }

    pub fn kind(&self) -> Option<ErrorKind> {
        match self.0 {
            Repr::Simple(kind) => Some(kind),
            Repr::Raw(code) => ErrorKind::from_raw_error(code),
        }
    }
}

impl Display for WLLError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use ErrorKind::*;
        use Repr::*;
        match self.0 {
            Simple(TypeError) => write!(f, stringify!(TypeError)),
            Simple(RankError) => write!(f, stringify!(RankError)),
            Simple(DimensionError) => write!(f, stringify!(DimensionError)),
            Simple(NumericalError) => write!(f, stringify!(NumericalError)),
            Simple(MemoryError) => write!(f, stringify!(MemoryError)),
            Simple(FunctionError) => write!(f, stringify!(FunctionError)),
            Simple(VersionError) => write!(f, stringify!(VersionError)),
            Raw(code) => write!(f, "RawError{{code={}}}", code),
        }
    }
}

impl Error for WLLError {}

impl From<ErrorKind> for WLLError {
    fn from(e: ErrorKind) -> Self {
        WLLError(Repr::Simple(e))
    }
}

impl ErrorKind {
    pub(crate) fn to_raw_error(&self) -> raw::c_int {
        use ErrorKind::*;
        match *self {
            TypeError => wll_sys::LIBRARY_TYPE_ERROR,
            RankError => wll_sys::LIBRARY_RANK_ERROR,
            DimensionError => wll_sys::LIBRARY_DIMENSION_ERROR,
            NumericalError => wll_sys::LIBRARY_NUMERICAL_ERROR,
            MemoryError => wll_sys::LIBRARY_MEMORY_ERROR,
            FunctionError => wll_sys::LIBRARY_FUNCTION_ERROR,
            VersionError => wll_sys::LIBRARY_VERSION_ERROR,
        }
    }

    pub(crate) fn from_raw_error(code: raw::c_int) -> Option<Self> {
        use ErrorKind::*;
        match code {
            wll_sys::LIBRARY_NO_ERROR => unreachable!(),
            wll_sys::LIBRARY_TYPE_ERROR => Some(TypeError),
            wll_sys::LIBRARY_RANK_ERROR => Some(RankError),
            wll_sys::LIBRARY_DIMENSION_ERROR => Some(DimensionError),
            wll_sys::LIBRARY_NUMERICAL_ERROR => Some(NumericalError),
            wll_sys::LIBRARY_MEMORY_ERROR => Some(MemoryError),
            wll_sys::LIBRARY_FUNCTION_ERROR => Some(FunctionError),
            wll_sys::LIBRARY_VERSION_ERROR => Some(VersionError),
            _ => None,
        }
    }
}
