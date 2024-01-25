cmake_minimum_required(VERSION 3.5)

if (CMAKE_SYSTEM_NAME STREQUAL "Darwin")
    SET(CMAKE_C_COMPILER "/usr/bin/clang")
    SET(CMAKE_CXX_COMPILER "/usr/bin/clang++")
elseif (CMAKE_SYSTEM_NAME STREQUAL "Linux")
    set(CMAKE_C_COMPILER "gcc")
    set(CMAKE_CXX_COMPILER "g++")
endif ()



SET(CMAKE_EXPORT_COMPILE_COMMANDS ON) # 生成compile_commands.json 查看编译单元所执行指令
SET(BUILD_SHARED_LIBS ON)  # 没有指定生成什么库 默认静态
SET(CMAKE_CXX_STANDARD 20)
SET(CMAKE_CXX_STANDARD_REQUIRED ON) # 必须支持20

project(lc
        VERSION 0.1
        )

# TO_CMAKE_PATH 将提供的路径转换成CMake风格路径
file(TO_CMAKE_PATH ${PROJECT_BINARY_DIR}/CMakeLists.txt, PATH_TO_CMAKE_TXT)
if (EXISTS "${PATH_TO_CMAKE_TXT}")
    message(FATAL_ERROR "Run CMake from a build subdirectory! \"mkdir build ; cd build ; cmake .. \" \
    Some junk files were created in this folder (CMakeCache.txt, CMakeFiles); you should delete those.")
endif ()
message(${PATH_TO_CMAKE_TXT})

SET(LC_CLANG_SEARCH_PATH, "usr/local/bin" "/usr/bin")

if (CMAKE_SYSTEM_NAME MATCHES "Darwin")
    message(STATUS "Platform: macOS")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -DOS_MACOS")
elseif (CMAKE_SYSTEM_NAME MATCHES "Linux")
    message(STATUS "Platform: Linux")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -DOS_LINUX")
elseif (CMAKE_SYSTEM_NAME MATCHES "Window")
    message(STATUS "Platform: Linux")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -DOS_WINDOW")
endif()

# -fPIC 生成位置无关的代码
# -Wall 开启所有警告信息
# -Wextra 开启额外警告信息
# -std=c++20 指定用c++20编译
# -pthread 启用POSIX线程的支持
# -g 生成调试信息，方便使用调试器（入gdb和lldb）进行调试
# CMAKE_CXX_FLAGS 用于指定 C++ 编译器标志的变量。这些标志会应用于所有的构建类型（如 Release、Debug）。
SET(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fPIC -Wall -Wextra -std=c++20 -pthread -g")
# CMAKE_EXE_LINKER_FLAGS 这个变量用于设置链接可执行文件时的链接器标志。这些设置会影响整个项目的所有可执行文件的链接过程。
SET(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -fPIC")
# CMAKE_SHARED_LINKER_FLAGS 这个变量用于设置链接共享库（动态库）时的链接器标志。这些设置专门用于影响共享库的链接过程。
SET(CMAKE_SHARED_LINKER_FLAGS "${CMAKE_SHARED_LINKER_FLAGS}" -fPIC)
#SET(CMAKE_STATIC_LINKER_FLAGS )

#SET(GCC_COVERAGE_LINK_FLAGS "-fPIC")
# CMAKE_ARCHIVE_OUTPUT_DIRECTORY 静态库输出目录
SET(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
# CMAKE_LIBRARY_OUTPUT_DIRECTORY 动态库输出目录
SET(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
# CMAKE_RUNTIME_OUTPUT_DIRECTORY 可执行文件输出目录s
SET(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

SET(LC_INCLUDE_DIR ${PROJECT_SOURCE_DIR}/src/include)
include_directories(${LC_INCLUDE_DIR})

add_subdirectory(src/dp_pra)
add_subdirectory(src/stack_pra)
add_subdirectory(src/str_pra)
add_subdirectory(src/list_pra)
add_subdirectory(src/arr_pra)
add_subdirectory(src/binary_pra)
add_subdirectory(src/test)
