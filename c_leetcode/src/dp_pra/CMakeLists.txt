file(GLOB_RECURSE dp_sources ${PROJECT_SOURCE_DIR}/src/dp_pra/*.cpp)

include_directories(${LC_INCLUDE_DIR})

add_library(dp_pra STATIC
        ${dp_sources}
        )