cmake_minimum_required(VERSION 3.5)

project(stack_pra)
set(CMAKE_CXX_STANDARD 20)

add_executable(${PROJECT_NAME} stack.cpp string_stack_md.cpp)
