cmake_minimum_required(VERSION 3.13.0)

project(binding VERSION 0.1.0)

set(TARGET binding)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(
    ${TARGET}
    SHARED
    binding.cc
)

add_subdirectory(hermes)

target_include_directories(
    ${TARGET}
    PRIVATE hermes/API/
    PRIVATE hermes/public/
)

target_link_libraries(
    ${TARGET}
    PRIVATE hermesapi
)

set_target_properties(${TARGET} PROPERTIES LINK_FLAGS "-Wl, -all_load")

install(TARGETS ${TARGET} DESTINATION .)
