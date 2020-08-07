use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub enum WLLError {
    TypeError,
    RankError,
    DimensionError,
    NumericalError,
    MemoryError,
    FunctionError,
    VersionError,
}

impl WLLError {
    pub fn to_raw_error(&self) -> ::std::os::raw::c_int {
        use WLLError::*;
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
}

impl Display for WLLError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use WLLError::*;
        match *self {
            TypeError => write!(f, stringify!(TypeError)),
            RankError => write!(f, stringify!(RankError)),
            DimensionError => write!(f, stringify!(DimensionError)),
            NumericalError => write!(f, stringify!(NumericalError)),
            MemoryError => write!(f, stringify!(MemoryError)),
            FunctionError => write!(f, stringify!(FunctionError)),
            VersionError => write!(f, stringify!(VersionError)),
        }
    }
}

impl Error for WLLError {}
