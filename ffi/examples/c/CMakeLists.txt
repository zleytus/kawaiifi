cmake_minimum_required(VERSION 3.22)
project(kawaiifi-examples C)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

include(FetchContent)
FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG master
)
FetchContent_MakeAvailable(Corrosion)

corrosion_import_crate(
    MANIFEST_PATH ../../Cargo.toml
    CRATE_TYPES staticlib
)

add_executable(scan scan.c)
target_link_libraries(scan PRIVATE kawaiifi)
target_include_directories(scan PRIVATE ../../include)
