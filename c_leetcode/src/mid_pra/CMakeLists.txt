file(GLOB_RECURSE sources ${PROJECT_SOURCE_DIR}/src/mid_pra/*.cpp)

include_directories(${LC_INCLUDE_DIR})

add_library(mid_pra STATIC
        ${sources}
        )