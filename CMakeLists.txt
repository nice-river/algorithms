cmake_minimum_required(VERSION 3.16)
project(algorithm)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_BUILD_TYPE Release)

include_directories(.)
include_directories(src)

add_executable(algorithm leetcode/src/lc2977.cpp)
