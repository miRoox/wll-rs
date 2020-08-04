/*************************************************************************
                        Mathematica source file

        Copyright 1986 through 2000 by Wolfram Research Inc.

This material contains trade secrets and may be registered with the
U.S. Copyright Office as an unpublished work, pursuant to Title 17,
U.S. Code, Section 408.  Unauthorized copying, adaptation, distribution
or display is prohibited.

*************************************************************************/

#ifndef WOLFRAMNUMERICARRAYLIBRARY_H
#define WOLFRAMNUMERICARRAYLIBRARY_H

#include "WolframLibrary.h"

#ifdef __cplusplus
extern "C" {
#endif

enum MNumericArray_Data_Type {
	MNumericArray_Type_Undef = 0,
	MNumericArray_Type_Bit8 = 1,
	MNumericArray_Type_UBit8, 
	MNumericArray_Type_Bit16,
	MNumericArray_Type_UBit16,
	MNumericArray_Type_Bit32,
	MNumericArray_Type_UBit32,
	MNumericArray_Type_Bit64,
	MNumericArray_Type_UBit64,
	MNumericArray_Type_Real32,
	MNumericArray_Type_Real64,
	MNumericArray_Type_Complex_Real32,
	MNumericArray_Type_Complex_Real64
};

typedef enum MNumericArray_Data_Type numericarray_data_t;

enum MNumericArray_Convert_Method {
	MNumericArray_Convert_Check = 1,
	MNumericArray_Convert_Clip_Check,
	MNumericArray_Convert_Coerce,
	MNumericArray_Convert_Clip_Coerce,
	MNumericArray_Convert_Round,
	MNumericArray_Convert_Clip_Round,
	MNumericArray_Convert_Scale,
	MNumericArray_Convert_Clip_Scale
};

typedef enum MNumericArray_Convert_Method numericarray_convert_method_t;

typedef struct st_WolframNumericArrayLibrary_Functions
{
	errcode_t (*MNumericArray_new)(const numericarray_data_t, const mint, const mint*, MNumericArray*);
	void (*MNumericArray_free)(MNumericArray);
	errcode_t (*MNumericArray_clone)(const MNumericArray, MNumericArray*);
	void (*MNumericArray_disown)(MNumericArray);
	void (*MNumericArray_disownAll)(MNumericArray);
	mint (*MNumericArray_shareCount)(const MNumericArray);
	
	numericarray_data_t (*MNumericArray_getType)(const MNumericArray);
	mint (*MNumericArray_getRank)(const MNumericArray);
	mint const* (*MNumericArray_getDimensions)(const MNumericArray);
	mint (*MNumericArray_getFlattenedLength)(const MNumericArray);
	void* (*MNumericArray_getData)(const MNumericArray);
	errcode_t (*MNumericArray_convertType)(MNumericArray*, const MNumericArray, const numericarray_data_t, const numericarray_convert_method_t, const mreal);
}* WolframNumericArrayLibrary_Functions;

#ifdef __cplusplus
}
#endif

#endif

