cmake_minimum_required(VERSION 3.25)

project($ENV{MAKE_NAME})

set(CMAKE_ASM_NASM_SOURCE_FILE_EXTENSIONS "${CMAKE_ASM_NASM_SOURCE_FILE_EXTENSIONS};s;asm")
set(CMAKE_ASM_NASM_OBJECT_FORMAT $ENV{MAKE_ASM_OBJECT_TYPE})
set(CMAKE_ASM_NASM_FLAGS "$ENV{MAKE_ASM_FLAGS}")
unset(CMAKE_ASM_NASM_STANDARD_LIBRARIES CACHE)
set(CMAKE_ASM_NASM_LINK_EXECUTABLE "$ENV{MAKE_LINKER_EXECUTABLE_PATH} $ENV{MAKE_LINKER_FLAGS} -T $ENV{MAKE_LINKER_SCRIPT} -o <TARGET> <OBJECTS> <LINK_LIBRARIES>")
set(Rust_CARGO_TARGET "$ENV{MAKE_RUST_TARGET}")

enable_language(ASM_NASM C)

include(FetchContent)
FetchContent_Declare(
        Corrosion
        GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
)
FetchContent_MakeAvailable(Corrosion)


if(NOT "$ENV{MAKE_CFLAGS}" STREQUAL "")
    string(REPLACE " " ";" CFLAGS_LIST $ENV{MAKE_CFLAGS})
    foreach(X IN LISTS CFLAGS_LIST)
        message(STATUS "Using C compilation flag ${X}")
        add_compile_options(${X})
    endforeach()
endif()

corrosion_import_crate(
    MANIFEST_PATH Cargo.toml
    NO_STD
)

add_executable($ENV{MAKE_NAME})
set_target_properties($ENV{MAKE_NAME} PROPERTIES LINK_DEPENDS "$ENV{MAKE_LINKER_SCRIPT}")

target_link_libraries($ENV{MAKE_NAME} PUBLIC kfs)

if(NOT "$ENV{MAKE_C_SRCS}" STREQUAL "")
    string(REPLACE " " ";" SRCS_LIST $ENV{MAKE_C_SRCS})
    foreach(X IN LISTS SRCS_LIST)
        message(STATUS "Added ${X} as a C project source file")
        target_sources($ENV{MAKE_NAME} PRIVATE ${X})
    endforeach()
endif()

if(NOT "$ENV{MAKE_ASM_SRCS}" STREQUAL "")
    string(REPLACE " " ";" SRCS_LIST $ENV{MAKE_ASM_SRCS})
    foreach(X IN LISTS SRCS_LIST)
        message(STATUS "Added ${X} as an NASM project source file")
        set_source_files_properties(${X} PROPERTIES LANGUAGE ASM_NASM)
        target_sources($ENV{MAKE_NAME} PRIVATE ${X})
    endforeach()
endif()

if(NOT "$ENV{MAKE_INCLUDES}" STREQUAL "")
    string(REPLACE " " ";" INCLUDES_LIST $ENV{MAKE_INCLUDES})
    foreach(X IN LISTS INCLUDES_LIST)
        message(STATUS "Added ${X} to include PATH")
        target_include_directories($ENV{MAKE_NAME} PUBLIC ${X})
    endforeach()
endif()
