set(NAME orbit-window)
set(INC_DIR ${CMAKE_CURRENT_SOURCE_DIR}/include)

set(SOURCES
  lib/Glfw.cc
)

add_library(${NAME} STATIC ${SOURCES})
target_link_libraries(${NAME} PRIVATE glfw)
target_include_directories(${NAME} PUBLIC ${INC_DIR})
add_library(orbit::window ALIAS ${NAME})