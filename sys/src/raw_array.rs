// from WolframRawArrayLibrary.h

use crate::{mint, st_MNumericArray};
use std::os::raw;

pub type MRawArray = *mut st_MNumericArray;

pub type MRawArray_Data_Type = raw::c_int;
pub use MRawArray_Data_Type as rawarray_t;

pub const MRawArray_Type_Undef: MRawArray_Data_Type = 0;
pub const MRawArray_Type_Bit8: MRawArray_Data_Type = 1;
pub const MRawArray_Type_Ubit8: MRawArray_Data_Type = 2;
pub const MRawArray_Type_Bit16: MRawArray_Data_Type = 3;
pub const MRawArray_Type_Ubit16: MRawArray_Data_Type = 4;
pub const MRawArray_Type_Bit32: MRawArray_Data_Type = 5;
pub const MRawArray_Type_Ubit32: MRawArray_Data_Type = 6;
pub const MRawArray_Type_Bit64: MRawArray_Data_Type = 7;
pub const MRawArray_Type_Ubit64: MRawArray_Data_Type = 8;
pub const MRawArray_Type_Real32: MRawArray_Data_Type = 9;
pub const MRawArray_Type_Real64: MRawArray_Data_Type = 10;
pub const MRawArray_Type_Float_Complex: MRawArray_Data_Type = 11;
pub const MRawArray_Type_Double_Complex: MRawArray_Data_Type = 12;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct st_WolframRawArrayLibrary_Functions {
    pub MRawArray_new: Option<
        unsafe extern "C" fn(
            arg1: rawarray_t,
            arg2: mint,
            arg3: *const mint,
            arg4: *mut MRawArray,
        ) -> raw::c_int,
    >,
    pub MRawArray_free: Option<unsafe extern "C" fn(arg1: MRawArray)>,
    pub MRawArray_clone:
        Option<unsafe extern "C" fn(arg1: MRawArray, arg2: *mut MRawArray) -> raw::c_int>,
    pub MRawArray_disown: Option<unsafe extern "C" fn(arg1: MRawArray)>,
    pub MRawArray_disownAll: Option<unsafe extern "C" fn(arg1: MRawArray)>,
    pub MRawArray_shareCount: Option<unsafe extern "C" fn(arg1: MRawArray) -> mint>,
    pub MRawArray_getType: Option<unsafe extern "C" fn(arg1: MRawArray) -> rawarray_t>,
    pub MRawArray_getRank: Option<unsafe extern "C" fn(arg1: MRawArray) -> mint>,
    pub MRawArray_getDimensions: Option<unsafe extern "C" fn(arg1: MRawArray) -> *const mint>,
    pub MRawArray_getFlattenedLength: Option<unsafe extern "C" fn(arg1: MRawArray) -> mint>,
    pub MRawArray_getData:
        Option<unsafe extern "C" fn(arg1: MRawArray) -> *mut ::std::os::raw::c_void>,
    pub MRawArray_convertType:
        Option<unsafe extern "C" fn(arg1: MRawArray, arg2: rawarray_t) -> MRawArray>,
}
pub type WolframRawArrayLibrary_Functions = *mut st_WolframRawArrayLibrary_Functions;
