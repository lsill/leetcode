file(GLOB_RECURSE sources ${CMAKE_SOURCE_DIR}/src/deque_pra/*.cpp)

include_directories(${LC_INCLUDE_DIR})

add_library(deque_pra STATIC
        ${sources})