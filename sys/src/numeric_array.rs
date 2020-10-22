// from WolframNumericArrayLibrary.h

use crate::{errcode_t, mint, mreal};
use std::os::raw;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct st_MNumericArray {
    _unused: [u8; 0],
}
pub type MNumericArray = *mut st_MNumericArray;

pub const MNumericArray_Type_Undef: MNumericArray_Data_Type = 0;
pub const MNumericArray_Type_Bit8: MNumericArray_Data_Type = 1;
pub const MNumericArray_Type_UBit8: MNumericArray_Data_Type = 2;
pub const MNumericArray_Type_Bit16: MNumericArray_Data_Type = 3;
pub const MNumericArray_Type_UBit16: MNumericArray_Data_Type = 4;
pub const MNumericArray_Type_Bit32: MNumericArray_Data_Type = 5;
pub const MNumericArray_Type_UBit32: MNumericArray_Data_Type = 6;
pub const MNumericArray_Type_Bit64: MNumericArray_Data_Type = 7;
pub const MNumericArray_Type_UBit64: MNumericArray_Data_Type = 8;
pub const MNumericArray_Type_Real32: MNumericArray_Data_Type = 9;
pub const MNumericArray_Type_Real64: MNumericArray_Data_Type = 10;
pub const MNumericArray_Type_Complex_Real32: MNumericArray_Data_Type = 11;
pub const MNumericArray_Type_Complex_Real64: MNumericArray_Data_Type = 12;
pub type MNumericArray_Data_Type = raw::c_int;
pub use MNumericArray_Data_Type as numericarray_data_t;

pub const MNumericArray_Convert_Check: MNumericArray_Convert_Method = 1;
pub const MNumericArray_Convert_Clip_Check: MNumericArray_Convert_Method = 2;
pub const MNumericArray_Convert_Coerce: MNumericArray_Convert_Method = 3;
pub const MNumericArray_Convert_Clip_Coerce: MNumericArray_Convert_Method = 4;
pub const MNumericArray_Convert_Round: MNumericArray_Convert_Method = 5;
pub const MNumericArray_Convert_Clip_Round: MNumericArray_Convert_Method = 6;
pub const MNumericArray_Convert_Scale: MNumericArray_Convert_Method = 7;
pub const MNumericArray_Convert_Clip_Scale: MNumericArray_Convert_Method = 8;
pub type MNumericArray_Convert_Method = raw::c_int;
pub use MNumericArray_Convert_Method as numericarray_convert_method_t;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct st_WolframNumericArrayLibrary_Functions {
    pub MNumericArray_new: Option<
        unsafe extern "C" fn(
            arg1: numericarray_data_t,
            arg2: mint,
            arg3: *const mint,
            arg4: *mut MNumericArray,
        ) -> errcode_t,
    >,
    pub MNumericArray_free: Option<unsafe extern "C" fn(arg1: MNumericArray)>,
    pub MNumericArray_clone:
        Option<unsafe extern "C" fn(arg1: MNumericArray, arg2: *mut MNumericArray) -> errcode_t>,
    pub MNumericArray_disown: Option<unsafe extern "C" fn(arg1: MNumericArray)>,
    pub MNumericArray_disownAll: Option<unsafe extern "C" fn(arg1: MNumericArray)>,
    pub MNumericArray_shareCount: Option<unsafe extern "C" fn(arg1: MNumericArray) -> mint>,
    pub MNumericArray_getType:
        Option<unsafe extern "C" fn(arg1: MNumericArray) -> numericarray_data_t>,
    pub MNumericArray_getRank: Option<unsafe extern "C" fn(arg1: MNumericArray) -> mint>,
    pub MNumericArray_getDimensions:
        Option<unsafe extern "C" fn(arg1: MNumericArray) -> *const mint>,
    pub MNumericArray_getFlattenedLength: Option<unsafe extern "C" fn(arg1: MNumericArray) -> mint>,
    pub MNumericArray_getData:
        Option<unsafe extern "C" fn(arg1: MNumericArray) -> *mut raw::c_void>,
    pub MNumericArray_convertType: Option<
        unsafe extern "C" fn(
            arg1: *mut MNumericArray,
            arg2: MNumericArray,
            arg3: numericarray_data_t,
            arg4: numericarray_convert_method_t,
            arg5: mreal,
        ) -> errcode_t,
    >,
}
pub type WolframNumericArrayLibrary_Functions = *mut st_WolframNumericArrayLibrary_Functions;
