cmake_minimum_required(VERSION 3.19)
project(algorithm)

set(CMAKE_CXX_STANDARD 14)

include_directories(.)
include_directories(src)

add_executable(algorithm src/leetcode/offer37.cpp)
