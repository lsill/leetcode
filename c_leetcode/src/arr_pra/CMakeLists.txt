file(GLOB_RECURSE arr_sources ${PROJECT_SOURCE_DIR}/src/arr_pra/*.cpp)

include_directories(${LC_INCLUDE_DIR})

add_library(arr_pra STATIC
        ${arr_sources})