cmake_minimum_required(VERSION 3.20.0)
project(lib_nv VERSION 0.1.0)

include(CTest)
enable_testing()
include(rust_interop.cmake)
build_rust(nv ${CMAKE_SOURCE_DIR}, ${CMAKE_BUILD_TYPE})
add_library(${PROJECT_NAME} INTERFACE)
target_include_directories(${PROJECT_NAME} INTERFACE ${CMAKE_SOURCE_DIR}/c_api/include/)

set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
include(CPack)
