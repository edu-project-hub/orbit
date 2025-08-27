set(INC_DIR ${CMAKE_CURRENT_SOURCE_DIR}/include)

set(SOURCES 
  lib/Context.cc
)

add_library(orbit-shell ${SOURCES})
target_include_directories(orbit-shell PUBLIC ${INC_DIR})
target_link_libraries(orbit-shell PRIVATE glfw)