file(GLOB_RECURSE sources ${PROJECT_SOURCE_DIR}/src/dichotomy_pra/*.cpp)

include_directories(${LC_INCLUDE_DIR})

add_library(dichotomy_pra STATIC
        ${sources})