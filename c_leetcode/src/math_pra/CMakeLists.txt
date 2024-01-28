file(GLOB_RECURSE sources ${PROJECT_SOURCE_DIR}/src/math_pra/*.cpp)

include_directories(${LC_INCLUDE_DIR})

add_library(math_pra STATIC
        ${sources})