
cmake_policy(SET CMP0048 NEW)
cmake_policy(SET CMP0003 NEW)
cmake_minimum_required(VERSION 3.20)
#Hard coded, terrible idea
find_program(CARGO cargo PATH $ENV{HOME}/.cargo/bin)
#message the current environment path
message(STATUS "Current path: $ENV{PATH}")
set(CMAKE_CXX_STANDARD 17)
if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD ${CARGO} build --verbose)
    set(TARGET_DIR "debug")
else ()
    set(CARGO_CMD ${CARGO} build --release --verbose)
    set(TARGET_DIR "release")
endif ()

if(ENABLE_LTO)
    set(RUST_FLAGS "-Clinker-plugin-lto" "-Clinker=clang-13" "-Clink-arg=-fuse-ld=lld-13")
endif()

set(NV_LIB "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/libnv_api.a")

include_directories(${CMAKE_BINARY_SOURCE_DIR}/nv_api/include)
set(NV_CXX "${CMAKE_CURRENT_BINARY_DIR}/nv_api/lib.rs.cc")
set(ENV{CARGO_TARGET_DIR} ${CMAKE_CURRENT_BINARY_DIR})
set(ENV{RUST_FLAGS} "${RUST_FLAGS}")

add_custom_command(
    OUTPUT ${NV_LIB}
    COMMAND ${CMAKE_COMMAND} -E env CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_CMD}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/nv_api/include/nv_api.h  ${CMAKE_CURRENT_SOURCE_DIR}/nv_api/src/lib.rs

)
add_custom_target(nv_api_target DEPENDS ${NV_CXX})
add_library(nv_api STATIC ${NV_CXX})
make_directory(${CMAKE_CURRENT_BINARY_DIR}/nv_api)
add_custom_command(
    OUTPUT ${NV_CXX}
    COMMAND ${CMAKE_COMMAND} -E copy ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/nv_api/src/*.cc ${CMAKE_CURRENT_BINARY_DIR}/nv_api/
    COMMAND ${CMAKE_COMMAND} -E copy ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/nv_api/src/*.h ${CMAKE_CURRENT_SOURCE_DIR}/nv_api/include/
   DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/nv_api/include/nv_api.h  ${CMAKE_CURRENT_SOURCE_DIR}/nv_api/src/lib.rs ${NV_LIB}
 
)
target_link_libraries(nv_api pthread dl ${NV_LIB} )
target_include_directories(nv_api PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/nv_api/include/ ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/rust/)

add_test(NAME nv_api_test 
    COMMAND cargo test
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
add_executable(test test.cpp)
target_link_libraries(test nv_api)