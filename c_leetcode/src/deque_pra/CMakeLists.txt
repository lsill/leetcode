file(GLOB_RECURSE sources ${CMAKE_SOURCE_DIR}/src/queue_pra/*.cpp)

include_directories(${LC_INCLUDE_DIR})

add_library(binary_pra STATIC
        ${sources})