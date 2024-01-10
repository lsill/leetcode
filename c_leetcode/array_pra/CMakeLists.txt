project(array_pra)

add_library(${PROJECT_NAME} SHARED
        arrTest.cpp)

target_include_directories(${PROJECT_NAME}
        PUBLIC
            ${PROJECT_SOURCE_DIR}/)