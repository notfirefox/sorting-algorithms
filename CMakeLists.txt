cmake_minimum_required(VERSION 3.20)

project(sorting-algorithms)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_executable(main)
target_sources(main PRIVATE src/main.c
                            src/mergesort.c
                            src/quicksort.c
                            src/heapsort.c
                            src/swap.c)

