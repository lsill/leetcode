file(GLOB_RECURSE sources ${CMAKE_SOURCE_DIR}/src/tree_pra/*.cpp)

include_directories(${LC_INCLUDE_DIR})

add_library(tree_pra STATIC
        ${sources})