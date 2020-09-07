use std::os::raw;

pub type errcode_t = raw::c_int;
pub type mint = isize;
pub type umint = usize;
pub type mreal = f64;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct mbool(raw::c_int);

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct mcomplex {
    pub ri: [mreal; 2usize],
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct st_MTensor {
    _unused: [u8; 0],
}
pub type MTensor = *mut st_MTensor;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct st_MNumericArray {
    _unused: [u8; 0],
}
pub type MRawArray = *mut st_MNumericArray;
pub type MNumericArray = *mut st_MNumericArray;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct MSparseArray_struct {
    _unused: [u8; 0],
}
pub type MSparseArray = *mut MSparseArray_struct;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct IMAGEOBJ_ENTRY {
    _unused: [u8; 0],
}
pub type MImage = *mut IMAGEOBJ_ENTRY;

#[repr(C)]
#[derive(Copy, Clone)]
pub union MArgument {
    pub boolean: *mut mbool,
    pub integer: *mut mint,
    pub real: *mut mreal,
    pub cmplex: *mut mcomplex,
    pub tensor: *mut MTensor,
    pub sparse: *mut MSparseArray,
    pub numeric: *mut MNumericArray,
    pub image: *mut MImage,
    pub utf8string: *mut *mut raw::c_char,
    _union_align: u64,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct MLink {
    _unused: [u8; 0],
}
pub type MLINK = *mut MLink;
pub type WSLINK = *mut MLink;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ml_environment {
    _unused: [u8; 0],
}
pub type MLENV = *mut ml_environment;
pub type MLEnvironment = MLENV;
pub type WSENV = *mut ml_environment;
pub type WSEnvironment = WSENV;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct st_MInputStream {
    _unused: [u8; 0],
}
pub type MInputStream = *mut st_MInputStream;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct st_MOutputStream {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct st_DataStore {
    _unused: [u8; 0],
}
pub type DataStore = *mut st_DataStore;

pub const WolframLibraryVersion: mint = 6;
pub const True: mbool = mbool(1);
pub const False: mbool = mbool(0);

pub const LIBRARY_NO_ERROR: errcode_t = 0;
pub const LIBRARY_TYPE_ERROR: errcode_t = 1;
pub const LIBRARY_RANK_ERROR: errcode_t = 2;
pub const LIBRARY_DIMENSION_ERROR: errcode_t = 3;
pub const LIBRARY_NUMERICAL_ERROR: errcode_t = 4;
pub const LIBRARY_MEMORY_ERROR: errcode_t = 5;
pub const LIBRARY_FUNCTION_ERROR: errcode_t = 6;
pub const LIBRARY_VERSION_ERROR: errcode_t = 7;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct st_WolframLibraryData {
    pub UTF8String_disown: Option<unsafe extern "C" fn(arg1: *mut raw::c_char)>,
    pub MTensor_new: Option<
        unsafe extern "C" fn(
            arg1: mint,
            arg2: mint,
            arg3: *const mint,
            arg4: *mut MTensor,
        ) -> raw::c_int,
    >,
    pub MTensor_free: Option<unsafe extern "C" fn(arg1: MTensor)>,
    pub MTensor_clone:
        Option<unsafe extern "C" fn(arg1: MTensor, arg2: *mut MTensor) -> raw::c_int>,
    pub MTensor_shareCount: Option<unsafe extern "C" fn(arg1: MTensor) -> mint>,
    pub MTensor_disown: Option<unsafe extern "C" fn(arg1: MTensor)>,
    pub MTensor_disownAll: Option<unsafe extern "C" fn(arg1: MTensor)>,
    pub MTensor_setInteger:
        Option<unsafe extern "C" fn(arg1: MTensor, arg2: *mut mint, arg3: mint) -> raw::c_int>,
    pub MTensor_setReal:
        Option<unsafe extern "C" fn(arg1: MTensor, arg2: *mut mint, arg3: mreal) -> raw::c_int>,
    pub MTensor_setComplex:
        Option<unsafe extern "C" fn(arg1: MTensor, arg2: *mut mint, arg3: mcomplex) -> raw::c_int>,
    pub MTensor_setMTensor: Option<
        unsafe extern "C" fn(
            arg1: MTensor,
            arg2: MTensor,
            arg3: *mut mint,
            arg4: mint,
        ) -> raw::c_int,
    >,
    pub MTensor_getInteger:
        Option<unsafe extern "C" fn(arg1: MTensor, arg2: *mut mint, arg3: *mut mint) -> raw::c_int>,
    pub MTensor_getReal: Option<
        unsafe extern "C" fn(arg1: MTensor, arg2: *mut mint, arg3: *mut mreal) -> raw::c_int,
    >,
    pub MTensor_getComplex: Option<
        unsafe extern "C" fn(arg1: MTensor, arg2: *mut mint, arg3: *mut mcomplex) -> raw::c_int,
    >,
    pub MTensor_getMTensor: Option<
        unsafe extern "C" fn(
            arg1: MTensor,
            arg2: *mut mint,
            arg3: mint,
            arg4: *mut MTensor,
        ) -> raw::c_int,
    >,
    pub MTensor_getRank: Option<unsafe extern "C" fn(arg1: MTensor) -> mint>,
    pub MTensor_getDimensions: Option<unsafe extern "C" fn(arg1: MTensor) -> *const mint>,
    pub MTensor_getType: Option<unsafe extern "C" fn(arg1: MTensor) -> mint>,
    pub MTensor_getFlattenedLength: Option<unsafe extern "C" fn(arg1: MTensor) -> mint>,
    pub MTensor_getIntegerData: Option<unsafe extern "C" fn(arg1: MTensor) -> *mut mint>,
    pub MTensor_getRealData: Option<unsafe extern "C" fn(arg1: MTensor) -> *mut mreal>,
    pub MTensor_getComplexData: Option<unsafe extern "C" fn(arg1: MTensor) -> *mut mcomplex>,
    pub Message: Option<unsafe extern "C" fn(arg1: *const raw::c_char)>,
    pub AbortQ: Option<unsafe extern "C" fn() -> mint>,
    pub getWSLINK: Option<unsafe extern "C" fn(arg1: WolframLibraryData) -> WSLINK>,
    pub processWSLINK: Option<unsafe extern "C" fn(arg1: WSLINK) -> raw::c_int>,
    pub evaluateExpression: Option<
        unsafe extern "C" fn(
            arg1: WolframLibraryData,
            arg2: *mut raw::c_char,
            arg3: raw::c_int,
            arg4: mint,
            arg5: *mut raw::c_void,
        ) -> raw::c_int,
    >,
    pub runtimeData: *mut st_WolframRuntimeData,
    pub compileLibraryFunctions: *mut st_WolframCompileLibrary_Functions,
    pub VersionNumber: mint,
    pub registerInputStreamMethod: Option<
        unsafe extern "C" fn(
            name: *const raw::c_char,
            ctor: Option<
                unsafe extern "C" fn(
                    arg1: MInputStream,
                    msgHead: *const raw::c_char,
                    optionsIn: *mut raw::c_void,
                ),
            >,
            handlerTest: Option<
                unsafe extern "C" fn(arg1: *mut raw::c_void, arg2: *mut raw::c_char) -> mbool,
            >,
            methodData: *mut raw::c_void,
            destroyMethod: Option<unsafe extern "C" fn(methodData: *mut raw::c_void)>,
        ) -> mbool,
    >,
    pub unregisterInputStreamMethod:
        Option<unsafe extern "C" fn(name: *const raw::c_char) -> mbool>,
    pub registerOutputStreamMethod: Option<
        unsafe extern "C" fn(
            name: *const raw::c_char,
            ctor: Option<
                unsafe extern "C" fn(
                    arg1: MOutputStream,
                    msgHead: *const raw::c_char,
                    optionsIn: *mut raw::c_void,
                    appendMode: mbool,
                ),
            >,
            handlerTest: Option<
                unsafe extern "C" fn(arg1: *mut raw::c_void, arg2: *mut raw::c_char) -> mbool,
            >,
            methodData: *mut raw::c_void,
            destroyMethod: Option<unsafe extern "C" fn(methodData: *mut raw::c_void)>,
        ) -> mbool,
    >,
    pub unregisterOutputStreamMethod:
        Option<unsafe extern "C" fn(name: *const raw::c_char) -> mbool>,
    pub ioLibraryFunctions: *mut st_WolframIOLibrary_Functions,
    pub getWSLINKEnvironment: Option<unsafe extern "C" fn(arg1: WolframLibraryData) -> WSENV>,
    pub sparseLibraryFunctions: *mut st_WolframSparseLibrary_Functions,
    pub imageLibraryFunctions: *mut st_WolframImageLibrary_Functions,
    pub registerLibraryExpressionManager: Option<
        unsafe extern "C" fn(
            mname: *const raw::c_char,
            mfun: Option<unsafe extern "C" fn(arg1: WolframLibraryData, arg2: mbool, arg3: mint)>,
        ) -> raw::c_int,
    >,
    pub unregisterLibraryExpressionManager:
        Option<unsafe extern "C" fn(mname: *const raw::c_char) -> raw::c_int>,
    pub releaseManagedLibraryExpression:
        Option<unsafe extern "C" fn(mname: *const raw::c_char, id: mint) -> raw::c_int>,
    pub registerLibraryCallbackManager: Option<
        unsafe extern "C" fn(
            name: *const raw::c_char,
            mfun: Option<
                unsafe extern "C" fn(arg1: WolframLibraryData, arg2: mint, arg3: MTensor) -> mbool,
            >,
        ) -> raw::c_int,
    >,
    pub unregisterLibraryCallbackManager:
        Option<unsafe extern "C" fn(name: *const raw::c_char) -> raw::c_int>,
    pub callLibraryCallbackFunction: Option<
        unsafe extern "C" fn(
            id: mint,
            ArgC: mint,
            Args: *mut MArgument,
            Res: MArgument,
        ) -> raw::c_int,
    >,
    pub releaseLibraryCallbackFunction: Option<unsafe extern "C" fn(id: mint) -> raw::c_int>,
    pub validatePath:
        Option<unsafe extern "C" fn(path: *mut raw::c_char, type_: raw::c_char) -> mbool>,
    pub protectedModeQ: Option<unsafe extern "C" fn() -> mbool>,
    pub rawarrayLibraryFunctions: *mut st_WolframRawArrayLibrary_Functions,
    pub numericarrayLibraryFunctions: *mut st_WolframNumericArrayLibrary_Functions,
    pub setParallelThreadNumber: Option<unsafe extern "C" fn(arg1: raw::c_int) -> raw::c_int>,
    pub restoreParallelThreadNumber: Option<unsafe extern "C" fn(arg1: raw::c_int)>,
    pub getParallelThreadNumber: Option<unsafe extern "C" fn() -> raw::c_int>,
}
pub type WolframLibraryData = *mut st_WolframLibraryData;
