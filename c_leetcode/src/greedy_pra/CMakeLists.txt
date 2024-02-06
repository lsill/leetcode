file(GLOB_RECURSE sources ${CMAKE_SOURCE_DIR}/src/greedy_pra/*.cpp)

include_directories(${LC_INCLUDE_DIR})

add_library(greedy_pra STATIC
        ${sources})