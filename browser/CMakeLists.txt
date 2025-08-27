add_subdirectory(shell)

set(SOURCES 
  main.cc
)

add_executable(orbit ${SOURCES})
target_link_libraries(orbit PRIVATE orbit-shell)