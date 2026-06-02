cmake_minimum_required(VERSION 3.22)
project(kawaiifi-examples C)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

include(FetchContent)
FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.6.1
)
FetchContent_MakeAvailable(Corrosion)

corrosion_import_crate(
    MANIFEST_PATH ../../Cargo.toml
    CRATE_TYPES staticlib
)

add_executable(scan scan.c)
target_link_libraries(scan PRIVATE kawaiifi)
if(APPLE)
  target_link_libraries(scan PRIVATE
        "-framework CoreFoundation"
        "-framework CoreWLAN"
        "-framework Foundation"
        "-framework Security"
        objc
    )
endif()
target_include_directories(scan PRIVATE ../../include)
