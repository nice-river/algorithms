cmake_minimum_required(VERSION 3.16)
project(algorithm)

set(CMAKE_CXX_STANDARD 14)

include_directories(.)
include_directories(src)

add_executable(algorithm src/leetcode/lc237.cpp)
