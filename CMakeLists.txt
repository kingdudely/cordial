cmake_minimum_required(VERSION 3.16)
project(cordial VERSION 0.18.0 LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

find_package(X11 REQUIRED)

add_subdirectory(native)

configure_file(${CMAKE_CURRENT_SOURCE_DIR}/native/flag-names.txt
  ${CMAKE_CURRENT_BINARY_DIR}/flag-names.txt COPYONLY)

add_executable(roblox
  native/main.cpp
  native/elf_imports.cpp
  native/symbols.cpp
  native/host_window.cpp
)

target_include_directories(roblox PRIVATE
  ${CMAKE_CURRENT_SOURCE_DIR}/native
  ${CMAKE_CURRENT_SOURCE_DIR}/third_party/libjnivm/include
)

target_link_libraries(roblox PRIVATE
  cordial_linker_shim
  cordial_jni_shim
  jnivm
  logger
  linker
  dl
  pthread
  z
  m
  stdc++
)

target_link_libraries(roblox PRIVATE X11::X11)

# Export every native Android/audio compatibility symbol implemented by the
# Cordial C++ layer so the Bionic linker can register them dynamically.
target_link_options(roblox PRIVATE
  -Wl,--whole-archive $<TARGET_FILE:cordial_liblog> -Wl,--no-whole-archive
)

target_link_options(roblox PRIVATE
  -Wl,--export-dynamic
)

install(TARGETS roblox RUNTIME DESTINATION bin)
