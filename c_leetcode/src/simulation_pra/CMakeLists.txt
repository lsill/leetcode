file(GLOB_RECURSE sources ${CMAKE_SOURCE_DIR}/src/simulation_pra/*.cpp)

include_directories(${LC_INCLUDE_DIR})

add_library(simulation_pra STATIC
        ${sources})