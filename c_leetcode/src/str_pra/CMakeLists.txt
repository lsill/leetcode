file(GLOB_RECURSE str_sources ${PROJECT_SOURCE_DIR}/src/str_pra/*.cpp)

include_directories(${LC_INCLUDE_DIR})

add_library(str_pra STATIC
        ${str_sources})