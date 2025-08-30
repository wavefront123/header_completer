cmake_minimum_required(VERSION 3.22)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
project(test_cmake CXX)
add_executable(myapp src/main.cpp src/sub.cpp)

target_include_directories(myapp
    PRIVATE
        "src"
)