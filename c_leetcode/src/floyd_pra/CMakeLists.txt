file(GLOB_RECURSE sources ${CMAKE_SOURCE_DIR}/src/floyd_pra/*.cpp)

include_directories(${LC_INCLUDE_DIR})

add_library(floyd_pra STATIC
        ${sources})