#[[
project(str_pra)

add_library(${PROJECT_NAME} STATIC
        strTest.cpp)

target_include_directories(${PROJECT_NAME}
        PUBLIC
        ${PROJECT_SOURCE_DIR}/)]]

cmake_minimum_required(VERSION 3.5)

project(str_lc)
set(CMAKE_CXX_STANDARD 20)

add_executable(${PROJECT_NAME} main.cpp strTest.cpp)
