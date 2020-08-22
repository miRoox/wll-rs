//! Wolfram LibraryLink errors.

use std::fmt::{self, Display, Formatter};
use wll_sys::errcode_t;

/// The error type for Wolfram LibraryLink.
///
/// **see also**: [Library Structure and Life Cycle: Errors](https://reference.wolfram.com/language/LibraryLink/tutorial/LibraryStructure.html#59563264).
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Error(Repr);

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Repr {
    Simple(ErrorKind),
    Raw(errcode_t),
}

/// A list specifying general categories of Wolfram LibraryLink error.
///
/// It is used with the [`Error`] type.
///
/// [`Error`]: ../struct.Error.html
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

impl Error {
    /// Creates a new instance of an `Error` from a raw error code.
    #[inline]
    pub fn from_raw_error(code: errcode_t) -> Option<Self> {
        if code == wll_sys::LIBRARY_NO_ERROR {
            None
        } else {
            Some(Error(Repr::Raw(code)))
        }
    }

    /// Returns the raw error code that this error represents.
    #[inline]
    pub fn to_raw_error(&self) -> errcode_t {
        match self.0 {
            Repr::Simple(kind) => kind.to_raw_error(),
            Repr::Raw(code) => code,
        }
    }

    /// Returns the corresponding [`ErrorKind`] for this error (if any).
    ///
    /// [`ErrorKind`]: ../enum.ErrorKind.html
    #[inline]
    pub fn kind(&self) -> Option<ErrorKind> {
        match self.0 {
            Repr::Simple(kind) => Some(kind),
            Repr::Raw(code) => ErrorKind::from_raw_error(code),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Repr::*;
        match self.0 {
            Simple(kind) => write!(f, "{}", kind),
            Raw(code) => {
                if let Some(kind) = ErrorKind::from_raw_error(code) {
                    write!(f, "{}", kind)
                } else {
                    write!(f, "unknown error code: {}", code)
                }
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<ErrorKind> for Error {
    #[inline]
    fn from(e: ErrorKind) -> Self {
        Error(Repr::Simple(e))
    }
}

impl ErrorKind {
    #[inline]
    pub(crate) fn to_raw_error(&self) -> errcode_t {
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

    #[inline]
    pub(crate) fn from_raw_error(code: errcode_t) -> Option<Self> {
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

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use ErrorKind::*;
        match *self {
            TypeError => write!(f, "unexpected type encountered"),
            RankError => write!(f, "unexpected rank encountered "),
            DimensionError => write!(f, "inconsistent dimensions encountered"),
            NumericalError => write!(f, "error in numerical computation"),
            MemoryError => write!(f, "problem allocating memory"),
            FunctionError => write!(f, "generic error from a function"),
            VersionError => write!(f, "incompatible version"),
        }
    }
}
