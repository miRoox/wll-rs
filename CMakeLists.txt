cmake_minimum_required(VERSION 3.5)

project (wll_rs)

set (CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/FindMathematica/CMake/Mathematica")

find_package(Mathematica COMPONENTS WolframLibrary MUnit)

if (${Mathematica_FOUND})
	message(STATUS "Mathematica Version ${Mathematica_VERSION}")
	message(STATUS "Mathematica Target System IDs ${Mathematica_SYSTEM_IDS}")
	message(STATUS "Mathematica Host System IDs ${Mathematica_HOST_SYSTEM_IDS}")
	message(STATUS "Mathematica Base Directory ${Mathematica_BASE_DIR}")
	message(STATUS "Mathematica User Base Directory ${Mathematica_USERBASE_DIR}")
	message(STATUS "Mathematica runtime library dirs: ${Mathematica_RUNTIME_LIBRARY_DIRS}")
	message(STATUS "Mathematica runtime library dirs debug: ${Mathematica_RUNTIME_LIBRARY_DIRS_DEBUG}")
endif()

add_custom_target(
    wll_rs
    ALL
    COMMAND cargo build --all
    WORKING_DIRECTORY "${CMAKE_SOURCE_DIR}"
)
