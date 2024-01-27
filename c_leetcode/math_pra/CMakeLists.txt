project(math_pra)

add_library(${PROJECT_NAME} SHARED
        MathPra.cpp)

target_include_directories(${PROJECT_NAME}
        PUBLIC
        ${PROJECT_SOURCE_DIR}/)