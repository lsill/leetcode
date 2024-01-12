file(GLOB_RECURSE stack_source ${PROJECT_SOURCE_DIR}/src/stack_pra/*.cpp)

include_directories(${LC_INCLUDE_DIR})

add_library(stack_pra STATIC
        ${stack_source}
        )