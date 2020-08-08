use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// The error type for Wolfram LibraryLink.
///
/// **see also**: [Library Structure and Life Cycle: Errors](https://reference.wolfram.com/language/LibraryLink/tutorial/LibraryStructure.html#59563264).
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct WLLError(Repr);

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Repr {
    Simple(ErrorKind),
    Raw(wll_sys::errcode_t),
}

/// A list specifying general categories of Wolfram LibraryLink error.
///
/// It is used with the [`WLLError`] type.
///
/// [`WLLError`]: ../errors/struct.WLLError.html
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ErrorKind {
    /// Unexpected type encountered.
    TypeError = 1,
    /// Unexpected rank encountered.
    RankError,
    /// Inconsistent dimensions encountered.
    DimensionError,
    /// Error in numerical computation.
    NumericalError,
    /// Problem allocating memory.
    MemoryError,
    /// Generic error from a function.
    FunctionError,
    /// Incompatible version.
    VersionError,
}

impl WLLError {
    /// Creates a new instance of an `WLLError` from a raw error code.
    pub fn from_raw_error(code: wll_sys::errcode_t) -> Option<Self> {
        if code == wll_sys::LIBRARY_NO_ERROR {
            None
        } else {
            Some(WLLError(Repr::Raw(code)))
        }
    }

    /// Returns the raw error code that this error represents.
    pub fn to_raw_error(&self) -> wll_sys::errcode_t {
        match self.0 {
            Repr::Simple(kind) => kind.to_raw_error(),
            Repr::Raw(code) => code,
        }
    }

    /// Returns the corresponding [`ErrorKind`] for this error (if any).
    ///
    /// [`ErrorKind`]: ../errors/enum.ErrorKind.html
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
            Simple(TypeError) => write!(f, "unexpected type encountered"),
            Simple(RankError) => write!(f, "unexpected rank encountered "),
            Simple(DimensionError) => write!(f, "inconsistent dimensions encountered"),
            Simple(NumericalError) => write!(f, "error in numerical computation"),
            Simple(MemoryError) => write!(f, "problem allocating memory"),
            Simple(FunctionError) => write!(f, "generic error from a function"),
            Simple(VersionError) => write!(f, "incompatible version"),
            Raw(code) => write!(f, "raw error: {}", code),
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
    pub(crate) fn to_raw_error(&self) -> wll_sys::errcode_t {
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

    pub(crate) fn from_raw_error(code: wll_sys::errcode_t) -> Option<Self> {
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
