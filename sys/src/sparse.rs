// from WolframSparseLibrary.h

use crate::{mint, MTensor};
use std::os::raw;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct MSparseArray_struct {
    _unused: [u8; 0],
}
pub type MSparseArray = *mut MSparseArray_struct;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct st_WolframSparseLibrary_Functions {
    pub MSparseArray_clone:
        Option<unsafe extern "C" fn(arg1: MSparseArray, arg2: *mut MSparseArray) -> raw::c_int>,
    pub MSparseArray_free: Option<unsafe extern "C" fn(arg1: MSparseArray)>,
    pub MSparseArray_disown: Option<unsafe extern "C" fn(arg1: MSparseArray)>,
    pub MSparseArray_disownAll: Option<unsafe extern "C" fn(arg1: MSparseArray)>,
    pub MSparseArray_shareCount: Option<unsafe extern "C" fn(arg1: MSparseArray) -> mint>,
    pub MSparseArray_getRank: Option<unsafe extern "C" fn(arg1: MSparseArray) -> mint>,
    pub MSparseArray_getDimensions: Option<unsafe extern "C" fn(arg1: MSparseArray) -> *const mint>,
    pub MSparseArray_getImplicitValue:
        Option<unsafe extern "C" fn(arg1: MSparseArray) -> *mut MTensor>,
    pub MSparseArray_getExplicitValues:
        Option<unsafe extern "C" fn(arg1: MSparseArray) -> *mut MTensor>,
    pub MSparseArray_getRowPointers:
        Option<unsafe extern "C" fn(arg1: MSparseArray) -> *mut MTensor>,
    pub MSparseArray_getColumnIndices:
        Option<unsafe extern "C" fn(arg1: MSparseArray) -> *mut MTensor>,
    pub MSparseArray_getExplicitPositions:
        Option<unsafe extern "C" fn(arg1: MSparseArray, arg2: *mut MTensor) -> raw::c_int>,
    pub MSparseArray_resetImplicitValue: Option<
        unsafe extern "C" fn(
            arg1: MSparseArray,
            arg2: MTensor,
            arg3: *mut MSparseArray,
        ) -> raw::c_int,
    >,
    pub MSparseArray_toMTensor:
        Option<unsafe extern "C" fn(arg1: MSparseArray, arg2: *mut MTensor) -> raw::c_int>,
    pub MSparseArray_fromMTensor: Option<
        unsafe extern "C" fn(arg1: MTensor, arg2: MTensor, arg3: *mut MSparseArray) -> raw::c_int,
    >,
    pub MSparseArray_fromExplicitPositions: Option<
        unsafe extern "C" fn(
            arg1: MTensor,
            arg2: MTensor,
            arg3: MTensor,
            arg4: MTensor,
            arg5: *mut MSparseArray,
        ) -> raw::c_int,
    >,
}
pub type WolframSparseLibrary_Functions = *mut st_WolframSparseLibrary_Functions;
