cmake_minimum_required(VERSION 3.5)

project (wlocate)

set (CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/../FindMathematica/CMake/Mathematica")

find_package(Mathematica COMPONENTS WolframLibrary)

if (${Mathematica_FOUND})
	message(STATUS "RUNTIME_LIBRARY_DIRS=${Mathematica_RUNTIME_LIBRARY_DIRS}")
endif()
