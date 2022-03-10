
cmake_policy(SET CMP0048 NEW)
cmake_policy(SET CMP0003 NEW)
cmake_minimum_required(VERSION 3.20)
if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD cargo build --verbose)
    set(TARGET_DIR "debug")
else ()
    set(CARGO_CMD cargo build --release --verbose)
    set(TARGET_DIR "release")
endif ()

if(ENABLE_LTO)
    set(RUST_FLAGS "-Clinker-plugin-lto" "-Clinker=clang-13" "-Clink-arg=-fuse-ld=lld-13")
endif()

set(NV_LIB "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/libnv_api.a")

include_directories(${CMAKE_CURRENT_SOURCE_DIR}/nv_api/include)
set(NV_CXX "${CMAKE_CURRENT_BINARY_DIR}/nv_api.cc")
add_library(nv_api STATIC ${NV_CXX})
add_custom_command(
    OUTPUT ${NV_CXX}
    COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} RUSTFLAGS="${RUST_FLAGS}" ${CARGO_CMD}
    COMMAND cp ${CMAKE_CURRENT_BINARY_DIR}/nv_api/cxxbridge/nv_api/src/lib.rs.cc ${NV_CXX}
    COMMAND cp ${CMAKE_CURRENT_BINARY_DIR}/nv_api/cxxbridge/nv_api/src/lib.rs.h ${CMAKE_CURRENT_BINARY_DIR}/nv_api.h
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)


target_link_libraries(nv_api pthread dl ${NV_LIB})

add_test(NAME nv_api_test 
    COMMAND cargo test
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})