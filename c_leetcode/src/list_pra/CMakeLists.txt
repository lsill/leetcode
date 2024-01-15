file(GLOB_RECURSE list_source ${PROJECT_SOURCE_DIR}/src/list_pra/*.cpp)

include_directories(${LC_INCLUDE_DIR})

add_library(list_pra STATIC
        ${list_source}
        )