# This file is used only for IDE support of src/bindings.cpp.

cmake_minimum_required (VERSION 2.6)
project (skia-bindings)

include_directories("skia/include/core")
include_directories("skia/include/effects")
include_directories("skia/include/gpu")

add_library(skiabindings src/bindings.cpp)
